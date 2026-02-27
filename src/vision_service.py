# src/vision_service.py
from PIL import ImageGrab, Image
import io
import base64

class VisionService:
    def capture_screen(self):
        try:
            screenshot = ImageGrab.grab()
            return screenshot
        except Exception as e:
            print(f"Failed to capture screen: {e}")
            return None

    def encode_image(self, image):
        buffered = io.BytesIO()
        image.save(buffered, format="JPEG")
        return base64.b64encode(buffered.getvalue()).decode('utf-8')

    def analyze_image(self, image, prompt="Describe this image"):
        # Placeholder for vision LLM (e.g. GPT-4o)
        # Would require sending base64 to API
        print(f"Analyzing image with prompt: {prompt}")
        return "Image analysis not implemented (requires Vision LLM API)."
