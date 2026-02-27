import threading
import sys
import os
import json
import time

# Add src to path if needed (but running as module `python -m src.main` handles it)
sys.path.append(os.path.dirname(os.path.abspath(__file__)))

from assistant_logic import Assistant
from gui import TrayIcon

def load_config():
    # Load from config.json if exists, else defaults
    default_config = {
        "hotkey_ptt": "ctrl+space",
        "hotkey_wake": "ctrl+alt+w",
        "wake_word_enabled": True,
        "stt_provider": "whisper_cpp",
        "whisper_cpp_path": "whisper.cpp/build/Release/main.exe",
        "whisper_cpp_model_path": "whisper.cpp/models/ggml-base.bin",
        "assemblyai_api_key": "",
        "tts_provider": "system",
        "inworld_api_key": "",
        "inworld_jwt_key": "",
        "inworld_jwt_secret": "",
        "openclaw_url": "http://localhost:18789/v1/chat/completions"
    }

    if os.path.exists("config.json"):
        try:
            with open("config.json", "r") as f:
                user_config = json.load(f)
                default_config.update(user_config)
        except Exception as e:
            print(f"Error loading config.json: {e}")

    return default_config

def main():
    print("Initializing Voice Assistant...")
    config = load_config()

    assistant = Assistant(config)

    try:
        assistant.start()
    except Exception as e:
        print(f"Failed to start assistant: {e}")
        return

    tray = TrayIcon(assistant)

    print("Running system tray icon...")
    try:
        tray.run()
    except KeyboardInterrupt:
        assistant.stop()
        print("Exiting.")

if __name__ == "__main__":
    main()
