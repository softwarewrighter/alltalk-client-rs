#!/bin/bash
# at-1-create-dockerfile.sh - Create AllTalk stack (gateway + 3 model containers)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/alltalk"

echo "=== Creating AllTalk Dockerfiles ==="

mkdir -p "${WORK_DIR}"

# =============================================================================
# Gateway - Routes requests to parler/piper/xtts backends
# =============================================================================
cat > "${WORK_DIR}/Dockerfile.gateway" << 'EOF'
# AllTalk Gateway - Routes to parler/piper/xtts backends
FROM tts-base:local

WORKDIR /app

# Gateway only needs httpx for proxying
RUN uv venv /opt/venvs/gateway \
    && . /opt/venvs/gateway/bin/activate \
    && uv pip install fastapi uvicorn python-multipart httpx \
    && deactivate

COPY gateway.py /app/server.py
COPY entrypoint-gateway.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

EXPOSE 7851

HEALTHCHECK --interval=30s --timeout=10s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:7851/health || exit 1

ENTRYPOINT ["/app/entrypoint.sh"]
EOF

cat > "${WORK_DIR}/entrypoint-gateway.sh" << 'EOF'
#!/bin/bash
set -e
source /opt/venvs/gateway/bin/activate
exec python -m uvicorn server:app --host 0.0.0.0 --port 7851
EOF

cat > "${WORK_DIR}/gateway.py" << 'EOF'
"""AllTalk Gateway - Routes requests to parler/piper/xtts backends."""

import os
import httpx
from fastapi import FastAPI, Form, HTTPException
from fastapi.responses import Response

app = FastAPI(title="AllTalk Gateway")

# Backend URLs (internal docker network)
BACKENDS = {
    "parler": os.environ.get("PARLER_URL", "http://at-parler:8001"),
    "piper": os.environ.get("PIPER_URL", "http://at-piper:8002"),
    "xtts": os.environ.get("XTTS_URL", "http://at-xtts:8003"),
}

@app.get("/health")
def health():
    return {"status": "healthy", "engines": list(BACKENDS.keys())}

@app.get("/api/ready")
async def ready():
    # Check if all backends are ready
    async with httpx.AsyncClient(timeout=5.0) as client:
        statuses = {}
        for engine, url in BACKENDS.items():
            try:
                r = await client.get(f"{url}/health")
                statuses[engine] = r.status_code == 200
            except:
                statuses[engine] = False
    return {"ready": all(statuses.values()), "backends": statuses}

@app.get("/api/engines")
def engines():
    return {
        "engines": [
            {"id": "parler", "name": "Parler TTS", "available": True},
            {"id": "piper", "name": "Piper", "available": True},
            {"id": "xtts", "name": "XTTS v2", "available": True},
        ]
    }

@app.get("/api/voices")
async def list_voices():
    """Aggregate voices from all backends."""
    result = {}
    async with httpx.AsyncClient(timeout=10.0) as client:
        for engine, url in BACKENDS.items():
            try:
                r = await client.get(f"{url}/api/voices")
                if r.status_code == 200:
                    data = r.json()
                    result[engine] = data.get("voices", [])
            except:
                result[engine] = []
    return {"voices": result}

@app.post("/api/tts-generate")
async def generate(
    text_input: str = Form(...),
    engine: str = Form("parler"),
    voice: str = Form(""),
):
    """Route generation request to appropriate backend."""
    if engine not in BACKENDS:
        raise HTTPException(400, f"Unknown engine: {engine}. Available: {list(BACKENDS.keys())}")

    backend_url = BACKENDS[engine]

    async with httpx.AsyncClient(timeout=120.0) as client:
        try:
            r = await client.post(
                f"{backend_url}/api/tts-generate",
                data={"text_input": text_input, "voice": voice},
            )
            if r.status_code != 200:
                raise HTTPException(r.status_code, f"Backend error: {r.text}")
            return Response(content=r.content, media_type="audio/wav")
        except httpx.TimeoutException:
            raise HTTPException(504, f"Backend {engine} timeout")
        except httpx.RequestError as e:
            raise HTTPException(502, f"Backend {engine} unavailable: {e}")
EOF

