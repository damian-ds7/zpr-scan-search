#![allow(dead_code)]
use pyo3::prelude::*;

mod constants;
mod dir_utils;
mod error;
mod file;
mod ocr;
mod searcher;
pub mod sem_searcher;
mod supported_file;
mod text_cacher;
pub mod text_encoder;
mod text_extractor;
pub mod text_searcher;

#[pymodule]
mod scan_search {
    use std::{path::PathBuf, sync::Arc};

    use pyo3::prelude::*;
    use rayon::prelude::*;

    use crate::{
        dir_utils::{ScannerConfig, get_fts_from_paths},
        error::{Result, ScanSearchError},
        file::TextFileLoader,
        ocr::TesseractEngine,
        supported_file::{FileKind, InferDetector, SupportedFile},
        text_cacher::{CacheWriter, LocalCache},
        text_extractor::{PdfExtractor, UniversalExtractor},
    };

    /// Processes given pdf file and saves data to cache file and returns extracted text
    #[pyfunction]
    fn process_file(path: String) -> PyResult<String> {
        let ocr_engine = Arc::new(TesseractEngine::new("eng")?);
        let text_extractor = PdfExtractor::new(ocr_engine);
        let backend = LocalCache;
        let loader = TextFileLoader::new(text_extractor, backend);
        let file = SupportedFile {
            path: PathBuf::from(path),
            kind: FileKind::Pdf,
        };
        let file = loader.load(file)?;
        let word_map = serde_json::to_string(file.map()).map_err(ScanSearchError::from)?;
        Ok(word_map)
    }

    /// Extracts text from multiple files or directories in parallel.
    #[pyfunction]
    #[pyo3(signature = (*paths))]
    fn process_files(paths: Vec<String>) -> PyResult<Vec<String>> {
        let path_bufs: Vec<PathBuf> = paths.into_iter().map(PathBuf::from).collect();
        let config = ScannerConfig::default();
        let detector = InferDetector;

        let supported_files = get_fts_from_paths(path_bufs, &config, &detector);

        let engine = Arc::new(TesseractEngine::new("eng")?);
        let extractor = UniversalExtractor::new(engine);
        let loader = TextFileLoader::new(extractor, LocalCache);

        let results = supported_files
            .into_par_iter()
            .map(|file| {
                let text_file = loader.load(file)?;
                Ok(text_file.text().to_string())
            })
            .collect::<Result<Vec<String>>>();

        Ok(results?)
    }

    /// Shuts down the background cache writer, ensuring all pending writes are completed.
    #[pyfunction]
    fn _cache_shutdown() {
        CacheWriter::get().shutdown();
    }
}
