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
        block_size = 4096

        try:
            with open(self.db_path, "rb") as f:
                f.seek(0, os.SEEK_END)
                file_size = f.tell()
                remaining_bytes = file_size
                buffer = b""

                while len(history) < limit and (remaining_bytes > 0 or buffer):
                    if remaining_bytes > 0:
                        read_size = min(block_size, remaining_bytes)
                        f.seek(remaining_bytes - read_size)
                        chunk = f.read(read_size)
                        remaining_bytes -= read_size
                        buffer = chunk + buffer

                    lines = buffer.split(b'\n')

                    # If we still have file to read, the first element might be partial
                    if remaining_bytes > 0:
                        buffer = lines.pop(0)
                    else:
                        buffer = b""

                    # Process available complete lines in reverse order
                    for line_bytes in reversed(lines):
                        if not line_bytes.strip():
                            continue

                        try:
                            line_str = line_bytes.decode('utf-8')
                            entry = json.loads(line_str)

                            if entry.get("session_id") == session_id:
                                history.insert(0, {"role": entry["role"], "content": entry["content"]})
                                if len(history) >= limit:
                                    break
                        except (json.JSONDecodeError, UnicodeDecodeError):
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
