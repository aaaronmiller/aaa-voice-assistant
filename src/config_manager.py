import os
import json
import platform

class ConfigManager:
    def __init__(self, config_path="config.json"):
        self.config_path = config_path
        self.config = self._load_default_config()
        self._load_user_config()
        self._detect_system_settings()

    def _load_default_config(self):
        return {
            "hotkey_ptt": "ctrl+space",
            "hotkey_wake": "ctrl+alt+w",
            "wake_word_enabled": True,
            "stt_provider": "whisper_cpp",
            "tts_provider": "system",
            "llm_backend": "api", # api, cli, openclaw
            "llm_model": "gpt-3.5-turbo",
            "api_keys": {
                "openai": os.getenv("OPENAI_API_KEY", ""),
                "anthropic": os.getenv("ANTHROPIC_API_KEY", ""),
                "assemblyai": os.getenv("ASSEMBLYAI_API_KEY", ""),
                "inworld": os.getenv("INWORLD_API_KEY", "")
            },
            "whisper_cpp_path": "whisper.cpp/build/bin/main", # Standard path
            "whisper_cpp_model_path": "whisper.cpp/models/ggml-base.bin",
            "openclaw_url": "http://localhost:18789/v1/chat/completions",
            "persona": "default"
        }

    def _load_user_config(self):
        if os.path.exists(self.config_path):
            try:
                with open(self.config_path, "r") as f:
                    user_config = json.load(f)
                    self._merge_config(self.config, user_config)
            except Exception as e:
                print(f"Error loading config: {e}")

    def _merge_config(self, default, user):
        for key, value in user.items():
            if isinstance(value, dict) and key in default:
                self._merge_config(default[key], value)
            else:
                default[key] = value

    def _detect_system_settings(self):
        # Adjust paths based on OS
        system = platform.system()
        if system == "Windows":
             if "whisper.cpp/build/bin/main" in self.config["whisper_cpp_path"]:
                 self.config["whisper_cpp_path"] = "whisper.cpp/build/Release/main.exe"

    def get(self, key, default=None):
        return self.config.get(key, default)

    def set(self, key, value):
        self.config[key] = value
        self.save()

    def save(self):
        with open(self.config_path, "w") as f:
            json.dump(self.config, f, indent=4)
