from pathlib import Path
from unittest.mock import patch

from scan_search.config import ScanSearchConfig
from utils import resolve_config


def call_resolve(
    sem_search: bool = False,
    context_after: int | None = None,
    context_before: int | None = None,
    context: int | None = None,
    follow_links: bool | None = None,
    include_hidden: bool | None = None,
    model: str | None = None,
    languages: tuple[str] | None = None,
    config_path: Path | None = None,
) -> ScanSearchConfig:
    return resolve_config(
        sem_search=sem_search,
        context_after=context_after,
        context_before=context_before,
        context=context,
        follow_links=follow_links,
        include_hidden=include_hidden,
        model=model,
        languages=languages,
        config_path=config_path,
    )


def test_defaults_when_no_config(tmp_path):
    config = call_resolve(config_path=tmp_path / "nonexistent.toml")
    assert isinstance(config, ScanSearchConfig)


def test_loads_config_file(tmp_path):
    config_file = tmp_path / "config.toml"
    config_file.write_text("[search]\ncontext_before = 99\n")
    config = call_resolve(config_path=config_file)
    assert config.search.context_before == 99


def test_uses_default_config_path_when_none(tmp_path):
    with patch("utils.resolve_config.user_config_path", return_value=tmp_path):
        config = call_resolve(config_path=None)
    assert isinstance(config, ScanSearchConfig)


def test_context_overrides_context_before_and_after():
    config = call_resolve(context=7, context_before=2, context_after=3)
    assert config.search.context_before == 7
    assert config.search.context_after == 7


def test_context_before_and_after_used_when_no_context():
    config = call_resolve(context_before=2, context_after=3)
    assert config.search.context_before == 2
    assert config.search.context_after == 3


def test_context_none_leaves_config_default(tmp_path):
    config_file = tmp_path / "config.toml"
    config_file.write_text("[search]\ncontext_before = 10\ncontext_after = 10\n")
    config = call_resolve(config_path=config_file, context=None, context_before=None, context_after=None)
    assert config.search.context_before == 10
    assert config.search.context_after == 10


def test_follow_links_overrides():
    config = call_resolve(follow_links=True)
    assert config.fs_scan.follow_links is True


def test_include_hidden_overrides():
    config = call_resolve(include_hidden=True)
    assert config.fs_scan.include_hidden is True


def test_false_flags_do_not_override_config(tmp_path):
    config_file = tmp_path / "config.toml"
    config_file.write_text("[fs_scan]\nfollow_links = true\ninclude_hidden = true\n")
    config = call_resolve(config_path=config_file, follow_links=False, include_hidden=False)
    assert config.fs_scan.follow_links is True
    assert config.fs_scan.include_hidden is True


def test_sem_search_always_set():
    config = call_resolve(sem_search=True)
    assert config.search.sem_search is True


def test_sem_search_false():
    config = call_resolve(sem_search=False)
    assert config.search.sem_search is False


def test_model_overrides():
    config = call_resolve(model="BGELargeENV15")
    assert config.sem_search.model == "BGELargeENV15"


def test_model_none_leaves_default():
    config = call_resolve(model=None)
    assert config.sem_search.model == "BGESmallENV15"


def test_languages_overrides():
    config = call_resolve(languages=("eng", "deu"))  # ty:ignore[invalid-argument-type]
    assert config.ocr.languages == ["eng", "deu"]


def test_languages_none_leaves_default():
    config = call_resolve(languages=None)
    assert config.ocr.languages == ["eng", "pol"]
