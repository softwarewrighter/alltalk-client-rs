#!/bin/bash
# dia-1-create-dockerfile.sh - Create Dia Dockerfile with proper venv
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/dia"

echo "=== Creating Dia Dockerfile ==="

mkdir -p "${WORK_DIR}"

cat > "${WORK_DIR}/requirements-dia.txt" << 'EOF'
nari-tts @ git+https://github.com/nari-labs/dia.git
EOF

cat > "${WORK_DIR}/Dockerfile" << 'EOF'
# Dia 1.6B - Apache 2.0 licensed dialogue TTS
# Single venv at /opt/venvs/dia
FROM tts-base:local

WORKDIR /app

COPY requirements-dia.txt /app/

# Create venv for Dia and install deps
# Using PyTorch nightly with CUDA 12.8 for Blackwell (sm_120) support
# Install numba/llvmlite first, then nari-tts, then REINSTALL nightly PyTorch
# (nari-tts pulls in an older torch that doesn't support sm_120)
RUN uv venv /opt/venvs/dia \
    && . /opt/venvs/dia/bin/activate \
    && uv pip install "numba>=0.57.0" "llvmlite>=0.40.0" "librosa>=0.10.0" \
    && uv pip install transformers accelerate huggingface_hub \
    && uv pip install -r /app/requirements-dia.txt \
    && uv pip install fastapi uvicorn python-multipart soundfile numpy \
    && uv pip install --pre --index-url https://download.pytorch.org/whl/nightly/cu128 \
       torch torchvision torchaudio --force-reinstall \
    && deactivate

COPY server.py /app/server.py
COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

VOLUME ["/models", "/app/voices"]

EXPOSE 8005

HEALTHCHECK --interval=30s --timeout=10s --start-period=300s --retries=5 \
    CMD curl -f http://localhost:8005/health || exit 1

ENTRYPOINT ["/app/entrypoint.sh"]
EOF

cat > "${WORK_DIR}/entrypoint.sh" << 'EOF'
#!/bin/bash
set -e
source /opt/venvs/dia/bin/activate
exec python -m uvicorn server:app --host 0.0.0.0 --port 8005
EOF

cat > "${WORK_DIR}/server.py" << 'EOF'
"""Dia 1.6B API Server - Apache 2.0 licensed dialogue TTS."""

import io
import os
import tempfile
import numpy as np
import soundfile as sf
from pathlib import Path
from fastapi import FastAPI, Form, UploadFile, File, HTTPException
from fastapi.responses import Response

app = FastAPI(title="Dia 1.6B API")

os.environ["HF_HOME"] = "/models/huggingface"
os.environ["HUGGINGFACE_HUB_CACHE"] = "/models/huggingface"

MODELS_DIR = Path("/models/dia")
VOICES_DIR = Path("/app/voices")
MODELS_DIR.mkdir(parents=True, exist_ok=True)
VOICES_DIR.mkdir(parents=True, exist_ok=True)

_model = None

def get_model():
    global _model
    if _model is None:
        try:
            from dia import Dia
            print("Loading Dia 1.6B model...")
            _model = Dia.from_pretrained("nari-labs/Dia-1.6B-0626", compute_dtype="float16")
            print("Dia model loaded!")
        except Exception as e:
            raise HTTPException(500, f"Failed to load Dia: {e}")
    return _model

@app.get("/health")
def health():
    return {"status": "healthy", "model": "dia-1.6b", "license": "Apache-2.0"}

@app.get("/api/ready")
def ready():
    return {"ready": True}

@app.get("/api/voices")
def list_voices():
    voices = [f.stem for f in VOICES_DIR.glob("*.wav")] if VOICES_DIR.exists() else []
    return {"voices": {"dia": voices}, "languages": ["en"]}

@app.post("/upload-voice")
async def upload_voice(
    voice_name: str = Form(...),
    audio: UploadFile = File(...),
    transcript: str = Form("Hello, this is my voice sample."),
):
    import subprocess
    safe_name = "".join(c for c in voice_name if c.isalnum() or c in "-_").lower()
    if not safe_name:
        raise HTTPException(400, "Invalid voice name")

    audio_path = VOICES_DIR / f"{safe_name}.wav"
    transcript_path = VOICES_DIR / f"{safe_name}.txt"

    content = await audio.read()

    with tempfile.NamedTemporaryFile(suffix=".tmp", delete=False) as tmp:
        tmp.write(content)
        tmp_path = tmp.name

    try:
        subprocess.run([
            "ffmpeg", "-y", "-i", tmp_path,
            "-af", "highpass=f=80,loudnorm=I=-16:TP=-1.5:LRA=11",
            "-ac", "1", "-ar", "44100",
            str(audio_path)
        ], check=True, capture_output=True)
    finally:
        os.unlink(tmp_path)

    transcript_path.write_text(transcript)
    return {"status": "success", "voice": safe_name}

@app.post("/api/tts-generate")
def generate(
    text_input: str = Form(...),
    engine: str = Form("dia"),
    voice: str = Form(""),
    temperature: float = Form(1.3),
    cfg_scale: float = Form(3.0),
):
    """Generate speech using Dia. Use [S1] and [S2] tags for dialogue."""
    try:
        model = get_model()

        voice_file = VOICES_DIR / f"{voice}.wav" if voice else None
        transcript_file = VOICES_DIR / f"{voice}.txt" if voice else None

        if voice_file and voice_file.exists():
            clone_transcript = transcript_file.read_text().strip() if transcript_file.exists() else "Hello."
            clone_text = f"[S1] {clone_transcript}"
            full_text = clone_text + " " + text_input
            audio_prompt = str(voice_file)
        else:
            full_text = text_input
            audio_prompt = None

        output = model.generate(
            full_text,
            audio_prompt=audio_prompt,
            use_torch_compile=False,
            verbose=False,
            cfg_scale=cfg_scale,
            temperature=temperature,
            top_p=0.95,
        )

        buffer = io.BytesIO()
        sf.write(buffer, output, 44100, format="WAV")
        buffer.seek(0)

        return Response(content=buffer.read(), media_type="audio/wav")
    except Exception as e:
        raise HTTPException(500, f"Generation failed: {str(e)}")
EOF

cat > "${WORK_DIR}/docker-compose.yml" << 'EOF'
services:
  dia:
    build: .
    image: dia:local
    container_name: dia
    ports:
      - "${DIA_PORT:-1110}:8005"
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

echo "Created ${WORK_DIR}/Dockerfile"
echo "Created ${WORK_DIR}/requirements-dia.txt"
echo "Created ${WORK_DIR}/server.py"
echo "Created ${WORK_DIR}/entrypoint.sh"
echo "Created ${WORK_DIR}/docker-compose.yml"
