from pathlib import Path

from classconf import ConfigParser
from constants import APP_NAME
from platformdirs import user_config_path
from scan_search.config import ScanSearchConfig


def resolve_config(
    sem_search: bool,
    context_after: int | None,
    context_before: int | None,
    context: int | None,
    follow_links: bool | None,
    include_hidden: bool | None,
    model: str | None,
    languages: tuple[str],
    config_path: Path | None,
) -> ScanSearchConfig:
    if config_path is None:
        config_path = user_config_path(APP_NAME) / "config.toml"
    if not config_path.exists():
        config = ScanSearchConfig()
    else:
        parser = ConfigParser(config_path, ScanSearchConfig)
        config = parser.get(ScanSearchConfig)

    if follow_links:
        config.fs_scan.follow_links = follow_links
    if include_hidden:
        config.fs_scan.include_hidden = include_hidden

    resolved_before = context if context is not None else context_before
    resolved_after = context if context is not None else context_after
    if resolved_before is not None:
        config.search.context_before = resolved_before
    if resolved_after is not None:
        config.search.context_after = resolved_after

    config.search.sem_search = sem_search

    if model is not None:
        config.sem_search.model = model
    if languages:
        config.ocr.languages = list(languages)

    return config
