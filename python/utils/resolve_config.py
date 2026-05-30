from pathlib import Path

from scan_search.config import ScanSearchConfig


def resolve_config(
    semsearch: str | None,
    context_after: int | None,
    context_before: int | None,
    context: int | None,
    follow_links: bool,
    include_hidden: bool,
    model: str | None,
    languages: tuple[str] | None,
    config_path: Path | None,
) -> ScanSearchConfig:
    return ScanSearchConfig()
