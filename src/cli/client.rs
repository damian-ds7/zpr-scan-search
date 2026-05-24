use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    cli::utils::process_files,
    config::ScanSearchConfig,
    error::Result,
    file::{TextFile, TextFileLoader},
    ocr::TesseractEngine,
    searcher::Search,
    sem_searcher::SemSearcher,
    supported_file::InferDetector,
    text_encoder::fastembed::FastEmbed,
    text_extractor::UniversalExtractor,
    text_searcher::TextSearcher,
};

pub struct Client {
    files: Vec<Arc<TextFile>>,
    config: ScanSearchConfig,
}

impl Client {
    pub fn new(config: ScanSearchConfig, paths: Vec<PathBuf>) -> Result<Self> {
        let detector = InferDetector;
        let cache = config.cache_config.build();
        let engine = Arc::new(TesseractEngine::new("eng")?);
        let extractor = UniversalExtractor::new(engine);
        let encoder = FastEmbed;
        let loader = TextFileLoader::new(extractor, cache, encoder);
        let files = process_files(paths, &config, detector, loader)?;
        Ok(Self { files, config })
    }

    pub fn test(&self) {
        for file in self.files.iter() {
            println!("{}", file.path().display());
        }
    }

    pub fn search(&self, query: &str) -> Result<Vec<(PathBuf, Vec<Arc<str>>)>> {
        self.files
            .par_iter()
            .map(|file| collect_search(TextSearcher::new(file.clone()), query, file.path()))
            .collect()
    }

    pub fn sem_search(&self, query: &str) -> Result<Vec<(PathBuf, Vec<Arc<str>>)>> {
        if !self.config.search_config.sem_search {
            return Ok(vec![]);
        }
        self.files
            .par_iter()
            .map(|file| {
                collect_search(
                    SemSearcher::new(file.clone(), FastEmbed {}, 100),
                    query,
                    file.path(),
                )
            })
            .collect()
    }
}

fn collect_search<S: Search>(
    searcher: S,
    query: &str,
    path: &Path,
) -> Result<(PathBuf, Vec<Arc<str>>)> {
    Ok((path.to_owned(), searcher.search(query)?.collect::<Vec<_>>()))
}
