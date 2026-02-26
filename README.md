# Windows Voice Assistant

A hands-free voice assistant for Windows, featuring wake word detection, STT (Whisper.cpp optimized for Intel Arc), TTS (Inworld/System), and OpenClaw integration.

## Features

- **Wake Word Detection**: Uses `openwakeword`.
- **Speech-to-Text**: Supports `whisper.cpp` (OpenVINO optimized) and AssemblyAI.
- **Text-to-Speech**: Supports Inworld AI (placeholder) and System TTS (`pyttsx3`).
- **Hands-Free**: Listen loop and PTT support.
- **System Tray**: Control wake word toggle and quit.
- **OpenClaw Integration**: Can route commands to a local OpenClaw instance.

## Prerequisites

- Python 3.8+
- [Intel OpenVINO Toolkit](https://docs.openvino.ai/latest/openvino_docs_install_guides_installing_openvino_windows.html) (for Arc GPU optimization)
- CMake (for building whisper.cpp)
- Git

## Installation

1.  **Clone the repository**:
    ```bash
    git clone <repo_url>
    cd <repo_name>
    ```

2.  **Install Python Dependencies**:
    ```bash
    pip install -r requirements.txt
    ```

3.  **Build whisper.cpp with OpenVINO**:
    Run the provided PowerShell script as Administrator (recommended):
    ```powershell
    .\setup_whisper_cpp.ps1
    ```
    This will clone `whisper.cpp`, download the base model, and build it with `-DWHISPER_OPENVINO=1`.

## Configuration

Create a `config.json` file in the root directory to override defaults. Example:

```json
{
    "hotkey_ptt": "ctrl+space",
    "hotkey_wake": "ctrl+alt+w",
    "wake_word_enabled": true,
    "stt_provider": "whisper_cpp",
    "whisper_cpp_path": "whisper.cpp/build/Release/main.exe",
    "whisper_cpp_model_path": "whisper.cpp/models/ggml-base.bin",
    "assemblyai_api_key": "YOUR_KEY_HERE",
    "tts_provider": "system",
    "openclaw_url": "http://localhost:18789/v1/chat/completions"
}
```

## Usage

Run the assistant:

```bash
python src/main.py
```

- A system tray icon will appear.
- Say "Hey Jarvis" (default model) to activate.
- Or use `Ctrl+Space` (default) for Push-to-Talk.
- Or use `Ctrl+Alt+W` to toggle listening manually.

## OpenClaw Integration

To integrate with OpenClaw, ensure your OpenClaw instance is running and exposes an API endpoint compatible with the configuration (default `http://localhost:18789/v1/chat/completions`). The assistant sends the transcribed text as a user message.

## Troubleshooting

- **Wake Word not working**: Ensure microphone permissions are granted and input device is correct.
- **Whisper.cpp fails**: Check if `main.exe` exists in the configured path. If building failed, ensure OpenVINO environment variables are set (`setupvars.bat`).
- **Hotkeys**: On Linux, running as root might be required for global hotkeys. On Windows, ensure no other app is blocking the keys.
