#![allow(dead_code)]
use pyo3::prelude::*;

mod cacher;
mod cli;
mod config;
mod constants;
mod dir_utils;
mod encoder;
mod error;
mod extractor;
mod file;
mod index_searcher;
mod ocr;
mod py_client;
mod searcher;
mod searcher_utils;
mod sem_searcher;
mod supported_file;

#[pymodule]
mod scan_search {
    use std::path::PathBuf;

    use pyo3::prelude::*;

    #[pymodule_export]
    use crate::py_client::{PyClient, PyQuery, PySearchResult};

    use crate::{
        cacher::{CacheWriter, GlobalCache},
        config::{CacheConfig, ScanSearchConfig},
        ocr,
    };

    /// Shuts down the background cache writer, ensuring all pending writes are completed.
    #[pyfunction]
    fn _cache_shutdown() {
        CacheWriter::get().shutdown();
    }

    #[pyfunction]
    fn get_tessdata_dir() -> PathBuf {
        ocr::get_tessdata_dir()
    }

    #[pyfunction]
    fn clear_old_cache(config: ScanSearchConfig, days: u64) -> PyResult<()> {
        match config.cache {
            CacheConfig::Local => {
                eprintln!("Clearing cache is supported only for global cache");
            }
            CacheConfig::Global { path } => {
                let cache = GlobalCache::new(path);
                cache.cleanup_unused(days)?;
            }
        }
        Ok(())
    }
}
