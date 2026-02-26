import abc
import subprocess
import os
import tempfile
import wave
import assemblyai as aai
import json

class STTProvider(abc.ABC):
    @abc.abstractmethod
    def transcribe(self, audio_data, sample_rate=16000):
        """
        Transcribe audio data (numpy array or raw bytes).
        Returns the transcribed text as a string.
        """
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
        # Save audio to temporary WAV file
        with tempfile.NamedTemporaryFile(suffix=".wav", delete=False) as temp_wav:
            with wave.open(temp_wav.name, 'wb') as wav_file:
                wav_file.setnchannels(1)
                wav_file.setsampwidth(2) # 16-bit
                wav_file.setframerate(sample_rate)
                wav_file.writeframes(audio_data.tobytes())

            temp_wav_path = temp_wav.name

        try:
            # Run whisper.cpp
            # Command format: ./main -m models/ggml-base.bin -f input.wav -otxt
            # We want the text output. Maybe -otxt is not needed if we capture stdout.
            # Default output is to stdout with timestamps. -nt removes timestamps.
            cmd = [
                self.executable_path,
                "-m", self.model_path,
                "-f", temp_wav_path,
                "-nt" # No timestamps
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
        aai.settings.api_key = api_key
        self.transcriber = aai.Transcriber()

    def transcribe(self, audio_data, sample_rate=16000):
        # Save audio to temporary WAV file (AssemblyAI can take file path or buffer)
        # Using file path for simplicity and robustness
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
