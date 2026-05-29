use fastembed::EmbeddingModel;

use crate::text_cacher::{CacheBackend, LocalCache};

#[derive(Default, Debug)]
pub struct ScanSearchConfig {
    pub fs_scan: FsScanConfig,
    pub cache_config: CacheConfig,
    pub search_config: SearchConfig,
    pub ocr_config: OcrConfig,
    pub sem_search_config: SemSearchConfig,
}

/// Configuration for scanning the filesystem and collecting supported files.
#[derive(Debug)]
pub struct FsScanConfig {
    /// Follow symbolic links when walking the filesystem tree.
    pub follow_links: bool,

    /// Include hidden files and directories in the results.
    pub include_hidden: bool,
}

impl Default for FsScanConfig {
    fn default() -> Self {
        Self {
            follow_links: true,
            include_hidden: false,
        }
    }
}

#[derive(Debug, Default)]
pub enum CacheConfig {
    #[default]
    Local,
}

impl CacheConfig {
    pub fn build(&self) -> impl CacheBackend {
        match self {
            CacheConfig::Local => LocalCache,
        }
    }
}

#[derive(Debug)]
pub struct SearchConfig {
    pub sem_search: bool,
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self { sem_search: true }
    }
}

#[derive(Debug)]
pub struct OcrConfig {
    pub languages: Vec<String>,
}

impl Default for OcrConfig {
    fn default() -> Self {
        Self {
            languages: vec!["eng".into(), "pol".into()],
        }
    }
}

#[derive(Debug, Default)]
pub struct SemSearchConfig {
    pub model: EmbeddingModel,
    pub queue_size: usize,
}
