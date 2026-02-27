import numpy as np
import os
import logging
import openwakeword

try:
    from openwakeword.model import Model
except ImportError:
    Model = None

class WakeWordDetector:
    def __init__(self, model_path=None):
        if Model is None:
             raise ImportError("openwakeword not installed.")

        # openwakeword downloads models automatically or uses path
        # models: "hey_jarvis", "alexa", etc.
        try:
            self.model = Model(wakeword_models=["hey_jarvis"])
        except Exception as e:
            logging.error(f"Models not found or failed to load: {e}. Downloading...")
            openwakeword.utils.download_models()
            self.model = Model(wakeword_models=["hey_jarvis"])

    def detect(self, audio_chunk):
        # audio_chunk: int16 numpy array
        # openwakeword expects int16
        prediction = self.model.predict(audio_chunk)

        # check if score > threshold (default 0.5 inside openwakeword usually, but let's check output)
        # prediction is a dict: {'hey_jarvis': 0.002, ...}
        for ww, score in prediction.items():
            if score > 0.5:
                return ww
        return None
