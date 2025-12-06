#!/bin/bash
# base-0-create-dockerfile.sh - Create shared TTS base image with multi-stage build
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/base"

echo "=== Creating TTS Base Image Dockerfile ==="

mkdir -p "${WORK_DIR}"

# Create requirements-common.txt with heavy shared libs
cat > "${WORK_DIR}/requirements-common.txt" << 'EOF'
# Heavy shared dependencies - built once, cached for all derived images
# Note: PyTorch installed separately with nightly/cu128 for Blackwell support
numpy
scipy
soundfile
transformers
accelerate
safetensors
huggingface_hub
sentencepiece
# librosa with Python 3.11 compatible deps
numba>=0.57.0
llvmlite>=0.40.0
librosa>=0.10.0
# API server deps
fastapi
uvicorn
python-multipart
httpx
aiofiles
EOF

cat > "${WORK_DIR}/Dockerfile" << 'EOF'
# =============================================================================
# TTS Base Image - Multi-stage build with UV cache warming
# =============================================================================
# Stage 1: Builder - compile wheels and warm UV cache
# Stage 2: Runtime - lean image with only runtime deps + warmed cache
# =============================================================================

# -----------------------------------------------------------------------------
# Stage 1: Builder
# -----------------------------------------------------------------------------
FROM nvidia/cuda:12.8.0-devel-ubuntu22.04 AS builder

ENV DEBIAN_FRONTEND=noninteractive
ENV UV_CACHE_DIR=/opt/uv-cache

WORKDIR /build

# Install build tools, Python, and dev headers
RUN apt-get update && apt-get install -y \
    build-essential \
    gcc \
    g++ \
    cmake \
    pkg-config \
    python3.11 \
    python3.11-venv \
    python3.11-dev \
    python3-pip \
    git \
    curl \
    wget \
    # Audio libs (dev versions for building)
    ffmpeg \
    libsndfile1-dev \
    libavformat-dev \
    libavcodec-dev \
    libavdevice-dev \
    libavutil-dev \
    libswscale-dev \
    libswresample-dev \
    # TTS deps
    espeak-ng \
    && rm -rf /var/lib/apt/lists/* \
    && ln -sf /usr/bin/python3.11 /usr/bin/python \
    && ln -sf /usr/bin/python3.11 /usr/bin/python3

# Install uv
RUN curl -LsSf https://astral.sh/uv/install.sh | sh
ENV PATH="/root/.local/bin:$PATH"

# Create cache directory
RUN mkdir -p /opt/uv-cache

# Copy common requirements
COPY requirements-common.txt /build/

# Warm the UV cache by installing into a temp venv, then discard the venv
# This builds/downloads all wheels into /opt/uv-cache
# Using PyTorch nightly with CUDA 12.8 for Blackwell (sm_120) support
RUN uv venv /tmp/warmup-venv \
    && . /tmp/warmup-venv/bin/activate \
    && uv pip install --pre --index-url https://download.pytorch.org/whl/nightly/cu128 \
       torch torchvision torchaudio \
    && uv pip install -r /build/requirements-common.txt \
    && deactivate \
    && rm -rf /tmp/warmup-venv

# -----------------------------------------------------------------------------
# Stage 2: Runtime
# -----------------------------------------------------------------------------
FROM nvidia/cuda:12.8.0-cudnn-runtime-ubuntu22.04 AS runtime

ENV DEBIAN_FRONTEND=noninteractive
ENV PYTHONUNBUFFERED=1
ENV UV_CACHE_DIR=/opt/uv-cache
ENV MODELS_ROOT=/models
ENV HUGGINGFACE_HUB_CACHE=/models/huggingface
ENV HF_HOME=/models/huggingface
ENV XDG_CACHE_HOME=/models/xdg-cache

WORKDIR /app

# Install only runtime dependencies (no compilers, no dev headers)
RUN apt-get update && apt-get install -y \
    python3.11 \
    python3.11-venv \
    python3-pip \
    git \
    curl \
    wget \
    ca-certificates \
    # Audio runtime libs
    ffmpeg \
    libsndfile1 \
    # TTS runtime deps
    espeak-ng \
    && rm -rf /var/lib/apt/lists/* \
    && ln -sf /usr/bin/python3.11 /usr/bin/python \
    && ln -sf /usr/bin/python3.11 /usr/bin/python3

# Copy uv binary from builder
COPY --from=builder /root/.local/bin/uv /usr/local/bin/uv

# Copy warmed UV cache from builder
COPY --from=builder /opt/uv-cache /opt/uv-cache

# Create directories for models (will be mounted from host)
RUN mkdir -p /models/huggingface /models/xdg-cache /opt/venvs

# Default command (overridden by derived images)
CMD ["python", "--version"]
EOF

echo "Created ${WORK_DIR}/Dockerfile"
echo "Created ${WORK_DIR}/requirements-common.txt"
