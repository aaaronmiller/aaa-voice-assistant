import pyaudio
import numpy as np
import threading
import queue

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
