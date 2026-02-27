# tests/test_assistant.py
import unittest
from unittest.mock import MagicMock, patch
import json
import os

class TestAssistantConfig(unittest.TestCase):
    def test_default_config(self):
        from src.config_manager import ConfigManager
        cm = ConfigManager("test_config.json")
        self.assertEqual(cm.get("hotkey_ptt"), "ctrl+space")
        if os.path.exists("test_config.json"):
            os.remove("test_config.json")

class TestMemoryStore(unittest.TestCase):
    def test_add_and_get(self):
        from src.memory_store import MemoryStore
        ms = MemoryStore("tests/memory.jsonl")
        ms.add_message("user", "Hello", "sess1")
        history = ms.get_recent_history(limit=1, session_id="sess1")
        self.assertEqual(len(history), 1)
        self.assertEqual(history[0]["content"], "Hello")
        if os.path.exists("tests/memory.jsonl"):
            os.remove("tests/memory.jsonl")

if __name__ == '__main__':
    unittest.main()
