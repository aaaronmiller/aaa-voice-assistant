import pyaudio
import numpy as np
import threading
import queue
import time

class AudioRecorder:
    def __init__(self, chunk_size=1280, sample_rate=16000):
        self.chunk_size = chunk_size
        self.sample_rate = sample_rate
        self.p = pyaudio.PyAudio()
        self.stream = None
        self.recording = False
        self.audio_queue = queue.Queue()

    def start_stream(self):
        if self.stream is not None:
            return

        self.stream = self.p.open(
            format=pyaudio.paInt16,
            channels=1,
            rate=self.sample_rate,
            input=True,
            frames_per_buffer=self.chunk_size,
            stream_callback=self._callback
        )
        self.stream.start_stream()
        self.recording = True
        print("Audio stream started.")

    def stop_stream(self):
        if self.stream is None:
            return

        self.recording = False
        self.stream.stop_stream()
        self.stream.close()
        self.stream = None
        print("Audio stream stopped.")

    def _callback(self, in_data, frame_count, time_info, status):
        if self.recording:
            audio_data = np.frombuffer(in_data, dtype=np.int16)
            self.audio_queue.put(audio_data)
        return (in_data, pyaudio.paContinue)

    def get_audio(self):
        try:
            return self.audio_queue.get(timeout=1)
        except queue.Empty:
            return None

    def terminate(self):
        self.stop_stream()
        self.p.terminate()

    def calibrate_silence(self, duration=2):
        print("Calibrating silence threshold... Please remain silent.")

        # Ensure stream is running
        temp_stream = False
        if self.stream is None:
            self.start_stream()
            temp_stream = True
            time.sleep(0.5) # Warmup

        rms_values = []
        end_time = time.time() + duration

        while time.time() < end_time:
            try:
                chunk = self.audio_queue.get(timeout=0.1)
                data = chunk.astype(np.float32)
                rms = np.sqrt(np.mean(data**2))
                rms_values.append(rms)
            except queue.Empty:
                continue

        if temp_stream:
            self.stop_stream()

        if not rms_values:
            print("Calibration failed: No audio data.")
            return 500 # Default

        avg_rms = np.mean(rms_values)
        threshold = max(avg_rms * 1.5, 300) # At least 300, or 1.5x ambient
        print(f"Calibration complete. Ambient RMS: {avg_rms:.2f}, Threshold: {threshold:.2f}")
        return threshold
