// TODO: remove later
#![allow(unused)]
use pyo3::prelude::*;

mod constants;
mod error;
mod file;
mod ocr;
mod text_cacher;
mod text_extractor;
pub mod text_searcher;

#[pymodule]
mod scan_search {
    use std::path::{Path, PathBuf};

    use pyo3::prelude::*;

    use crate::{
        error::ScanSearchError, file::TextFile, ocr::TesseractEngine, text_cacher::CacheWriter,
        text_extractor::PdfExtractor,
    };

    /// Processes given pdf file and saves data to cache file and returns word map
    #[pyfunction]
    fn process_file(path: String) -> PyResult<String> {
        let ocr_engine = TesseractEngine::new("eng")?;
        let text_extractor = PdfExtractor::new(&ocr_engine);
        let file = TextFile::new(PathBuf::from(path), &text_extractor)?;
        let word_map = serde_json::to_string(&*file.map).map_err(ScanSearchError::from)?;
        Ok(word_map)
    }

    #[pyfunction]
    fn _cache_shutdown() {
        CacheWriter::get().shutdown();
    }
}
