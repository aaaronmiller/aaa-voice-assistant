import abc
import subprocess
import os
import tempfile
import wave
import assemblyai as aai
import json
import requests

class STTProvider(abc.ABC):
    @abc.abstractmethod
    def transcribe(self, audio_data, sample_rate=16000):
        pass

class WhisperCPPProvider(STTProvider):
    def __init__(self, executable_path, model_path):
        self.executable_path = executable_path
        self.model_path = model_path

        if not os.path.exists(self.executable_path):
            print(f"Warning: whisper.cpp executable not found at {self.executable_path}")
        if not os.path.exists(self.model_path):
            print(f"Warning: whisper.cpp model not found at {self.model_path}")

    def transcribe(self, audio_data, sample_rate=16000):
        with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as temp_wav:
            with wave.open(temp_wav.name, 'wb') as wav_file:
                wav_file.setnchannels(1)
                wav_file.setsampwidth(2) # 16-bit
                wav_file.setframerate(sample_rate)
                wav_file.writeframes(audio_data.tobytes())

            temp_wav_path = temp_wav.name

        try:
            # -nt: no timestamp, -f: file
            cmd = [
                self.executable_path,
                "-m", self.model_path,
                "-f", temp_wav_path,
                "-nt"
            ]

            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            return result.stdout.strip()

        except subprocess.CalledProcessError as e:
            print(f"Error running whisper.cpp: {e}")
            return ""
        finally:
            if os.path.exists(temp_wav_path):
                os.remove(temp_wav_path)

class AssemblyAIProvider(STTProvider):
    def __init__(self, api_key):
        if not api_key:
            raise ValueError("AssemblyAI API key is required.")
        aai.settings.api_key = api_key
        self.transcriber = aai.Transcriber()

    def transcribe(self, audio_data, sample_rate=16000):
        with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as temp_wav:
            with wave.open(temp_wav.name, 'wb') as wav_file:
                wav_file.setnchannels(1)
                wav_file.setsampwidth(2)
                wav_file.setframerate(sample_rate)
                wav_file.writeframes(audio_data.tobytes())
            temp_wav_path = temp_wav.name

        try:
            transcript = self.transcriber.transcribe(temp_wav_path)
            if transcript.status == aai.TranscriptStatus.error:
                print(f"AssemblyAI Error: {transcript.error}")
                return ""
            return transcript.text
        except Exception as e:
            print(f"AssemblyAI Exception: {e}")
            return ""
        finally:
             if os.path.exists(temp_wav_path):
                os.remove(temp_wav_path)

class OpenAIAPIProvider(STTProvider):
    def __init__(self, api_key):
        self.api_key = api_key

    def transcribe(self, audio_data, sample_rate=16000):
        # Implementation for OpenAI Whisper API
        if not self.api_key:
             print("OpenAI API key missing for STT.")
             return ""

        with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as temp_wav:
            with wave.open(temp_wav.name, 'wb') as wav_file:
                wav_file.setnchannels(1)
                wav_file.setsampwidth(2)
                wav_file.setframerate(sample_rate)
                wav_file.writeframes(audio_data.tobytes())
            temp_wav_path = temp_wav.name

        try:
            headers = {"Authorization": f"Bearer {self.api_key}"}
            with open(temp_wav_path, "rb") as audio_file:
                files = {"file": audio_file, "model": (None, "whisper-1")}
                response = requests.post("https://api.openai.com/v1/audio/transcriptions", headers=headers, files=files, timeout=30)
                response.raise_for_status()
                return response.json().get("text", "")
        except Exception as e:
            print(f"OpenAI STT Error: {e}")
            return ""
        finally:
            if os.path.exists(temp_wav_path):
                os.remove(temp_wav_path)

class FallbackSTTProvider(STTProvider):
    def __init__(self, providers):
        self.providers = providers

    def transcribe(self, audio_data, sample_rate=16000):
        for provider in self.providers:
            try:
                result = provider.transcribe(audio_data, sample_rate)
                if result:
                    return result
            except Exception as e:
                print(f"Provider failed: {e}")
        return ""
