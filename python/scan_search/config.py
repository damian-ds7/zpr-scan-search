from dataclasses import field
from pathlib import Path
from typing import ClassVar, Protocol, runtime_checkable

from classconf import configclass
from classconf.parser import ConfigParser


@configclass(top_level=True)
class FsScanConfig:
    follow_links: bool = True
    include_hidden: bool = False


@runtime_checkable
class CacheBackend(Protocol):
    type: ClassVar[str]


@configclass
class LocalCacheConfig:
    type: ClassVar[str] = "local"


def resolve_cache(name: str, parser: ConfigParser) -> CacheBackend:
    match name:
        case "local":
            return parser.get(LocalCacheConfig)
        case _:
            raise ValueError(f"Unknown cache type: {name}")


def serialize_cache(cfg: CacheBackend) -> str:
    return cfg.type


@configclass(top_level=True)
class SearchConfig:
    sem_search: bool = True


@configclass
class OcrConfig:
    languages: list[str] = field(default_factory=lambda: ["eng", "pol"])


@configclass
class SemSearchConfig:
    # Supported models: https://docs.rs/fastembed/5.13.4/fastembed/enum.EmbeddingModel.html
    model: str = "BGESmallENV15"
    queue_size: int = 10


@configclass(
    top_level=True,
    field_deserialzers={"cache_config": resolve_cache},
    field_serializers={"cache_config": serialize_cache},
    field_name_mappings={"cache_config": "cache"},
)
class ScanSearchConfig:
    fs_scan: FsScanConfig = field(default_factory=FsScanConfig)
    cache_config: CacheBackend = field(default_factory=LocalCacheConfig)
    search_config: SearchConfig = field(default_factory=SearchConfig)
    ocr_config: OcrConfig = field(default_factory=OcrConfig)
    sem_search_config: SemSearchConfig = field(default_factory=SemSearchConfig)


if __name__ == "__main__":
    parser = ConfigParser(Path("./config.toml"), ScanSearchConfig, create_noexist=True)
