import threading
import queue
import time
import keyboard
import pyperclip
import requests
import json
import numpy as np
try:
    from .audio_recorder import AudioRecorder
    from .wake_word import WakeWordDetector
    from .stt_service import WhisperCPPProvider, AssemblyAIProvider
    from .tts_service import InworldTTSProvider, SystemTTSProvider, OpenAITTSProvider
    from .llm_service import LLMService
    from .overlay import OverlayWindow
    from .plugin_manager import PluginManager
except ImportError:
    from audio_recorder import AudioRecorder
    from wake_word import WakeWordDetector
    from stt_service import WhisperCPPProvider, AssemblyAIProvider
    from tts_service import InworldTTSProvider, SystemTTSProvider, OpenAITTSProvider
    from llm_service import LLMService
    from overlay import OverlayWindow
    from plugin_manager import PluginManager

class Assistant:
    def __init__(self, config):
        self.config = config
        self.running = False
        self.listening = False
        self.recording_for_stt = False
        self.wake_word_enabled = config.get("wake_word_enabled", True)
        self.lock = threading.Lock()

        # Overlay
        self.overlay = OverlayWindow()
        self.overlay.start()

        # Audio
        self.audio_recorder = AudioRecorder()

        # Plugin Manager
        self.plugin_manager = PluginManager()
        self.plugin_manager.load_plugins()

        # Wake Word
        try:
            self.wake_word_detector = WakeWordDetector()
        except Exception as e:
            print(f"Wake Word Detector init failed: {e}")
            self.wake_word_detector = None

        # STT
        if config.get("stt_provider") == "whisper_cpp":
            self.stt_provider = WhisperCPPProvider(
                config.get("whisper_cpp_path", "whisper.cpp/build/bin/main"),
                config.get("whisper_cpp_model_path", "whisper.cpp/models/ggml-base.bin")
            )
        elif config.get("stt_provider") == "assemblyai":
            self.stt_provider = AssemblyAIProvider(config.get("assemblyai_api_key", ""))
        else:
            self.stt_provider = None
            print("No STT provider configured.")

        # TTS
        self.tts_provider = SystemTTSProvider() # Default
        if config.get("tts_provider") == "inworld":
            self.tts_provider = InworldTTSProvider(config.get("inworld_api_key", ""), config.get("inworld_api_secret", ""))
        elif config.get("tts_provider") == "openai":
            self.tts_provider = OpenAITTSProvider(config.get("api_keys", {}).get("openai"), config.get("voice_id", "alloy"))
        elif config.get("tts_provider") == "system":
            self.tts_provider = SystemTTSProvider(config.get("voice_id"))

        # LLM Service
        self.llm_service = LLMService(config)

        # State
        self.audio_buffer = []

    def start(self):
        self.running = True
        self.audio_recorder.start_stream()

        threading.Thread(target=self._wake_word_loop, daemon=True).start()

        try:
            hotkey_ptt = self.config.get("hotkey_ptt", "ctrl+space")
            keyboard.add_hotkey(hotkey_ptt, self._handle_ptt_press, suppress=True, trigger_on_release=False)
            keyboard.on_release_key(hotkey_ptt.split('+')[-1], self._handle_ptt_release)

            hotkey_wake = self.config.get("hotkey_wake", "ctrl+alt+w")
            keyboard.add_hotkey(hotkey_wake, self._toggle_listening)
            print(f"Hotkeys registered: PTT={hotkey_ptt}, Wake={hotkey_wake}")
        except Exception as e:
            print(f"Error registering hotkeys: {e}")

        self.overlay.update_status("Idle", "#888")
        print("Assistant started.")

    def stop(self):
        self.running = False
        self.audio_recorder.stop_stream()
        self.overlay.stop()
        print("Assistant stopped.")

    def _wake_word_loop(self):
        while self.running:
            audio_chunk = self.audio_recorder.get_audio()
            if audio_chunk is None:
                continue

            with self.lock:
                is_listening = self.listening or self.recording_for_stt

            if not is_listening:
                if self.wake_word_enabled and self.wake_word_detector:
                    ww = self.wake_word_detector.detect(audio_chunk)
                    if ww:
                        print(f"Wake word detected: {ww}")
                        self.overlay.update_status("Wake Word Detected!", "cyan")
                        with self.lock:
                            self.listening = True
                            self.audio_buffer = []
            else:
                with self.lock:
                    self.audio_buffer.append(audio_chunk)

    def _handle_ptt_press(self):
        with self.lock:
            if not self.recording_for_stt:
                print("PTT Pressed - Recording...")
                self.overlay.update_status("Recording...", "red")
                self.recording_for_stt = True
                self.audio_buffer = []

    def _handle_ptt_release(self, event):
        with self.lock:
            was_recording = self.recording_for_stt
            if was_recording:
                print("PTT Released - Transcribing...")
                self.overlay.update_status("Processing...", "orange")
                self.recording_for_stt = False

        if was_recording:
             self._process_audio_buffer(mode="type")

    def _toggle_listening(self):
        with self.lock:
            self.listening = not self.listening
            is_listening = self.listening

        if is_listening:
            print("Listening started (manual)...")
            self.overlay.update_status("Listening...", "green")
            with self.lock:
                self.audio_buffer = []
        else:
            print("Listening stopped (manual).")
            self.overlay.update_status("Processing...", "orange")
            self._process_audio_buffer(mode="assistant")

    def _process_audio_buffer(self, mode="assistant"):
        with self.lock:
            if not self.audio_buffer:
                self.overlay.update_status("Idle", "#888")
                return
            full_audio = np.concatenate(self.audio_buffer)
            self.audio_buffer = []

        # Simple auditory feedback
        print('\a')

        if self.stt_provider:
            print("Transcribing...")
            self.overlay.update_status("Transcribing...", "blue")
            text = self.stt_provider.transcribe(full_audio)
            print(f"Transcribed: {text}")

            if text:
                if mode == "type":
                    self._type_text(text)
                elif mode == "assistant":
                    self._handle_assistant_command(text)
            else:
                self.overlay.update_status("Idle", "#888")
        else:
            self.overlay.update_status("Idle", "#888")

    def _type_text(self, text):
        if self.config.get("privacy_mode", False):
            print("Privacy Mode enabled: Clipboard typing disabled.")
            self.overlay.update_status("Privacy Mode Blocked Typing", "red")
            return

        try:
            old_clipboard = pyperclip.paste()
            pyperclip.copy(text)
            keyboard.send('ctrl+v')
            time.sleep(0.1)
            pyperclip.copy(old_clipboard)
            self.overlay.update_status("Typed!", "green")
            time.sleep(1)
            self.overlay.update_status("Idle", "#888")
        except Exception as e:
            print(f"Error typing text: {e}")
            self.overlay.update_status("Error Typing", "red")

    def _handle_assistant_command(self, text):
        # Use LLM Service
        self.overlay.update_status("Thinking...", "purple")
        response_text = self.llm_service.process(text)

        self.overlay.update_status("Speaking...", "yellow")
        # Speak response
        self.tts_provider.speak(response_text)
        self.overlay.update_status("Idle", "#888")
