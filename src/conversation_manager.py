# src/conversation_manager.py
import json

class ConversationManager:
    def __init__(self, backend, max_history=10):
        self.backend = backend
        self.history = []
        self.max_history = max_history

    def add_user_message(self, content):
        self.history.append({"role": "user", "content": content})
        self._trim_history()

    def add_assistant_message(self, content):
        self.history.append({"role": "assistant", "content": content})
        self._trim_history()

    def _trim_history(self):
        if len(self.history) > self.max_history:
            self.history = self.history[-self.max_history:]

    def get_messages(self, system_prompt=None):
        msgs = []
        if system_prompt:
            msgs.append({"role": "system", "content": system_prompt})
        msgs.extend(self.history)
        return msgs

    def process_turn(self, prompt, system_prompt=None):
        self.add_user_message(prompt)

        # Need backend to accept full history
        # For simplicity, if backend is OpenClaw, it manages history itself or we pass all messages?
        # Let's assume backend.generate can take messages or prompt+history

        # For now, pass prompt and let backend handle it, or update backend interface.
        response = self.backend.generate(prompt, system_prompt)

        self.add_assistant_message(response)
        return response
