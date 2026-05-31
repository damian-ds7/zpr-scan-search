use std::path::PathBuf;

use fastembed::EmbeddingModel;
use pyo3::{Borrowed, FromPyObject, PyAny, PyErr, exceptions::PyValueError, types::PyAnyMethods};

use crate::cacher::{CacheBackend, GlobalCache, LocalCache};

#[derive(Default, Debug, FromPyObject)]
pub struct ScanSearchConfig {
    pub fs_scan: FsScanConfig,
    pub search: SearchConfig,
    pub ocr: OcrConfig,
    pub sem_search: SemSearchConfig,
    pub cache: CacheConfig,
}

/// Configuration for scanning the filesystem and collecting supported files.
#[derive(Debug, FromPyObject)]
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

#[derive(Debug)]
pub struct SearchConfig {
    pub sem_search: bool,
}

impl FromPyObject<'_, '_> for SearchConfig {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
        Ok(Self {
            sem_search: obj.getattr("sem_search")?.extract()?,
        })
    }
}

impl Default for SearchConfig {
    fn default() -> Self {
        Self { sem_search: true }
    }
}

#[derive(Debug, FromPyObject)]
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

impl FromPyObject<'_, '_> for SemSearchConfig {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
        let model_str: String = obj.getattr("model")?.extract()?;
        Ok(Self {
            model: model_str.parse().map_err(PyValueError::new_err)?,
            queue_size: obj.getattr("queue_size")?.extract()?,
        })
    }
}

#[derive(Debug, Default)]
pub enum CacheConfig {
    #[default]
    Local,
    Global {
        path: PathBuf,
    },
}

impl CacheConfig {
    pub fn build(&self) -> Box<dyn CacheBackend> {
        match self {
            CacheConfig::Local => Box::new(LocalCache),
            CacheConfig::Global { path } => Box::new(GlobalCache { path: path.clone() }),
        }
    }
}

impl FromPyObject<'_, '_> for CacheConfig {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, '_, PyAny>) -> Result<Self, Self::Error> {
        let cache_type: String = obj.getattr("typ")?.extract()?;
        match cache_type.as_str() {
            "local" => Ok(CacheConfig::Local),
            "global" => {
                let path: PathBuf = obj.getattr("path")?.extract()?;
                Ok(CacheConfig::Global { path })
            }
            other => Err(PyValueError::new_err(format!(
                "Unknown cache type: {other}"
            ))),
        }
    }
}
