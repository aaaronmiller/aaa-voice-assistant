# AAA Voice Assistant

A robust, hands-free, cross-platform voice assistant designed for Windows, Linux, and macOS. It features wake word detection, optimized Speech-to-Text (using Whisper.cpp with GPU support), flexible Text-to-Speech options, and integration with powerful LLM backends including OpenClaw autonomous agents.

## 🚀 Features

*   **Wake Word Detection**: Low-latency detection using `openwakeword`.
*   **Speech-to-Text (STT)**:
    *   **Whisper.cpp**: Local, privacy-focused, and optimized for:
        *   NVIDIA GPUs (CUDA)
        *   AMD GPUs (ROCm)
        *   Intel Arc/iGPUs (OpenVINO)
        *   Apple Silicon (Metal)
    *   **AssemblyAI**: Cloud-based fallback.
*   **Text-to-Speech (TTS)**:
    *   **System TTS**: Offline, low-latency using `pyttsx3`.
    *   **Inworld AI**: High-quality emotional voices (API key required).
    *   **OpenAI TTS**: High-quality neural voices.
*   **LLM Integration**:
    *   **OpenClaw**: Connect to a local autonomous agent instance.
    *   **API**: OpenAI, Anthropic, etc.
    *   **CLI**: Wrap command-line tools like `claude` or `gh` securely.
*   **User Interface**:
    *   **Overlay**: Visual feedback for listening, processing, and error states.
    *   **System Tray**: Quick access to toggle wake word or quit.
*   **Cross-Platform**:
    *   **Windows**: System Tray, Startup Registry.
    *   **Linux**: `.desktop` autostart, `xclip` support.
    *   **macOS**: LaunchAgent autostart.
*   **Extensibility**: Plugin system for adding custom providers.
*   **Privacy Mode**: Disable clipboard interactions for sensitive workflows.

## 🛠️ Prerequisites

*   **Python 3.8+**
*   **Git**
*   **CMake** (for building Whisper.cpp)
*   **C++ Compiler** (Visual Studio / GCC / Clang)

### Linux Specifics
```bash
sudo apt-get install python3-dev portaudio19-dev xclip cmake build-essential
```

### macOS Specifics
```bash
brew install portaudio cmake
```

## ⚡ Quickstart

1.  **Clone the Repository**:
    ```bash
    git clone https://github.com/your-repo/aaa-voice-assistant.git
    cd aaa-voice-assistant
    ```

2.  **Run the Quickstart Script**:
    ```bash
    python quickstart.py
    ```
    This interactive script will:
    *   Check system dependencies.
    *   Detect your OS and GPU.
    *   Build `whisper.cpp` with the best optimization flags.
    *   Install Python requirements.
    *   Start the assistant.

## ⚙️ Configuration

Manage all settings via the CLI tool.

### API Keys
```bash
python cli.py config api_keys.openai "sk-..."
python cli.py config api_keys.anthropic "sk-ant-..."
```

### Backend Selection
```bash
# Use OpenAI API
python cli.py config llm_backend api
python cli.py config api_provider openai

# Use OpenClaw (Agent)
python cli.py config llm_backend openclaw
python cli.py config openclaw_url "http://localhost:18789/v1/chat/completions"

# Use Local CLI Tool
python cli.py config llm_backend cli
python cli.py config cli_command "claude -p"
```

### Voice Settings
```bash
# List available system voices
python cli.py voice --list

# Set a specific voice ID
python cli.py voice --set "HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Speech\Voices\Tokens\TTS_MS_EN-US_DAVID_11.0"
```

### Startup
```bash
# Enable start on boot
python cli.py startup --enable

# Disable start on boot
python cli.py startup --disable
```

## 🖥️ Usage

*   **Wake Word**: Say "Hey Jarvis" (default) to activate. The system will beep and show a "Listening..." overlay.
*   **Push-to-Talk**: Hold `Ctrl+Space` (default) to record. Release to transcribe and type into the active window.
*   **Manual Toggle**: Press `Ctrl+Alt+W` to toggle Voice Assistant mode manually.
*   **Visual Overlay**: An unobtrusive window at the bottom of the screen indicates the current status (Idle, Listening, Transcribing, Speaking).

## 🔧 Troubleshooting

*   **Build Failures**: Ensure CMake is in your PATH. On Windows, ensure you are running in a Developer Command Prompt if using MSVC.
*   **Audio Issues**:
    *   **Linux**: If you get ALSA errors, ensure `portaudio19-dev` is installed.
    *   **Permissions**: Grant Microphone access in OS settings (especially macOS).
*   **OpenClaw**: Ensure the Docker container is running and the port `18789` is exposed.

## 🔒 Security

*   **API Keys**: Keys are stored in `config.json` locally. Do not share this file.
*   **Privacy Mode**: Enable via `config.json` ("privacy_mode": true) to prevent the assistant from reading/writing to your clipboard.

## 🤝 Contributing

Contributions are welcome! Please submit PRs for new backends, voice providers, or OS-specific improvements.
