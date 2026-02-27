import requests
import json
import logging

class OpenClawBackend:
    def __init__(self, url):
        self.url = url
        self.history = []
        self.max_history = 20

    def generate(self, prompt, system_prompt=None):
        try:
            messages = []
            if system_prompt:
                messages.append({"role": "system", "content": system_prompt})

            # Add history
            messages.extend(self.history)

            # Add current prompt
            messages.append({"role": "user", "content": prompt})

            payload = {
                "model": "openclaw-agent", # Specific model name if needed
                "messages": messages,
                "stream": False # Streaming not yet implemented
            }

            headers = {"Content-Type": "application/json"}

            response = requests.post(self.url, json=payload, headers=headers, timeout=60)
            response.raise_for_status()

            data = response.json()
            content = data.get('choices', [{}])[0].get('message', {}).get('content', "")

            if not content:
                logging.warning("OpenClaw returned empty content.")
                return "I didn't get a response from the agent."

            # Update history
            self.history.append({"role": "user", "content": prompt})
            self.history.append({"role": "assistant", "content": content})

            # Trim history
            if len(self.history) > self.max_history:
                self.history = self.history[-self.max_history:]

            return content

        except requests.exceptions.RequestException as e:
            logging.error(f"OpenClaw connection error: {e}")
            return f"Error connecting to OpenClaw agent: {e}"
        except Exception as e:
            logging.error(f"OpenClaw unexpected error: {e}")
            return f"An error occurred: {e}"
