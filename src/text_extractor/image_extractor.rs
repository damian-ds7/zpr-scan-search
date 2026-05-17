use std::{path::Path, sync::Arc};

use crate::{
    error::{Result, ScanSearchError},
    ocr::OcrEngine,
    text_extractor::TextExtractor,
};

/// Extractor that uses an OCR engine to extract text from image files.
#[allow(dead_code)] // TODO: remove allow
pub struct ImageTextExtractor<E: OcrEngine> {
    engine: Arc<E>,
}

impl<E: OcrEngine + Sync + Send> ImageTextExtractor<E> {
    /// Creates a new ImageTextExtractor with the specified OCR engine.
    #[allow(dead_code)] // TODO: remove allow
    pub fn new(engine: Arc<E>) -> Self {
        Self { engine }
    }
}

impl<E: OcrEngine + Sync + Send> TextExtractor for ImageTextExtractor<E> {
    fn extract_from(&self, path: &Path) -> Result<String> {
        let image = image::open(path).map_err(|e| ScanSearchError::Image(e.to_string()))?;
        self.engine.extract_text_from_image(image)
    }
}
