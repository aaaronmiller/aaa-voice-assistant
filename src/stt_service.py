import abc
import numpy as np
import subprocess
import os
import json
import requests
import io
import wave
import tempfile
import logging

logger = logging.getLogger(__name__)

# Optional dependencies
try:
    import assemblyai as aai
except ImportError:
    aai = None


class STTProvider(abc.ABC):
    @abc.abstractmethod
    def transcribe(self, audio_data):
        pass

class WhisperCPPProvider(STTProvider):
    def __init__(self, binary_path, model_path):
        self.binary_path = binary_path
        self.model_path = model_path

    def transcribe(self, audio_data):
        # audio_data: numpy array int16
        # Whisper.cpp main expects a WAV file or similar.

        # Use tempfile for thread safety and cleanup
        with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as temp_wav:
            with wave.open(temp_wav.name, "wb") as wf:
                wf.setnchannels(1)
                wf.setsampwidth(2)
                wf.setframerate(16000)
                wf.writeframes(audio_data.tobytes())
            temp_path = temp_wav.name

        try:
            # -nt: no timestamp, just text
            cmd = [
                self.binary_path,
                "-m", self.model_path,
                "-f", temp_path,
                "-nt"
            ]

            # Run
            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            return result.stdout.strip()
        except subprocess.CalledProcessError as e:
            logger.error(f"Whisper Error: {e}")
            return ""
        except FileNotFoundError:
             logger.error("Whisper binary not found.")
             return ""
        finally:
             if os.path.exists(temp_path):
                 os.remove(temp_path)

class AssemblyAIProvider(STTProvider):
    def __init__(self, api_key):
        if aai:
            aai.settings.api_key = api_key
        else:
            logger.warning("AssemblyAI SDK not installed.")
        self.transcriber = aai.Transcriber() if aai else None

    def transcribe(self, audio_data):
        if not self.transcriber:
            return "AssemblyAI not available."

        # AssemblyAI SDK usually takes file or stream
        # Let's use buffer

        # Convert numpy to bytes
        buffer = io.BytesIO()
        with wave.open(buffer, "wb") as wf:
            wf.setnchannels(1)
            wf.setsampwidth(2)
            wf.setframerate(16000)
            wf.writeframes(audio_data.tobytes())

        buffer.seek(0)

        try:
            transcript = self.transcriber.transcribe(buffer)
            return transcript.text
        except Exception as e:
            logger.error(f"AssemblyAI Error: {e}")
            return ""

class OpenAIAPIProvider(STTProvider):
    def __init__(self, api_key):
        self.api_key = api_key

    def transcribe(self, audio_data):
        # https://api.openai.com/v1/audio/transcriptions

        buffer = io.BytesIO()
        buffer.name = "audio.wav" # needs name for requests to guess mime
        with wave.open(buffer, "wb") as wf:
            wf.setnchannels(1)
            wf.setsampwidth(2)
            wf.setframerate(16000)
            wf.writeframes(audio_data.tobytes())
        buffer.seek(0)

        headers = {
            "Authorization": f"Bearer {self.api_key}"
        }

        files = {
            "file": ("audio.wav", buffer, "audio/wav"),
            "model": (None, "whisper-1")
        }

        try:
            response = requests.post("https://api.openai.com/v1/audio/transcriptions", headers=headers, files=files, timeout=30)
            response.raise_for_status()
            return response.json()["text"]
        except Exception as e:
            logger.error(f"OpenAI STT Error: {e}")
            return ""
