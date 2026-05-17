use std::sync::Arc;

use crate::{
    error::Result,
    ocr::OcrEngine,
    supported_file::{FileKind, SupportedFile},
    text_extractor::{PdfExtractor, TextExtractor, image_extractor::ImageTextExtractor},
};

pub struct UniversalExtractor<E: OcrEngine> {
    engine: Arc<E>,
}

impl<E: OcrEngine> UniversalExtractor<E> {
    pub fn new(engine: Arc<E>) -> Self {
        Self { engine }
    }
}

impl<E: OcrEngine> TextExtractor for UniversalExtractor<E> {
    fn extract_from(&self, file: &SupportedFile) -> Result<String> {
        match file {
            SupportedFile {
                kind: FileKind::Pdf,
                path: _,
            } => PdfExtractor::new(self.engine.clone()).extract_from(file),
            SupportedFile {
                kind: FileKind::Image,
                path: _,
            } => ImageTextExtractor::new(self.engine.clone()).extract_from(file),
        }
    }
}