# =============================================================================
# Parler Backend
# =============================================================================
cat > "${WORK_DIR}/Dockerfile.parler" << 'EOF'
# Parler TTS Backend
FROM tts-base:local

WORKDIR /app

# Create venv for Parler
# Using PyTorch nightly with CUDA 12.8 for Blackwell (sm_120) support
RUN uv venv /opt/venvs/parler \
    && . /opt/venvs/parler/bin/activate \
    && uv pip install --pre --index-url https://download.pytorch.org/whl/nightly/cu128 \
       torch torchvision torchaudio \
    && uv pip install "numba>=0.57.0" "llvmlite>=0.40.0" "librosa>=0.10.0" \
    && uv pip install transformers accelerate huggingface_hub \
    && uv pip install parler-tts \
    && uv pip install fastapi uvicorn python-multipart soundfile numpy \
    && deactivate

COPY parler.py /app/server.py
COPY entrypoint-parler.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

VOLUME ["/models"]

EXPOSE 8001

HEALTHCHECK --interval=30s --timeout=10s --start-period=120s --retries=5 \
    CMD curl -f http://localhost:8001/health || exit 1

ENTRYPOINT ["/app/entrypoint.sh"]
EOF

cat > "${WORK_DIR}/entrypoint-parler.sh" << 'EOF'
#!/bin/bash
set -e
source /opt/venvs/parler/bin/activate
exec python -m uvicorn server:app --host 0.0.0.0 --port 8001
EOF

cat > "${WORK_DIR}/parler.py" << 'EOF'
"""Parler TTS Backend."""

import io
import os
from pathlib import Path
from fastapi import FastAPI, Form, HTTPException
from fastapi.responses import Response

app = FastAPI(title="Parler TTS")

os.environ["HF_HOME"] = "/models/huggingface"
MODELS_DIR = Path("/models/parler")
VOICES_DIR = Path("/app/voices")
MODELS_DIR.mkdir(parents=True, exist_ok=True)
VOICES_DIR.mkdir(parents=True, exist_ok=True)

_engine = None

def get_engine():
    global _engine
    if _engine is None:
        from parler_tts import ParlerTTSForConditionalGeneration
        from transformers import AutoTokenizer
        import torch

        device = "cuda" if torch.cuda.is_available() else "cpu"
        model_name = "parler-tts/parler-tts-mini-v1"

        _engine = {
            "model": ParlerTTSForConditionalGeneration.from_pretrained(
                model_name, cache_dir=MODELS_DIR
            ).to(device),
            "tokenizer": AutoTokenizer.from_pretrained(
                model_name, cache_dir=MODELS_DIR
            ),
            "device": device,
        }
        print(f"Parler loaded on {device}")
    return _engine

@app.on_event("startup")
def startup():
    """Preload model at startup so health checks are meaningful."""
    print("Preloading Parler model...")
    get_engine()
    print("Parler ready!")

@app.get("/health")
def health():
    ready = _engine is not None
    return {"status": "healthy" if ready else "loading", "engine": "parler", "ready": ready}

@app.get("/api/voices")
def list_voices():
    voices = [f.stem for f in VOICES_DIR.glob("*.txt")]
    return {"voices": voices}

@app.post("/api/tts-generate")
def generate(text_input: str = Form(...), voice: str = Form("")):
    import soundfile as sf
    import numpy as np

    try:
        p = get_engine()

        if voice:
            desc_file = VOICES_DIR / f"{voice}.txt"
            if desc_file.exists():
                description = desc_file.read_text().strip()
            else:
                description = voice
        else:
            description = "A clear voice speaks at a moderate pace."

        input_ids = p["tokenizer"](description, return_tensors="pt").input_ids.to(p["device"])
        prompt_ids = p["tokenizer"](text_input, return_tensors="pt").input_ids.to(p["device"])
        generation = p["model"].generate(input_ids=input_ids, prompt_input_ids=prompt_ids)
        audio = generation.cpu().numpy().squeeze()
        sr = p["model"].config.sampling_rate

        buffer = io.BytesIO()
        sf.write(buffer, audio.astype(np.float32), sr, format="WAV")
        buffer.seek(0)
        return Response(content=buffer.read(), media_type="audio/wav")
    except Exception as e:
        raise HTTPException(500, f"Generation failed: {str(e)}")
