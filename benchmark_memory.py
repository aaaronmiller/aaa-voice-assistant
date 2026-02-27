
import time
import json
import os
import uuid
from src.memory_store import MemoryStore

DB_PATH = "memory/conversations_bench.jsonl"

def setup_data():
    if os.path.exists(DB_PATH):
        os.remove(DB_PATH)

    # Create a store just to use its logic or manually write
    # We want lots of old data
    old_session = "old_session_123"

    print("Generating data...")
    with open(DB_PATH, "w") as f:
        # Write 50,000 old lines
        for i in range(50000):
            entry = {
                "timestamp": "2023-01-01T00:00:00",
                "session_id": f"old_session_{i%100}",
                "role": "user",
                "content": f"Old message {i}"
            }
            f.write(json.dumps(entry) + "\n")

    # Now add current session data
    ms = MemoryStore(DB_PATH)
    current_session = ms.current_session_id

    for i in range(20):
        ms.add_message("user", f"New message {i}")

    return current_session

def run_benchmark():
    session_id = setup_data()
    ms = MemoryStore(DB_PATH)
    # Force the session id to be the one we just added data for
    ms.current_session_id = session_id

    start_time = time.time()
    # Run multiple times to average
    for _ in range(100):
        _ = ms.get_recent_history(limit=10, session_id=session_id)
    end_time = time.time()

    avg_time = (end_time - start_time) / 100
    print(f"Average time per call: {avg_time:.6f} seconds")

    # Cleanup
    if os.path.exists(DB_PATH):
        os.remove(DB_PATH)

if __name__ == "__main__":
    run_benchmark()
