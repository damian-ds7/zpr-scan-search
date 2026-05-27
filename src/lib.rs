#![allow(dead_code)]
use pyo3::prelude::*;

mod cli;
mod config;
mod constants;
mod dir_utils;
mod error;
mod file;
mod ocr;
mod py_client;
mod searcher;
mod searcher_utils;
mod sem_searcher;
mod supported_file;
mod text_cacher;
mod text_encoder;
mod text_extractor;
mod text_searcher;

#[pymodule]
mod scan_search {
    use std::path::PathBuf;

    use pyo3::prelude::*;

    #[pymodule_export]
    use crate::py_client::{PyClient, PyQuery, PySearchResult};

    use crate::{ocr, text_cacher::CacheWriter};

    /// Shuts down the background cache writer, ensuring all pending writes are completed.
    #[pyfunction]
    fn _cache_shutdown() {
        CacheWriter::get().shutdown();
    }

    #[pyfunction]
    fn get_tessdata_dir() -> PathBuf {
        ocr::get_tessdata_dir()
    }
}
