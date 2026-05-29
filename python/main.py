from pathlib import Path

from scan_search import Client, Query

filepath = Path(__file__).resolve()
filepath = filepath.parent.parent / "resources"
print(filepath)

client = Client([filepath])
results = client.search(Query("quick brown fox", 1, 3))
for path, matches in results:
    print(path)
    for match in matches:
        print("Before")
        print("\t", match.before())
        print("Matched")
        print("\t", match.matched())
        print("After")
        print("\t", match.after())


results = client.search(Query("quick brown fox"))
for path, matches in results:
    print(path)
    for match in matches:
        print("Before")
        print("\t", match.before())
        print("Matched")
        print("\t", match.matched())
        print("After")
        print("\t", match.after())
