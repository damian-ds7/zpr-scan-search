use std::{path::PathBuf, sync::Arc};

use crate::{
    cli::utils::process_files,
    config::ScanSearchConfig,
    error::Result,
    file::{TextFile, TextFileLoader},
    ocr::TesseractEngine,
    supported_file::InferDetector,
    text_encoder::fastembed::FastEmbed,
    text_extractor::UniversalExtractor,
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
}
