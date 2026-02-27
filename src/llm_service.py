import abc
import subprocess
import requests
import json
import os
import time

class LLMBackend(abc.ABC):
    @abc.abstractmethod
    def generate(self, prompt, system_prompt=None):
        pass

class APIBackend(LLMBackend):
    def __init__(self, provider, api_key, model):
        self.provider = provider
        self.api_key = api_key
        self.model = model

    def generate(self, prompt, system_prompt=None):
        if self.provider == "openai":
            return self._call_openai(prompt, system_prompt)
        elif self.provider == "anthropic":
            return self._call_anthropic(prompt, system_prompt)
        else:
            return f"Provider {self.provider} not implemented."

    def _call_openai(self, prompt, system_prompt):
        try:
            headers = {
                "Authorization": f"Bearer {self.api_key}",
                "Content-Type": "application/json"
            }
            data = {
                "model": self.model,
                "messages": [
                    {"role": "system", "content": system_prompt or "You are a helpful assistant."},
                    {"role": "user", "content": prompt}
                ]
            }
            response = requests.post("https://api.openai.com/v1/chat/completions", headers=headers, json=data, timeout=30)
            response.raise_for_status()
            return response.json()['choices'][0]['message']['content']
        except Exception as e:
            return f"OpenAI API Error: {e}"

    def _call_anthropic(self, prompt, system_prompt):
        try:
            headers = {
                "x-api-key": self.api_key,
                "anthropic-version": "2023-06-01",
                "Content-Type": "application/json"
            }
            data = {
                "model": self.model,
                "system": system_prompt or "You are a helpful assistant.",
                "messages": [
                    {"role": "user", "content": prompt}
                ],
                "max_tokens": 1024
            }
            response = requests.post("https://api.anthropic.com/v1/messages", headers=headers, json=data, timeout=30)
            response.raise_for_status()
            return response.json()['content'][0]['text']
        except Exception as e:
            return f"Anthropic API Error: {e}"

class CLIBackend(LLMBackend):
    def __init__(self, command_template):
        self.command_template = command_template

    def generate(self, prompt, system_prompt=None):
        try:
            # Safer CLI execution: pass prompt via stdin to avoid shell injection
            if isinstance(self.command_template, str):
                cmd = self.command_template.split()
            else:
                cmd = self.command_template

            # Ensure cmd is valid
            if not cmd:
                return "Error: Empty command template."

            # subprocess.run with input=prompt
            # timeout added to prevent hanging
            result = subprocess.run(cmd, input=prompt, capture_output=True, text=True, check=True, timeout=60)
            return result.stdout.strip()
        except subprocess.TimeoutExpired:
            return "Error: CLI command timed out."
        except Exception as e:
            return f"Error executing CLI: {e}"

class OpenClawBackend(LLMBackend):
    def __init__(self, url):
        self.url = url
        self.history = [] # Maintain history for agentic context

    def generate(self, prompt, system_prompt=None):
        try:
            # Build messages from history
            messages = [{"role": "system", "content": system_prompt or "You are a helpful assistant."}]
            messages.extend(self.history[-10:]) # Keep last 10 turns
            messages.append({"role": "user", "content": prompt})

            payload = {
                "model": "gpt-3.5-turbo", # OpenClaw might require this
                "messages": messages
            }
            response = requests.post(self.url, json=payload, timeout=30)
            response.raise_for_status()

            content = response.json()['choices'][0]['message']['content']

            # Update history
            self.history.append({"role": "user", "content": prompt})
            self.history.append({"role": "assistant", "content": content})

            return content
        except Exception as e:
            return f"Error communicating with OpenClaw: {e}"

class LLMService:
    def __init__(self, config):
        self.config = config
        self.backend = self._create_backend()

    def _create_backend(self):
        backend_type = self.config.get("llm_backend", "api")

        if backend_type == "openclaw":
            return OpenClawBackend(self.config.get("openclaw_url"))
        elif backend_type == "cli":
            return CLIBackend(self.config.get("cli_command", "cat"))
        else:
            provider = self.config.get("api_provider", "openai")
            key = self.config.get("api_keys", {}).get(provider, "")
            model = self.config.get("llm_model", "gpt-3.5-turbo")
            return APIBackend(provider, key, model)

    def process(self, text):
        persona = self.config.get("persona", "default")
        system_prompt = "You are a helpful assistant."
        if persona == "pirate":
            system_prompt = "You are a pirate. Arrr!"

        return self.backend.generate(text, system_prompt)
