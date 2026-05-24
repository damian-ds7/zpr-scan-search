from pathlib import Path

from scan_search import Client

filepath = Path(__file__).resolve()
filepath = filepath.parent.parent / "resources"
print(filepath)

client = Client([filepath])
results = client.sem_search("quick brown fox")
for path, matches in results:
    print(f"{path}: {matches}")
