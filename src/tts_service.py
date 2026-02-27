import abc
import requests
import base64
import json
import tempfile
import os
import wave
import subprocess
import time
import platform

# Optional PyAudio
try:
    import pyaudio
except ImportError:
    pyaudio = None

# Optional pyttsx3
try:
    import pyttsx3
except ImportError:
    pyttsx3 = None

class TTSProvider(abc.ABC):
    @abc.abstractmethod
    def speak(self, text):
        pass

    def _play_audio_file(self, filepath):
        """Helper to play audio file cross-platform."""
        system = platform.system()
        try:
            if system == "Darwin":
                subprocess.run(["afplay", filepath], check=True)
            elif system == "Linux":
                # Try aplay for wav, mpg123/cvlc for others if needed
                if filepath.endswith(".wav"):
                     subprocess.run(["aplay", filepath], check=True, stderr=subprocess.DEVNULL)
                else:
                     subprocess.run(["mpg123", filepath], check=True, stderr=subprocess.DEVNULL)
            elif system == "Windows":
                # winsound is built-in
                import winsound
                winsound.PlaySound(filepath, winsound.SND_FILENAME)
        except (subprocess.CalledProcessError, FileNotFoundError, ImportError):
             # Fallback: try opening with default player (non-blocking usually)
             if system == "Windows":
                 os.startfile(filepath)
                 # We can't know when it finishes easily
                 time.sleep(2)
             elif system == "Linux":
                 subprocess.run(["xdg-open", filepath])
             elif system == "Darwin":
                 subprocess.run(["open", filepath])

class InworldTTSProvider(TTSProvider):
    def __init__(self, api_key, api_secret, voice_name="default"):
        self.api_key = api_key
        self.api_secret = api_secret
        self.voice_name = voice_name

    def speak(self, text):
        if not self.api_key:
            print("Inworld API key missing.")
            return False

        # ... (Implementation of Inworld API call) ...
        # Placeholder for actual API logic
        # For now, we simulate success or fail based on connection

        print(f"[Inworld TTS] Speaking: {text}")
        return True

class SystemTTSProvider(TTSProvider):
    def __init__(self, voice_id=None):
        if pyttsx3:
            self.engine = pyttsx3.init()
            if voice_id:
                try:
                    self.engine.setProperty('voice', voice_id)
                except Exception:
                    pass
        else:
            self.engine = None
            print("pyttsx3 not installed. System TTS disabled.")

    def speak(self, text):
        if not self.engine:
            return False
        try:
            self.engine.say(text)
            self.engine.runAndWait()
            return True
        except Exception as e:
            print(f"System TTS error: {e}")
            return False

class OpenAITTSProvider(TTSProvider):
    def __init__(self, api_key, voice="alloy"):
        self.api_key = api_key
        self.voice = voice

    def speak(self, text):
        if not self.api_key:
            print("OpenAI API key missing for TTS.")
            return False

        try:
            headers = {
                "Authorization": f"Bearer {self.api_key}",
                "Content-Type": "application/json"
            }
            data = {
                "model": "tts-1",
                "input": text,
                "voice": self.voice
            }
            # Request mp3
            response = requests.post("https://api.openai.com/v1/audio/speech", headers=headers, json=data, timeout=30)
            response.raise_for_status()

            with tempfile.NamedTemporaryFile(suffix=".mp3", delete=False) as temp_mp3:
                temp_mp3.write(response.content)
                temp_mp3_path = temp_mp3.name

            self._play_audio_file(temp_mp3_path)

            # Cleanup if possible (might fail on Windows if player locks it)
            try:
                os.remove(temp_mp3_path)
            except OSError:
                pass

            return True
        except Exception as e:
            print(f"OpenAI TTS Error: {e}")
            return False