EOF

# =============================================================================
# Piper Backend
# =============================================================================
cat > "${WORK_DIR}/Dockerfile.piper" << 'EOF'
# Piper TTS Backend
FROM tts-base:local

WORKDIR /app

# Create venv for Piper (CPU-only, lightweight)
RUN uv venv /opt/venvs/piper \
    && . /opt/venvs/piper/bin/activate \
    && uv pip install piper-tts \
    && uv pip install fastapi uvicorn python-multipart soundfile numpy \
    && deactivate

COPY piper.py /app/server.py
COPY entrypoint-piper.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

VOLUME ["/models"]

EXPOSE 8002

HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD curl -f http://localhost:8002/health || exit 1

ENTRYPOINT ["/app/entrypoint.sh"]
EOF

cat > "${WORK_DIR}/entrypoint-piper.sh" << 'EOF'
#!/bin/bash
set -e
source /opt/venvs/piper/bin/activate
exec python -m uvicorn server:app --host 0.0.0.0 --port 8002
EOF

cat > "${WORK_DIR}/piper.py" << 'EOF'
"""Piper TTS Backend."""

import io
import wave
import subprocess
from pathlib import Path
from fastapi import FastAPI, Form, HTTPException
from fastapi.responses import Response

app = FastAPI(title="Piper TTS")

MODELS_DIR = Path("/models/piper")
MODELS_DIR.mkdir(parents=True, exist_ok=True)

DEFAULT_MODEL = "en_US-amy-medium"
DEFAULT_MODEL_URL = "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/amy/medium/en_US-amy-medium.onnx"
DEFAULT_CONFIG_URL = "https://huggingface.co/rhasspy/piper-voices/resolve/main/en/en_US/amy/medium/en_US-amy-medium.onnx.json"

_voices = {}

def download_default_model():
    """Download default Piper model if not present."""
    model_path = MODELS_DIR / f"{DEFAULT_MODEL}.onnx"
    config_path = MODELS_DIR / f"{DEFAULT_MODEL}.onnx.json"

    if not model_path.exists():
        print(f"Downloading Piper model: {DEFAULT_MODEL}...")
        subprocess.run(["curl", "-L", "-o", str(model_path), DEFAULT_MODEL_URL], check=True)
        subprocess.run(["curl", "-L", "-o", str(config_path), DEFAULT_CONFIG_URL], check=True)
        print("Piper model downloaded!")

def get_voice(voice_name: str = ""):
    """Get or load a Piper voice."""
    global _voices

    download_default_model()

    if not voice_name:
        voice_name = DEFAULT_MODEL

    if voice_name not in _voices:
        from piper import PiperVoice

        model_path = MODELS_DIR / f"{voice_name}.onnx"
        if not model_path.exists():
            raise HTTPException(400, f"Voice not found: {voice_name}")

        _voices[voice_name] = PiperVoice.load(str(model_path))
        print(f"Piper voice loaded: {voice_name}")

    return _voices[voice_name]

@app.on_event("startup")
def startup():
    """Preload model at startup so health checks are meaningful."""
    print("Preloading Piper model...")
    get_voice()  # Load default voice
    print("Piper ready!")

@app.get("/health")
def health():
    ready = len(_voices) > 0
    return {"status": "healthy" if ready else "loading", "engine": "piper", "ready": ready}

@app.get("/api/voices")
def list_voices():
    download_default_model()
    voices = [f.stem for f in MODELS_DIR.glob("*.onnx")]
    return {"voices": voices}

@app.post("/api/tts-generate")
def generate(text_input: str = Form(...), voice: str = Form("")):
    try:
        piper = get_voice(voice)

        buffer = io.BytesIO()
        with wave.open(buffer, "wb") as wav:
            # Use synthesize_wav for direct WAV output (piper-tts 1.3.0+)
            if hasattr(piper, "synthesize_wav"):
                piper.synthesize_wav(text_input, wav)
            else:
                # Fallback for older versions - synthesize() as generator
                wav.setnchannels(1)
                wav.setsampwidth(2)
                wav.setframerate(piper.config.sample_rate)
                for chunk in piper.synthesize(text_input):
                    wav.writeframes(chunk.audio_int16_bytes)
        buffer.seek(0)
        return Response(content=buffer.read(), media_type="audio/wav")
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(500, f"Generation failed: {str(e)}")
EOF

