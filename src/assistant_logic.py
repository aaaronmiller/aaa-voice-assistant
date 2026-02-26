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
    from .tts_service import InworldTTSProvider, SystemTTSProvider
except ImportError:
    from audio_recorder import AudioRecorder
    from wake_word import WakeWordDetector
    from stt_service import WhisperCPPProvider, AssemblyAIProvider
    from tts_service import InworldTTSProvider, SystemTTSProvider

class Assistant:
    def __init__(self, config):
        self.config = config
        self.running = False
        self.listening = False
        self.recording_for_stt = False
        self.wake_word_enabled = config.get("wake_word_enabled", True)
        self.lock = threading.Lock()

        # Audio
        self.audio_recorder = AudioRecorder()

        # Wake Word
        # Try to init wake word detector, handle failure gracefully
        try:
            self.wake_word_detector = WakeWordDetector()
        except Exception as e:
            print(f"Wake Word Detector init failed: {e}")
            self.wake_word_detector = None

        # STT
        if config.get("stt_provider") == "whisper_cpp":
            self.stt_provider = WhisperCPPProvider(
                config.get("whisper_cpp_path", "whisper.cpp/main"),
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
            # Placeholder: Inworld requires complex auth. Fallback to System for now.
            print("Inworld TTS configured but using System fallback due to auth complexity.")
            pass

        # OpenClaw
        self.openclaw_url = config.get("openclaw_url", "http://localhost:18789/v1/chat/completions")

        # State
        self.audio_buffer = []

    def start(self):
        self.running = True
        self.audio_recorder.start_stream()

        # Start wake word loop
        threading.Thread(target=self._wake_word_loop, daemon=True).start()

        # Register hotkeys
        # Note: keyboard library requires root/admin on Linux usually. Windows is fine.
        try:
            hotkey_ptt = self.config.get("hotkey_ptt", "ctrl+space")
            keyboard.add_hotkey(hotkey_ptt, self._handle_ptt_press, suppress=True, trigger_on_release=False)
            keyboard.on_release_key(hotkey_ptt.split('+')[-1], self._handle_ptt_release) # Limitation of on_release_key

            hotkey_wake = self.config.get("hotkey_wake", "ctrl+alt+w")
            keyboard.add_hotkey(hotkey_wake, self._toggle_listening)
            print(f"Hotkeys registered: PTT={hotkey_ptt}, Wake={hotkey_wake}")
        except Exception as e:
            print(f"Error registering hotkeys: {e}")

        print("Assistant started.")

    def stop(self):
        self.running = False
        self.audio_recorder.stop_stream()
        print("Assistant stopped.")

    def _wake_word_loop(self):
        while self.running:
            audio_chunk = self.audio_recorder.get_audio()
            if audio_chunk is None:
                continue

            # Use lock when accessing shared state
            with self.lock:
                is_listening = self.listening or self.recording_for_stt

            if not is_listening:
                # Check for wake word
                if self.wake_word_enabled and self.wake_word_detector:
                    ww = self.wake_word_detector.detect(audio_chunk)
                    if ww:
                        print(f"Wake word detected: {ww}")
                        # Start listening automatically
                        with self.lock:
                            self.listening = True
                            self.audio_buffer = []
                        # Optional: Play sound
            else:
                # Accumulate audio
                with self.lock:
                    self.audio_buffer.append(audio_chunk)

    def _handle_ptt_press(self):
        with self.lock:
            if not self.recording_for_stt:
                print("PTT Pressed - Recording...")
                self.recording_for_stt = True
                self.audio_buffer = []

    def _handle_ptt_release(self, event):
        # We need to check if we were recording
        with self.lock:
            was_recording = self.recording_for_stt
            if was_recording:
                print("PTT Released - Transcribing...")
                self.recording_for_stt = False

        if was_recording:
             self._process_audio_buffer(mode="type")

    def _toggle_listening(self):
        with self.lock:
            self.listening = not self.listening
            is_listening = self.listening

        if is_listening:
            print("Listening started (manual)...")
            with self.lock:
                self.audio_buffer = []
        else:
            print("Listening stopped (manual).")
            self._process_audio_buffer(mode="assistant")

    def _process_audio_buffer(self, mode="assistant"):
        with self.lock:
            if not self.audio_buffer:
                return
            full_audio = np.concatenate(self.audio_buffer)
            self.audio_buffer = []

        # Transcribe
        if self.stt_provider:
            print("Transcribing...")
            text = self.stt_provider.transcribe(full_audio)
            print(f"Transcribed: {text}")

            if text:
                if mode == "type":
                    self._type_text(text)
                elif mode == "assistant":
                    self._handle_assistant_command(text)

    def _type_text(self, text):
        # Use clipboard to paste text
        try:
            old_clipboard = pyperclip.paste()
            pyperclip.copy(text)
            keyboard.send('ctrl+v')
            time.sleep(0.1)
            pyperclip.copy(old_clipboard)
        except Exception as e:
            print(f"Error typing text: {e}")

    def _handle_assistant_command(self, text):
        # Send to backend (OpenClaw)
        response_text = "I heard: " + text

        # Simple OpenClaw integration placeholder
        if self.openclaw_url:
             try:
                # Example payload for OpenAI-compatible API
                payload = {
                    "model": "gpt-3.5-turbo", # or whatever model OpenClaw expects
                    "messages": [{"role": "user", "content": text}]
                }
                # Uncomment to enable actual request when URL is valid
                # response = requests.post(self.openclaw_url, json=payload, timeout=5)
                # if response.status_code == 200:
                #    response_text = response.json()['choices'][0]['message']['content']
                pass
             except Exception:
                 pass

        # Speak response
        self.tts_provider.speak(response_text)
