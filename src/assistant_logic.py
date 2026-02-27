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
    from .stt_service import WhisperCPPProvider, AssemblyAIProvider, OpenAIAPIProvider
    from .tts_service import InworldTTSProvider, SystemTTSProvider, OpenAITTSProvider
    from .llm_service import LLMService
    from .overlay import OverlayWindow
    from .persona_manager import PersonaManager
    from .memory_store import MemoryStore
except ImportError:
    from audio_recorder import AudioRecorder
    from wake_word import WakeWordDetector
    from stt_service import WhisperCPPProvider, AssemblyAIProvider, OpenAIAPIProvider
    from tts_service import InworldTTSProvider, SystemTTSProvider, OpenAITTSProvider
    from llm_service import LLMService
    from overlay import OverlayWindow
    from persona_manager import PersonaManager
    from memory_store import MemoryStore

class Assistant:
    def __init__(self, config):
        self.config = config
        self.running = False
        self.listening = False
        self.recording_for_stt = False
        self.wake_word_enabled = config.get("wake_word_enabled", True)
        self.lock = threading.Lock()

        # VAD / Silence Detection
        self.silence_threshold = config.get("silence_threshold", 500) # RMS value, needs tuning
        self.silence_duration = config.get("silence_duration", 1.5) # Seconds of silence to stop
        self.max_recording_duration = config.get("max_recording_duration", 15.0)
        self.speech_start_time = None
        self.silence_start_time = None

        # Overlay
        self.overlay = OverlayWindow()
        self.overlay.start()

        # Audio
        self.audio_recorder = AudioRecorder()

        # Persona & Memory
        self.persona_manager = PersonaManager()
        self.memory_store = MemoryStore()

        # Wake Word
        try:
            self.wake_word_detector = WakeWordDetector()
        except Exception as e:
            print(f"Wake Word Detector init failed: {e}")
            self.wake_word_detector = None

        # STT
        stt_type = config.get("stt_provider", "whisper_cpp")
        if stt_type == "whisper_cpp":
            self.stt_provider = WhisperCPPProvider(
                config.get("whisper_cpp_path", "whisper.cpp/build/bin/main"),
                config.get("whisper_cpp_model_path", "whisper.cpp/models/ggml-base.bin")
            )
        elif stt_type == "assemblyai":
            self.stt_provider = AssemblyAIProvider(config.get("assemblyai_api_key", ""))
        elif stt_type == "openai":
            self.stt_provider = OpenAIAPIProvider(config.get("api_keys", {}).get("openai"))
        else:
            self.stt_provider = None
            print("No STT provider configured.")

        # TTS
        tts_type = config.get("tts_provider", "system")
        if tts_type == "inworld":
            self.tts_provider = InworldTTSProvider(config.get("inworld_api_key", ""), config.get("inworld_api_secret", ""))
        elif tts_type == "openai":
            self.tts_provider = OpenAITTSProvider(config.get("api_keys", {}).get("openai"), config.get("voice_id", "alloy"))
        else:
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

        # Load Persona
        p_name = self.config.get("persona", "default")
        persona = self.persona_manager.get_persona(p_name)
        print(f"Loaded persona: {persona['name']}")
        self.overlay.update_status(f"✨ Ready ({persona['name']})", "#AAA")

    def stop(self):
        self.running = False
        self.audio_recorder.stop_stream()
        self.overlay.stop()
        print("Assistant stopped.")

    def _calculate_rms(self, audio_chunk):
        # audio_chunk is int16 numpy array
        # prevent overflow by casting to float
        data = audio_chunk.astype(np.float32)
        rms = np.sqrt(np.mean(data**2))
        return rms

    def _wake_word_loop(self):
        while self.running:
            audio_chunk = self.audio_recorder.get_audio()
            if audio_chunk is None:
                continue

            with self.lock:
                is_listening = self.listening
                is_recording_ptt = self.recording_for_stt

            if not (is_listening or is_recording_ptt):
                if self.wake_word_enabled and self.wake_word_detector:
                    ww = self.wake_word_detector.detect(audio_chunk)
                    if ww:
                        print(f"Wake word detected: {ww}")
                        self.overlay.update_status("📢 Wake Word Detected!", "cyan")
                        # Start listening
                        self._toggle_listening(force_start=True)

            elif is_listening:
                # Automatic listening logic (VAD)
                with self.lock:
                    self.audio_buffer.append(audio_chunk)

                rms = self._calculate_rms(audio_chunk)
                current_time = time.time()

                # Check silence
                if rms < self.silence_threshold:
                    if self.silence_start_time is None:
                        self.silence_start_time = current_time
                    elif current_time - self.silence_start_time > self.silence_duration:
                        print("Silence detected, stopping recording.")
                        self._toggle_listening(force_stop=True)
                else:
                    self.silence_start_time = None

                # Max duration check
                if self.speech_start_time and (current_time - self.speech_start_time > self.max_recording_duration):
                     print("Max duration reached, stopping recording.")
                     self._toggle_listening(force_stop=True)

            elif is_recording_ptt:
                # Manual PTT: Just append, user controls stop
                with self.lock:
                    self.audio_buffer.append(audio_chunk)

    def _handle_ptt_press(self):
        with self.lock:
            if not self.recording_for_stt:
                print("PTT Pressed - Recording...")
                self.overlay.update_status("🔴 Recording...", "red")
                self.recording_for_stt = True
                self.audio_buffer = []

    def _handle_ptt_release(self, event):
        with self.lock:
            was_recording = self.recording_for_stt
            if was_recording:
                print("PTT Released - Transcribing...")
                self.overlay.update_status("⚙️ Processing...", "orange")
                self.recording_for_stt = False

        if was_recording:
             self._process_audio_buffer(mode="type")

    def _toggle_listening(self, force_start=False, force_stop=False):
        should_process = False

        with self.lock:
            if force_start:
                if not self.listening:
                    self.listening = True
                    self.audio_buffer = []
                    self.speech_start_time = time.time()
                    self.silence_start_time = None
                    print('\a')
                    print("Listening started (auto)...")
                    self.overlay.update_status("👂 Listening...", "green")
            elif force_stop:
                if self.listening:
                    self.listening = False
                    should_process = True
            else:
                # Manual toggle
                if self.listening:
                    self.listening = False
                    should_process = True
                else:
                    self.listening = True
                    self.audio_buffer = []
                    self.speech_start_time = time.time()
                    self.silence_start_time = None
                    print('\a')
                    print("Listening started (manual)...")
                    self.overlay.update_status("👂 Listening...", "green")

        if should_process:
            print("Listening stopped.")
            self.overlay.update_status("⚙️ Processing...", "orange")
            # Process in separate thread to avoid blocking loop if possible,
            # but current structure calls it directly.
            # self._process_audio_buffer calls lock so it's safeish.
            threading.Thread(target=self._process_audio_buffer, args=("assistant",)).start()

    def _process_audio_buffer(self, mode="assistant"):
        with self.lock:
            if not self.audio_buffer:
                self.overlay.update_status("💤 Idle", "#AAA")
                return
            full_audio = np.concatenate(self.audio_buffer)
            self.audio_buffer = []

        # Simple auditory feedback
        print('\a')

        if self.stt_provider:
            print("Transcribing...")
            self.overlay.update_status("📝 Transcribing...", "blue")
            text = self.stt_provider.transcribe(full_audio)
            print(f"Transcribed: {text}")

            if text:
                # Add to memory
                self.memory_store.add_message("user", text)

                if mode == "type":
                    self._type_text(text)
                elif mode == "assistant":
                    self._handle_assistant_command(text)
            else:
                self.overlay.update_status("💤 Idle", "#AAA")
        else:
            self.overlay.update_status("💤 Idle", "#AAA")

    def _type_text(self, text):
        if self.config.get("privacy_mode", False):
            print("Privacy Mode enabled: Clipboard typing disabled.")
            self.overlay.update_status("🔒 Privacy Mode Blocked Typing", "red")
            return

        try:
            old_clipboard = pyperclip.paste()
            pyperclip.copy(text)
            keyboard.send('ctrl+v')
            time.sleep(0.1)
            pyperclip.copy(old_clipboard)
            self.overlay.update_status("⌨️ Typed!", "green")
            time.sleep(1)
            self.overlay.update_status("💤 Idle", "#AAA")
        except Exception as e:
            print(f"Error typing text: {e}")
            self.overlay.update_status("❌ Error Typing", "red")

    def _handle_assistant_command(self, text):
        # Retrieve recent history for context
        history = self.memory_store.get_recent_history(limit=5)

        context_str = "\n".join([f"{msg['role']}: {msg['content']}" for msg in history])
        full_prompt = f"Recent History:\n{context_str}\n\nUser: {text}"

        self.overlay.update_status("🧠 Thinking...", "purple")
        response_text = self.llm_service.process(full_prompt)

        # Add response to memory
        self.memory_store.add_message("assistant", response_text)

        self.overlay.update_status("🗣️ Speaking...", "yellow")
        # Speak response
        self.tts_provider.speak(response_text)
        self.overlay.update_status("💤 Idle", "#AAA")
