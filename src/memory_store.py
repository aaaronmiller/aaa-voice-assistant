# src/memory_store.py
import json
import os
import uuid
import datetime

class MemoryStore:
    def __init__(self, db_path="memory/conversations.jsonl"):
        self.db_path = db_path
        os.makedirs(os.path.dirname(db_path), exist_ok=True)
        self.current_session_id = str(uuid.uuid4())

    def add_message(self, role, content, session_id=None):
        if session_id is None:
            session_id = self.current_session_id

        entry = {
            "timestamp": datetime.datetime.utcnow().isoformat(),
            "session_id": session_id,
            "role": role,
            "content": content
        }

        with open(self.db_path, "a") as f:
            f.write(json.dumps(entry) + "\n")

    def get_recent_history(self, limit=10, session_id=None):
        if session_id is None:
            session_id = self.current_session_id

        history = []
        try:
            with open(self.db_path, "r") as f:
                # Read from end efficiently (tail) - hard in python without seeking
                # For MVP, read all and filter
                lines = f.readlines()
                for line in reversed(lines):
                    try:
                        entry = json.loads(line)
                        if entry.get("session_id") == session_id:
                            history.insert(0, {"role": entry["role"], "content": entry["content"]})
                            if len(history) >= limit:
                                break
                    except json.JSONDecodeError:
                        continue
        except FileNotFoundError:
            return []

        return history

    def search_memory(self, query):
        # Placeholder for RAG / semantic search
        # Would require embeddings (sentence-transformers) + vector store (chroma/faiss)
        # For now, simple keyword search
        results = []
        try:
            with open(self.db_path, "r") as f:
                for line in f:
                    try:
                        entry = json.loads(line)
                        if query.lower() in entry["content"].lower():
                            results.append(entry["content"])
                    except:
                        continue
        except FileNotFoundError:
            return []
        return results[-3:] # return top 3 recent matches
