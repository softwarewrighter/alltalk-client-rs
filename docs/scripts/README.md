# TTS Backend Docker Scripts

Scripts for building and running TTS backend containers on a system with Docker and NVIDIA GPU (e.g., RTX 5060 Ti with Blackwell architecture).

## Prerequisites

- Arch Linux (or similar) with Docker installed
- NVIDIA GPU with drivers and nvidia-container-toolkit
- ~50GB disk space for Docker images and models

```bash
# Verify GPU access in Docker
docker run --rm --gpus all nvidia/cuda:12.8.0-runtime-ubuntu22.04 nvidia-smi
```

## Quick Start

### AllTalk Stack (Parler + Piper + XTTS)

The AllTalk stack runs 4 containers: a gateway plus 3 TTS engines.

```bash
# Copy scripts to remote system
scp -r docs/scripts user@gpu-server:~/tts-backend/

# On the remote system:
cd ~/tts-backend

# Run everything: create, build, start, test
./at-all.sh
```

This will:
1. Generate all Dockerfiles and server code
2. Build the base image (shared dependencies)
3. Build gateway + parler + piper + xtts images
4. Start all containers via docker-compose
5. Run tests against all 3 engines

### Individual Commands

```bash
./at-1-create-dockerfile.sh  # Generate Dockerfiles
./at-2-build.sh              # Build images
./at-6-run.sh                # Start containers
./at-7-test.sh               # Test all engines
./at-8-stop.sh               # Stop containers
./at-9-logs.sh               # View logs
./at-9-logs.sh at-parler 50  # View specific service logs
```

## Standalone Backends

For engines not in the AllTalk stack:

### Dia (Dialogue TTS)
```bash
./dia-1-create-dockerfile.sh
./dia-2-build.sh
./dia-6-run.sh
./dia-7-test.sh
```
Port: 1110 (configurable via DIA_PORT)

### GPT-SoVITS (Voice Cloning)
```bash
./gpt-1-create-dockerfile.sh
./gpt-2-build.sh
./gpt-6-run.sh
./gpt-7-test.sh
```
Port: 6910 (configurable via GPT_PORT)

### Toucan (Multilingual)
```bash
./tou-1-create-dockerfile.sh
./tou-2-build.sh
./tou-6-run.sh
./tou-7-test.sh
```
Port: 1721 (configurable via TOU_PORT)

## Ports Summary

| Backend   | Default Port | Env Variable |
|-----------|--------------|--------------|
| AllTalk   | 5157         | AT_PORT      |
| Dia       | 1110         | DIA_PORT     |
| GPT-SoVITS| 6910         | GPT_PORT     |
| Toucan    | 1721         | TOU_PORT     |

## Models Directory

Models are persisted on the host to avoid re-downloading:

```bash
export MODELS_DIR=/path/to/shared/models
./at-all.sh
```

Default: `./models` relative to the work directory.

Structure:
```
models/
  huggingface/    # HuggingFace model cache
  piper/          # Piper .onnx models
  tts/            # Coqui TTS/XTTS models
  parler/         # Parler model cache
```

## API Endpoints

All backends expose a compatible API:

```bash
# Health check
curl http://localhost:5157/health

# Check if all backends ready
curl http://localhost:5157/api/ready

# List available engines
curl http://localhost:5157/api/engines

# List voices
curl http://localhost:5157/api/voices

# Generate speech
curl -X POST http://localhost:5157/api/tts-generate \
  -d "text_input=Hello world" \
  -d "engine=parler" \
  -o output.wav
```

## Troubleshooting

### GPU Not Detected
```bash
# Reconfigure Docker runtime
sudo nvidia-ctk runtime configure --runtime=docker
sudo systemctl restart docker
```

### Container Won't Start
```bash
./at-9-logs.sh at-parler 100  # Check specific container
docker compose -f work/alltalk/docker-compose.yml logs
```

### Model Download Slow
First run downloads large models (1-4GB each). Be patient or pre-download:
```bash
# Pre-warm HuggingFace cache
docker run --rm -v ${MODELS_DIR:-./models}:/models \
  -e HF_HOME=/models/huggingface \
  python:3.11 pip download parler-tts transformers
```

### Out of Memory
Parler and XTTS require significant GPU memory (~4-8GB each). If running multiple GPU models, ensure sufficient VRAM or run one at a time.

## File Structure

```
docs/scripts/
  at-*.sh         # AllTalk stack scripts
  dia-*.sh        # Dia standalone
  gpt-*.sh        # GPT-SoVITS standalone
  tou-*.sh        # Toucan standalone
  base-*.sh       # Shared base image
  build-all.sh    # Build all components
  work/           # Generated Dockerfiles (gitignored)
    alltalk/
    dia/
    gptsovits/
    toucan/
    base/
```

## See Also

- `docs/backend-setup.org` - Detailed manual setup guide
- `docs/architecture.md` - System architecture overview
- `docs/prd.md` - Product requirements
