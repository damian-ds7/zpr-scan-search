#![allow(dead_code)]
use pyo3::prelude::*;

mod cli;
mod constants;
mod error;
mod file;
mod filetype;
mod ocr;
mod searcher;
pub mod sem_searcher;
mod text_cacher;
pub mod text_encoder;
mod text_extractor;
pub mod text_searcher;

#[pymodule]
mod scan_search {
    use std::path::PathBuf;

    use pyo3::prelude::*;

    use crate::{
        error::ScanSearchError,
        file::TextFileLoader,
        ocr::TesseractEngine,
        text_cacher::{CacheWriter, LocalCache},
        text_extractor::PdfExtractor,
    };

    /// Processes given pdf file and saves data to cache file and returns extracted text
    #[pyfunction]
    fn process_file(path: String) -> PyResult<String> {
        let ocr_engine = TesseractEngine::new("eng")?;
        let text_extractor = PdfExtractor::new(&ocr_engine);
        let backend = LocalCache;
        let loader = TextFileLoader::new(text_extractor, backend);
        let file = loader.load(PathBuf::from(path))?;
        let word_map = serde_json::to_string(file.map()).map_err(ScanSearchError::from)?;
        Ok(word_map)
    }

    /// Shuts down the background cache writer, ensuring all pending writes are completed.
    #[pyfunction]
    fn _cache_shutdown() {
        CacheWriter::get().shutdown();
    }
}
