import openwakeword
from openwakeword.model import Model
import numpy as np
import os

class WakeWordDetector:
    def __init__(self, model_names=["hey_jarvis"], threshold=0.5):
        # Ensure models are downloaded
        # Check if the default model exists or rely on the library to handle it.
        # openwakeword.utils.download_models() downloads to the package directory or specific path.
        try:
            # Attempt to initialize with default models to see if they exist
            self.owwModel = Model(wakeword_models=model_names)
        except Exception:
            print("Models not found, downloading...")
            openwakeword.utils.download_models()
            self.owwModel = Model(wakeword_models=model_names)

        self.model_names = model_names
        self.threshold = threshold

    def detect(self, audio_chunk):
        """
        Detects wake word in the given audio chunk (numpy array of int16).
        Returns the model name if detected, else None.
        """
        # audio_chunk is int16 numpy array
        prediction = self.owwModel.predict(audio_chunk)

        # prediction is a dict: {model_name: score}
        for name, score in prediction.items():
            if score > self.threshold:
                return name
        return None
