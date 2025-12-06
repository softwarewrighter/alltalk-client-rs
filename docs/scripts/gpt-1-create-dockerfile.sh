#!/bin/bash
# gpt-1-create-dockerfile.sh - Create GPT-SoVITS docker-compose using official image
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/gptsovits"

echo "=== Creating GPT-SoVITS Configuration ==="

mkdir -p "${WORK_DIR}"
mkdir -p "${WORK_DIR}/models"
mkdir -p "${WORK_DIR}/voices"
mkdir -p "${WORK_DIR}/outputs"

# Use official GPT-SoVITS Docker image with CUDA 12.8 support
cat > "${WORK_DIR}/docker-compose.yml" << 'EOF'
services:
  gptsovits:
    image: xxxxrt666/gpt-sovits:latest-cu128
    container_name: gptsovits
    ports:
      - "${GPT_PORT:-6910}:9880"
    volumes:
      - ./models/pretrained_models:/workspace/GPT-SoVITS/GPT_SoVITS/pretrained_models
      - ./voices:/workspace/GPT-SoVITS/voices
    environment:
      - is_half=true
      - HF_HOME=/workspace/models/huggingface
    tty: true
    stdin_open: true
    shm_size: "16g"
    deploy:
      resources:
        reservations:
          devices:
            - driver: nvidia
              count: 1
              capabilities: [gpu]
    restart: unless-stopped
    # Start the API server on container start
    command: >
      bash -c "cd /workspace/GPT-SoVITS &&
      source /root/miniconda3/bin/activate &&
      python api_v2.py -a 0.0.0.0 -p 9880 -c GPT_SoVITS/configs/tts_infer.yaml"
    healthcheck:
      test: ["CMD", "curl", "-sf", "http://localhost:9880/", "||", "exit", "1"]
      interval: 30s
      timeout: 10s
      start_period: 300s
      retries: 5
EOF

echo "Created ${WORK_DIR}/docker-compose.yml"
echo ""
echo "Using official image: xxxxrt666/gpt-sovits:latest-cu128"
echo "API will be available at: http://localhost:\${GPT_PORT:-6910}/tts"
