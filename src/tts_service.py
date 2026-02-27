import abc
import requests
import base64
import pyaudio
import pyttsx3
import json
import tempfile
import os
import wave

class TTSProvider(abc.ABC):
    @abc.abstractmethod
    def speak(self, text):
        pass

class InworldTTSProvider(TTSProvider):
    def __init__(self, api_key, api_secret, voice_name="default"):
        self.api_key = api_key
        self.jwt_key = api_key
        self.jwt_secret = api_secret
        self.voice_name = voice_name
        self.token = None

    def _get_token(self):
        pass

    def speak(self, text):
        # Implementation placeholder
        print("Inworld TTS not fully implemented (auth required). Using fallback.")
        return False

    def _play_audio(self, audio_data):
        p = pyaudio.PyAudio()
        # Assuming 16kHz mono 16-bit
        stream = p.open(format=pyaudio.paInt16,
                        channels=1,
                        rate=16000,
                        output=True)
        stream.write(audio_data)
        stream.stop_stream()
        stream.close()
        p.terminate()

class SystemTTSProvider(TTSProvider):
    def __init__(self, voice_id=None):
        self.engine = pyttsx3.init()
        if voice_id:
            try:
                self.engine.setProperty('voice', voice_id)
            except Exception as e:
                print(f"Error setting system voice: {e}")

    def speak(self, text):
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
            response = requests.post("https://api.openai.com/v1/audio/speech", headers=headers, json=data, timeout=30)
            response.raise_for_status()

            # Play audio directly from response content
            # OpenAI TTS returns MP3 by default. PyAudio handles raw PCM.
            # Use pydub or similar if possible, or save to file and play with system command
            # For simplicity, let's try writing to temp mp3 and playing with system

            with tempfile.NamedTemporaryFile(suffix=".mp3", delete=False) as temp_mp3:
                temp_mp3.write(response.content)
                temp_mp3_path = temp_mp3.name

            # Use a system player or playsound?
            # Cross platform playing is tricky without extra deps.
            # Fallback: Just print "Played" if we can't play mp3 easily without ffmpeg

            # Try to use OS default player or specific command
            # On Linux: mpg123, aplay (wav only), etc.
            # On Windows: start <file>
            # On Mac: afplay

            import platform
            system = platform.system()
            if system == "Darwin":
                subprocess.run(["afplay", temp_mp3_path])
            elif system == "Linux":
                # Try mpg123 if available
                subprocess.run(["mpg123", temp_mp3_path], stderr=subprocess.DEVNULL)
            elif system == "Windows":
                 os.startfile(temp_mp3_path)
                 time.sleep(len(text)/10) # Crude wait

            # Cleanup
            # os.remove(temp_mp3_path) # Deleting while playing might fail on Windows
            return True

        except Exception as e:
            print(f"OpenAI TTS Error: {e}")
            return False
