# tests/test_integration.py
import unittest
from unittest.mock import MagicMock, patch
import os
import json
import numpy as np
import sys

# Add path
sys.path.append(os.path.abspath("src"))

class TestIntegration(unittest.TestCase):

    def test_pipeline(self):
        # We need to import classes after sys.path update
        from src.assistant_logic import Assistant

        # Patch classes in the module where they are USED, not defined, for robustness
        # src.assistant_logic imports them.

        with patch('src.assistant_logic.AudioRecorder') as MockRecorder, \
             patch('src.assistant_logic.SystemTTSProvider') as MockTTS, \
             patch('src.assistant_logic.WhisperCPPProvider') as MockWhisper, \
             patch('src.assistant_logic.LLMService') as MockLLM, \
             patch('src.assistant_logic.OverlayWindow'): # Mock overlay too

            # Setup Mocks
            recorder_instance = MockRecorder.return_value
            recorder_instance.get_audio.side_effect = [None] # Just init

            stt_instance = MockWhisper.return_value
            stt_instance.transcribe.return_value = "Hello world"

            tts_instance = MockTTS.return_value

            llm_instance = MockLLM.return_value
            llm_instance.process.return_value = "Hi there!"

            config = {
                "wake_word_enabled": False,
                "stt_provider": "whisper_cpp",
                "tts_provider": "system",
                "llm_backend": "api",
                "silence_threshold": 100,
                "voice_id": "test"
            }

            assistant = Assistant(config)

            # Manually trigger process logic
            assistant.audio_buffer = [np.zeros(16000, dtype=np.int16)]
            assistant._process_audio_buffer(mode="assistant")

            # Verify STT called
            stt_instance.transcribe.assert_called()

            # Verify LLM called (checking args)
            args, _ = llm_instance.process.call_args
            self.assertIn("Hello world", args[0])

            # Verify TTS called
            tts_instance.speak.assert_called_with("Hi there!")

if __name__ == '__main__':
    unittest.main()
