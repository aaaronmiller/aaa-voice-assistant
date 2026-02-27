# Windows Voice Assistant

A hands-free voice assistant for Windows, Linux, and Mac.

## Features

- **Wake Word Detection**: Uses `openwakeword`.
- **Speech-to-Text**: `whisper.cpp` (GPU optimized for NVIDIA, AMD, Intel OpenVINO, Apple Metal) and AssemblyAI.
- **Text-to-Speech**: System TTS, Inworld AI, OpenAI TTS.
- **LLM Backends**: API (OpenAI, Anthropic), CLI (Claude Code), OpenClaw (autonomous agent).
- **Cross-Platform**: Windows, Linux, macOS.

## Installation

1.  **Clone the repository**:
    ```bash
    git clone <repo_url>
    cd <repo_name>
    ```

2.  **Run Setup Wizard**:
    ```bash
    python setup_assistant.py
    ```
    This script will:
    - Install Python dependencies.
    - Detect your GPU and build `whisper.cpp` with appropriate optimization.
    - Offer to setup OpenClaw via Docker.

## Configuration

Use the CLI to manage configuration:

```bash
# Set OpenAI API Key
python cli.py config api_keys.openai "sk-..."

# Set LLM Backend to OpenClaw
python cli.py config llm_backend openclaw

# List TTS Voices
python cli.py voice --list
```

## Usage

Run the assistant:

```bash
python cli.py run
```
Or directly:
```bash
python src/main.py
```

- **Hotkeys**:
  - `Ctrl+Space`: Push-to-Talk (types text).
  - `Ctrl+Alt+W`: Toggle listening (Voice Assistant mode).

## OpenClaw Integration

The setup script can help you install OpenClaw using Docker. Once running, configure the `openclaw_url` in `config.json` (default `http://localhost:18789/v1/chat/completions`).

## Troubleshooting

- **Build Errors**: Ensure you have CMake and the relevant GPU toolkit (CUDA, ROCm, OpenVINO) installed.
- **Permission Errors**: On Linux, global hotkeys might require running as root or specific udev rules.
- **Missing Dependencies**: On Linux, you may need `sudo apt-get install portaudio19-dev` for `pyaudio`.
