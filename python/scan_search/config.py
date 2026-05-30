from dataclasses import field

from classconf import configclass


@configclass(top_level=True)
class FsScanConfig:
    follow_links: bool = True
    include_hidden: bool = False


@configclass
class SearchConfig:
    sem_search: bool = True
    context_before: int = 5
    context_after = 5


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
)
class ScanSearchConfig:
    fs_scan: FsScanConfig = field(default_factory=FsScanConfig)
    search: SearchConfig = field(default_factory=SearchConfig)
    ocr: OcrConfig = field(default_factory=OcrConfig)
    sem_search: SemSearchConfig = field(default_factory=SemSearchConfig)
