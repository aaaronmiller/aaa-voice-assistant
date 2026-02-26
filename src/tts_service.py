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
        # Note: Inworld API might require JWT generation first using key/secret

        self.api_key = api_key
        self.jwt_key = api_key
        self.jwt_secret = api_secret
        self.voice_name = voice_name
        self.token = None

    def _get_token(self):
        # Implement token generation if necessary.
        # For now, placeholder or simple usage.
        pass

    def speak(self, text):
        url = "https://api.inworld.ai/tts/v1/voice"

        # Construct headers/auth
        # If the user provided the "inworld api" string which looks like "Key:Secret" base64,
        # it might be an Authorization: Basic <string> header.

        # Let's try to use the raw string provided by user in transcript as the Authorization header value if it starts with 'Basic '.
        # But the transcript says "inworld api WVFl..."
        # I will assume it's the key.

        # NOTE: This implementation is speculative on auth method.
        # The standard Inworld AI API uses Studio Access Token or Client credentials.

        payload = {
            "text": text,
            "voice": self.voice_name,
            # "speed": 1.0,
            # "pitch": 1.0
        }

        # For now, I'll print a warning that Inworld implementation is partial due to Auth complexity
        # and fallback to system TTS if it fails.

        try:
             # Placeholder for actual implementation details
             # response = requests.post(url, json=payload, headers={"Authorization": f"Bearer {self.token}"})
             # if response.status_code == 200:
             #    audio_content = base64.b64decode(response.json()['audio'])
             #    self._play_audio(audio_content)
             print("Inworld TTS not fully implemented (auth required). Using fallback.")
             return False
        except Exception as e:
            print(f"Inworld TTS error: {e}")
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
    def __init__(self):
        self.engine = pyttsx3.init()

    def speak(self, text):
        try:
            self.engine.say(text)
            self.engine.runAndWait()
            return True
        except Exception as e:
            print(f"System TTS error: {e}")
            return False
