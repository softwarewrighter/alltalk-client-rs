#!/bin/bash
# tou-1-create-dockerfile.sh - Create Toucan Dockerfile with proper venv
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/toucan"

echo "=== Creating Toucan Dockerfile ==="

mkdir -p "${WORK_DIR}"

cat > "${WORK_DIR}/requirements-toucan.txt" << 'EOF'
numpy
scipy
matplotlib
Pillow
phonemizer
pypinyin
dragonmapper
jieba
tokenizers
speechbrain
Unidecode
inflect
pyloudnorm
langid
pyworld
g2p_en
epitran
mecab-python3
unidic-lite
fugashi
EOF

cat > "${WORK_DIR}/Dockerfile" << 'EOF'
# IMS-Toucan - Apache 2.0 licensed multilingual TTS with voice cloning
# Single venv at /opt/venvs/toucan
FROM tts-base:local

WORKDIR /app

# Install python3.11-dev for building C extensions (pyworld) and libportaudio2 for sounddevice
RUN apt-get update && apt-get install -y python3.11-dev libportaudio2 && rm -rf /var/lib/apt/lists/*

# Clone IMS-Toucan repository
RUN git clone https://github.com/DigitalPhonetics/IMS-Toucan.git /app/IMS-Toucan

COPY requirements-toucan.txt /app/

# Create venv for Toucan and install deps
# Using PyTorch nightly with CUDA 12.8 for Blackwell (sm_120) support
# Install numba/llvmlite first to prevent old incompatible versions
# Reinstall PyTorch nightly at end to override any downgrades from deps
RUN uv venv /opt/venvs/toucan \
    && . /opt/venvs/toucan/bin/activate \
    && uv pip install --pre --index-url https://download.pytorch.org/whl/nightly/cu128 \
       torch torchvision torchaudio \
    && uv pip install "numba>=0.57.0" "llvmlite>=0.40.0" "librosa>=0.10.0" \
    && uv pip install transformers accelerate huggingface_hub \
    && uv pip install -r /app/requirements-toucan.txt \
    && uv pip install dotwiz torch_complex alias_free_torch einops transphone phonepiece pykakasi imageio praat-parselmouth cvxopt \
    && uv pip install pip fastapi uvicorn python-multipart soundfile sounddevice \
    && uv pip install --pre --index-url https://download.pytorch.org/whl/nightly/cu128 \
       --force-reinstall --no-deps torch torchvision torchaudio \
    && deactivate

COPY server.py /app/server.py
COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

ENV PYTHONPATH=/app/IMS-Toucan

VOLUME ["/models", "/app/voices"]

EXPOSE 8006

HEALTHCHECK --interval=30s --timeout=10s --start-period=300s --retries=5 \
    CMD curl -f http://localhost:8006/health || exit 1

ENTRYPOINT ["/app/entrypoint.sh"]
EOF

cat > "${WORK_DIR}/entrypoint.sh" << 'EOF'
#!/bin/bash
set -e
source /opt/venvs/toucan/bin/activate
exec python -m uvicorn server:app --host 0.0.0.0 --port 8006
EOF

cat > "${WORK_DIR}/server.py" << 'EOF'
"""IMS-Toucan API Server - Apache 2.0 licensed multilingual TTS with voice cloning."""

import io
import os
import tempfile
import subprocess
import torch
import torchaudio
import soundfile as sf
from pathlib import Path
from fastapi import FastAPI, Form, UploadFile, File, HTTPException
from fastapi.responses import Response

# Monkey-patch for torchaudio nightly compatibility with speechbrain
if not hasattr(torchaudio, 'list_audio_backends'):
    torchaudio.list_audio_backends = lambda: ['soundfile']
if not hasattr(torchaudio, 'get_audio_backend'):
    torchaudio.get_audio_backend = lambda: 'soundfile'
if not hasattr(torchaudio, 'set_audio_backend'):
    torchaudio.set_audio_backend = lambda x: None

app = FastAPI(title="IMS-Toucan API")

os.environ["HF_HOME"] = "/models/huggingface"
os.environ["HUGGINGFACE_HUB_CACHE"] = "/models/huggingface"

MODELS_DIR = Path("/models/toucan")
VOICES_DIR = Path("/app/voices")
MODELS_DIR.mkdir(parents=True, exist_ok=True)
VOICES_DIR.mkdir(parents=True, exist_ok=True)

_tts = None

def get_tts():
    global _tts
    if _tts is None:
        try:
            from InferenceInterfaces.ToucanTTSInterface import ToucanTTSInterface
            print("Loading Toucan TTS model...")
            _tts = ToucanTTSInterface(device="cuda", tts_model_path=None, language="eng")
            print("Toucan loaded!")
        except Exception as e:
            import traceback
            traceback.print_exc()
            raise HTTPException(500, f"Failed to load Toucan: {e}")
    return _tts

@app.get("/health")
def health():
    return {"status": "healthy", "model": "toucan", "license": "Apache-2.0"}

@app.get("/api/ready")
def ready():
    return {"ready": True}

@app.get("/api/voices")
def list_voices():
    voices = [f.stem for f in VOICES_DIR.glob("*.wav")] if VOICES_DIR.exists() else []
    return {"voices": {"toucan": voices}, "languages": ["multilingual"]}

@app.post("/upload-voice")
async def upload_voice(voice_name: str = Form(...), audio: UploadFile = File(...)):
    safe_name = "".join(c for c in voice_name if c.isalnum() or c in "-_").lower()
    if not safe_name:
        raise HTTPException(400, "Invalid voice name")

    output_path = VOICES_DIR / f"{safe_name}.wav"
    content = await audio.read()

    with tempfile.NamedTemporaryFile(suffix=".tmp", delete=False) as tmp:
        tmp.write(content)
        tmp_path = tmp.name

    try:
        subprocess.run([
            "ffmpeg", "-y", "-i", tmp_path,
            "-af", "highpass=f=80,loudnorm=I=-16:TP=-1.5:LRA=11",
            "-ac", "1", "-ar", "24000",
            str(output_path)
        ], check=True, capture_output=True)
    finally:
        os.unlink(tmp_path)

    return {"status": "success", "voice": safe_name}

@app.post("/api/tts-generate")
async def generate(
    text_input: str = Form(...),
    engine: str = Form("toucan"),
    voice: str = Form(""),
    language: str = Form("eng"),
):
    voice_file = VOICES_DIR / f"{voice}.wav" if voice else None
    if voice_file and not voice_file.exists():
        voice_file = None

    try:
        tts = get_tts()

        # Set language
        tts.set_language(language)

        # Set voice embedding if provided
        if voice_file:
            tts.set_utterance_embedding(path_to_reference_audio=str(voice_file))

        # Generate audio using forward method
        with torch.inference_mode():
            wav, sr = tts(text_input)

        # Convert to numpy if tensor
        if hasattr(wav, 'numpy'):
            wav = wav.numpy()

        buffer = io.BytesIO()
        sf.write(buffer, wav, sr, format="WAV")
        buffer.seek(0)

        return Response(content=buffer.read(), media_type="audio/wav")
    except HTTPException:
        raise
    except Exception as e:
        import traceback
        traceback.print_exc()
        raise HTTPException(500, f"Generation failed: {str(e)}")
EOF

cat > "${WORK_DIR}/docker-compose.yml" << 'EOF'
services:
  toucan:
    build: .
    image: toucan:local
    container_name: toucan
    ports:
      - "${TOU_PORT:-1721}:8006"
    volumes:
      - ${MODELS_DIR:-./models}:/models
      - ./voices:/app/voices
    environment:
      - HUGGINGFACE_HUB_CACHE=/models/huggingface
      - HF_HOME=/models/huggingface
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]
    restart: unless-stopped
EOF

cat > "${WORK_DIR}/.dockerignore" << 'EOF'
models/
voices/
outputs/
*.log
EOF

echo "Created ${WORK_DIR}/Dockerfile"
echo "Created ${WORK_DIR}/requirements-toucan.txt"
echo "Created ${WORK_DIR}/server.py"
echo "Created ${WORK_DIR}/entrypoint.sh"
echo "Created ${WORK_DIR}/docker-compose.yml"
echo "Created ${WORK_DIR}/.dockerignore"