# =============================================================================
# XTTS Backend
# =============================================================================
cat > "${WORK_DIR}/Dockerfile.xtts" << 'EOF'
# XTTS v2 Backend
FROM tts-base:local

WORKDIR /app

# Create venv for XTTS
# Using stable PyTorch with CUDA 12.8 for Blackwell (sm_120) support
RUN uv venv /opt/venvs/xtts \
    && . /opt/venvs/xtts/bin/activate \
    && uv pip install --index-url https://download.pytorch.org/whl/cu128 \
       torch torchvision torchaudio \
    && uv pip install "numba>=0.57.0" "llvmlite>=0.40.0" "librosa>=0.10.0" \
    && uv pip install "transformers>=4.33.0,<4.46.0" accelerate huggingface_hub \
    && uv pip install TTS inflect phonemizer \
    && uv pip install fastapi uvicorn python-multipart soundfile numpy \
    && deactivate

COPY xtts.py /app/server.py
COPY entrypoint-xtts.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

VOLUME ["/models", "/app/voices"]

EXPOSE 8003

HEALTHCHECK --interval=30s --timeout=10s --start-period=120s --retries=5 \
    CMD curl -f http://localhost:8003/health || exit 1

ENTRYPOINT ["/app/entrypoint.sh"]
EOF

cat > "${WORK_DIR}/entrypoint-xtts.sh" << 'EOF'
#!/bin/bash
set -e
source /opt/venvs/xtts/bin/activate
exec python -m uvicorn server:app --host 0.0.0.0 --port 8003
EOF

cat > "${WORK_DIR}/xtts.py" << 'EOF'
"""XTTS v2 Backend."""

import io
import os
import subprocess
from pathlib import Path
from fastapi import FastAPI, Form, HTTPException
from fastapi.responses import Response

# Set cache directories BEFORE importing TTS
os.environ["HF_HOME"] = "/models/huggingface"
os.environ["XDG_DATA_HOME"] = "/models"
os.environ["COQUI_TOS_AGREED"] = "1"

# PyTorch 2.6+ changed torch.load default to weights_only=True
# TTS checkpoints use custom classes, so we need to allow unsafe loading
import torch
_original_torch_load = torch.load
def _patched_torch_load(*args, **kwargs):
    kwargs.setdefault("weights_only", False)
    return _original_torch_load(*args, **kwargs)
torch.load = _patched_torch_load

# Patch torchaudio.load to use librosa instead of torchcodec
# This bypasses the torchcodec FFmpeg linking issues
import torchaudio
import librosa
import numpy as np
_original_torchaudio_load = torchaudio.load
def _patched_torchaudio_load(filepath, *args, **kwargs):
    try:
        # Try librosa first (more reliable, uses soundfile backend)
        audio, sr = librosa.load(str(filepath), sr=None, mono=False)
        if audio.ndim == 1:
            audio = audio.reshape(1, -1)
        return torch.from_numpy(audio.astype(np.float32)), sr
    except Exception:
        # Fall back to original if librosa fails
        return _original_torchaudio_load(filepath, *args, **kwargs)
torchaudio.load = _patched_torchaudio_load

app = FastAPI(title="XTTS v2")

MODELS_DIR = Path("/models/xtts")
VOICES_DIR = Path("/app/voices")
MODELS_DIR.mkdir(parents=True, exist_ok=True)
VOICES_DIR.mkdir(parents=True, exist_ok=True)

# Default sample voice URL (LJSpeech sample)
DEFAULT_VOICE_URL = "https://huggingface.co/datasets/Xenova/transformers.js-docs/resolve/main/jfk.wav"
DEFAULT_VOICE = "default"

_engine = None

def ensure_default_voice():
    """Download a default voice sample if none exist."""
    default_path = VOICES_DIR / f"{DEFAULT_VOICE}.wav"
    if not default_path.exists() and not list(VOICES_DIR.glob("*.wav")):
        print(f"Downloading default voice sample...")
        subprocess.run(["curl", "-L", "-o", str(default_path), DEFAULT_VOICE_URL], check=True)
        print("Default voice downloaded!")
    return default_path if default_path.exists() else None

