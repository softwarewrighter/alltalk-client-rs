#!/bin/bash
# gpt-0-setup-dirs.sh - Setup GPT-SoVITS directories and download pretrained models
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WORK_DIR="${SCRIPT_DIR}/work/gptsovits"
MODELS_DIR="${WORK_DIR}/models/pretrained_models"

echo "=== Setting up GPT-SoVITS directories ==="

mkdir -p "${WORK_DIR}"
mkdir -p "${WORK_DIR}/models"
mkdir -p "${WORK_DIR}/voices"
mkdir -p "${WORK_DIR}/outputs"
mkdir -p "${MODELS_DIR}/gsv-v2final-pretrained"

echo ""
echo "GPT-SoVITS directory structure:"
echo "  ${WORK_DIR}/"
echo "  ├── models/    # GPT-SoVITS pretrained models"
echo "  ├── voices/    # Reference audio (.wav)"
echo "  └── outputs/   # Generated audio files"
echo ""

# Check if models already downloaded
if [[ -f "${MODELS_DIR}/gsv-v2final-pretrained/s2G2333k.pth" ]]; then
    echo "Pretrained models already present."
    exit 0
fi

echo "=== Downloading GPT-SoVITS pretrained models from HuggingFace ==="
echo "Source: https://huggingface.co/lj1995/GPT-SoVITS"
echo ""

# Download v2 pretrained models
echo "Downloading gsv-v2final-pretrained models..."
cd "${MODELS_DIR}/gsv-v2final-pretrained"

# s1bert checkpoint (~155MB)
if [[ ! -f "s1bert25hz-5kh-longer-epoch=12-step=369668.ckpt" ]]; then
    echo "  Downloading s1bert25hz-5kh-longer-epoch=12-step=369668.ckpt..."
    curl -L -o "s1bert25hz-5kh-longer-epoch=12-step=369668.ckpt" \
        "https://huggingface.co/lj1995/GPT-SoVITS/resolve/main/gsv-v2final-pretrained/s1bert25hz-5kh-longer-epoch%3D12-step%3D369668.ckpt"
fi

# s2G model (~106MB)
if [[ ! -f "s2G2333k.pth" ]]; then
    echo "  Downloading s2G2333k.pth..."
    curl -L -o "s2G2333k.pth" \
        "https://huggingface.co/lj1995/GPT-SoVITS/resolve/main/gsv-v2final-pretrained/s2G2333k.pth"
fi

# s2D model (~93MB)
if [[ ! -f "s2D2333k.pth" ]]; then
    echo "  Downloading s2D2333k.pth..."
    curl -L -o "s2D2333k.pth" \
        "https://huggingface.co/lj1995/GPT-SoVITS/resolve/main/gsv-v2final-pretrained/s2D2333k.pth"
fi

# Download chinese-roberta-wwm-ext-large
echo ""
echo "Downloading chinese-roberta-wwm-ext-large..."
mkdir -p "${MODELS_DIR}/chinese-roberta-wwm-ext-large"
cd "${MODELS_DIR}/chinese-roberta-wwm-ext-large"

for file in config.json pytorch_model.bin tokenizer.json; do
    if [[ ! -f "$file" ]]; then
        echo "  Downloading $file..."
        curl -L -o "$file" \
            "https://huggingface.co/lj1995/GPT-SoVITS/resolve/main/chinese-roberta-wwm-ext-large/$file" || true
    fi
done

# Download chinese-hubert-base
echo ""
echo "Downloading chinese-hubert-base..."
mkdir -p "${MODELS_DIR}/chinese-hubert-base"
cd "${MODELS_DIR}/chinese-hubert-base"

for file in config.json preprocessor_config.json pytorch_model.bin; do
    if [[ ! -f "$file" ]]; then
        echo "  Downloading $file..."
        curl -L -o "$file" \
            "https://huggingface.co/lj1995/GPT-SoVITS/resolve/main/chinese-hubert-base/$file"
    fi
done

echo ""
echo "=== GPT-SoVITS models downloaded ==="
ls -la "${MODELS_DIR}/"
