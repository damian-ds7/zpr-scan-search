import signal
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from threading import Event

import requests
from rich.progress import (
    BarColumn,
    DownloadColumn,
    Progress,
    TaskID,
    TextColumn,
    TimeRemainingColumn,
    TransferSpeedColumn,
)

URL = "https://github.com/tesseract-ocr/tessdata_best/raw/main"
CHUNK = 1024 * 64

DONE_EVENT = Event()


def handle_sigint(_signum, _frame):
    DONE_EVENT.set()


signal.signal(signal.SIGINT, handle_sigint)


def download(url: str, filename: Path, dest: Path, task_id: TaskID, progress: Progress):
    file_path = dest / filename

    with requests.get(url, stream=True) as r:
        r.raise_for_status()

        with open(file_path, "wb") as f:
            for chunk in r.iter_content(CHUNK):
                if chunk:
                    f.write(chunk)
                    progress.update(task_id, advance=len(chunk))
                    if DONE_EVENT.is_set():
                        return


def get_content_length(url: str) -> int:
    r = requests.head(url, allow_redirects=True)
    return int(r.headers.get("Content-Length", 0))


def download_train_data(tessdata_path: Path, langs: list[str]):
    urls = [f"{URL}/{lang}.traineddata" for lang in langs]

    with ThreadPoolExecutor(max_workers=4) as ex:
        total = sum(ex.map(get_content_length, urls))

    progress = Progress(
        TextColumn("[bold blue]Downloading traineddata"),
        BarColumn(bar_width=None),
        "[progress.percentage]{task.percentage:>3.1f}%",
        "•",
        DownloadColumn(),
        "•",
        TransferSpeedColumn(),
        "•",
        TimeRemainingColumn(),
        transient=True,
    )
    with progress, ThreadPoolExecutor(max_workers=4) as ex:
        task_id = progress.add_task("Downloading traineddata", total=total)
        for url in urls:
            filename = Path(url.split("/")[-1])
            ex.submit(download, url, Path(filename), tessdata_path, task_id, progress)
