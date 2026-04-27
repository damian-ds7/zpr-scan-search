from pathlib import Path

from scan_search import process_file

filepath = Path(__file__).resolve()
filepath = filepath.parent.parent / "resources/text.pdf"
print(filepath)

word_map = process_file(str(filepath))
print(word_map)
