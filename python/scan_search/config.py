from dataclasses import field
from pathlib import Path
from typing import ClassVar, Protocol, runtime_checkable

from classconf import ConfigParser, configclass
from constants import APP_NAME
from platformdirs import user_cache_path, user_config_path


@runtime_checkable
class CacheConfig(Protocol):
    typ: ClassVar[str]


@configclass
class LocalCacheConfig:
    typ: ClassVar[str] = "local"


def serialize_path(path: Path) -> str:
    home = Path.home()
    try:
        path = Path("~") / path.relative_to(home)
    except ValueError:
        pass
    return str(path)


def deserialize_path(path: str) -> Path:
    return Path(path).expanduser()


@configclass(name="cache", field_serializers={"path": serialize_path}, field_deserialzers={"path": deserialize_path})
class GlobalCacheConfig:
    typ: ClassVar[str] = "global"
    path: Path = field(default_factory=lambda: user_cache_path(APP_NAME))


def resolve_cache(name: str, parser: ConfigParser) -> CacheConfig:
    match name:
        case "local":
            return LocalCacheConfig()
        case "global":
            return parser.get(GlobalCacheConfig)
        case _:
            raise ValueError(f"Unknown cache type: {name}")


def serialize_cache(cfg: CacheConfig) -> str:
    return cfg.typ


@configclass(top_level=True)
class FsScanConfig:
    follow_links: bool = True
    include_hidden: bool = False


@configclass
class SearchConfig:
    sem_search: bool = False
    context_before: int = 5
    context_after: int = 5


@configclass
class OcrConfig:
    languages: list[str] = field(default_factory=lambda: ["eng", "pol"])


@configclass
class SemSearchConfig:
    # Supported models: https://docs.rs/fastembed/5.13.4/fastembed/enum.EmbeddingModel.html
    model: str = "AllMiniLML6V2"
    queue_size: int = 10


@configclass(
    top_level=True,
    field_name_mappings={"cache": "cache_type"},
    field_deserialzers={"cache": resolve_cache},
    field_serializers={"cache": serialize_cache},
)
class ScanSearchConfig:
    fs_scan: FsScanConfig = field(default_factory=FsScanConfig)
    search: SearchConfig = field(default_factory=SearchConfig)
    ocr: OcrConfig = field(default_factory=OcrConfig)
    sem_search: SemSearchConfig = field(default_factory=SemSearchConfig)
    cache: CacheConfig = field(default_factory=GlobalCacheConfig)


def generate_default_config():
    ConfigParser(
        user_config_path(APP_NAME, ensure_exists=True) / "config.toml",
        ScanSearchConfig,
        GlobalCacheConfig,
        create_noexist=True,
    )
