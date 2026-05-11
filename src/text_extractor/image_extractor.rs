use std::path::Path;

use crate::{
    error::{Result, ScanSearchError},
    ocr::OcrEngine,
    text_extractor::TextExtractor,
};

#[allow(dead_code)] // TODO: remove allow
pub struct ImageTextExtractor<E: OcrEngine> {
    engine: E,
}

impl<E: OcrEngine> ImageTextExtractor<E> {
    #[allow(dead_code)] // TODO: remove allow
    pub fn new(engine: E) -> Self {
        Self { engine }
    }
}

impl<E: OcrEngine> TextExtractor for ImageTextExtractor<E> {
    fn extract_from(&self, path: &Path) -> Result<String> {
        let image = image::open(path).map_err(|e| ScanSearchError::Image(e.to_string()))?;
        self.engine.extract_text_from_image(image)
    }
}