def get_engine():
    global _engine
    if _engine is None:
        from TTS.api import TTS
        import torch

        device = "cuda" if torch.cuda.is_available() else "cpu"
        _engine = TTS("tts_models/multilingual/multi-dataset/xtts_v2").to(device)
        print(f"XTTS loaded on {device}")
    return _engine

@app.on_event("startup")
def startup():
    """Preload model at startup so health checks are meaningful."""
    print("Preloading XTTS model...")
    get_engine()
    ensure_default_voice()
    print("XTTS ready!")

@app.get("/health")
def health():
    ready = _engine is not None
    return {"status": "healthy" if ready else "loading", "engine": "xtts", "ready": ready}

@app.get("/api/voices")
def list_voices():
    ensure_default_voice()
    voices = [f.stem for f in VOICES_DIR.glob("*.wav")]
    return {"voices": voices}

@app.post("/api/tts-generate")
def generate(text_input: str = Form(...), voice: str = Form("")):
    import soundfile as sf
    import numpy as np

    try:
        xtts = get_engine()
        ensure_default_voice()

        if voice:
            speaker_wav = VOICES_DIR / f"{voice}.wav"
            if not speaker_wav.exists():
                raise HTTPException(400, f"Voice not found: {voice}")
            speaker_wav = str(speaker_wav)
        else:
            # Use first available voice or default
            voice_files = list(VOICES_DIR.glob("*.wav"))
            if voice_files:
                speaker_wav = str(voice_files[0])
            else:
                raise HTTPException(400, "XTTS requires a voice sample. Upload one first.")

        wav = xtts.tts(text=text_input, speaker_wav=speaker_wav, language="en")

        audio = np.array(wav)
        buffer = io.BytesIO()
        sf.write(buffer, audio.astype(np.float32), 24000, format="WAV")
        buffer.seek(0)
        return Response(content=buffer.read(), media_type="audio/wav")
    except HTTPException:
        raise
    except Exception as e:
        raise HTTPException(500, f"Generation failed: {str(e)}")
EOF

# =============================================================================
# Docker Compose - AllTalk Stack
# =============================================================================
cat > "${WORK_DIR}/docker-compose.yml" << 'EOF'
services:
  at-gateway:
    build:
      context: .
      dockerfile: Dockerfile.gateway
    image: at-gateway:local
    container_name: at-gateway
    ports:
      - "${AT_PORT:-5157}:7851"
    environment:
      - PARLER_URL=http://at-parler:8001
      - PIPER_URL=http://at-piper:8002
      - XTTS_URL=http://at-xtts:8003
    depends_on:
      - at-parler
      - at-piper
      - at-xtts
    restart: unless-stopped

  at-parler:
    build:
      context: .
      dockerfile: Dockerfile.parler
    image: at-parler:local
    container_name: at-parler
    volumes:
      - ${MODELS_DIR:-./models}:/models
      - ./voices/parler:/app/voices
    environment:
      - HF_HOME=/models/huggingface
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]
    restart: unless-stopped

  at-piper:
    build:
      context: .
      dockerfile: Dockerfile.piper
    image: at-piper:local
    container_name: at-piper
    volumes:
      - ${MODELS_DIR:-./models}:/models
    restart: unless-stopped

  at-xtts:
    build:
      context: .
      dockerfile: Dockerfile.xtts
    image: at-xtts:local
    container_name: at-xtts
    volumes:
      - ${MODELS_DIR:-./models}:/models
      - ./voices/xtts:/app/voices
    environment:
      - HF_HOME=/models/huggingface
      - COQUI_TOS_AGREED=1
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]
    restart: unless-stopped
EOF

echo "Created ${WORK_DIR}/Dockerfile.gateway"
echo "Created ${WORK_DIR}/Dockerfile.parler"
echo "Created ${WORK_DIR}/Dockerfile.piper"
echo "Created ${WORK_DIR}/Dockerfile.xtts"
echo "Created ${WORK_DIR}/docker-compose.yml"
echo "Created gateway.py, parler.py, piper.py, xtts.py"
echo "Created entrypoint scripts"
