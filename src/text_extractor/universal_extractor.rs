use std::sync::Arc;

use crate::{
    error::Result,
    ocr::OcrEngine,
    supported_file::{FileKind, SupportedFile},
    text_extractor::{PdfExtractor, TextExtractor, image_extractor::ImageTextExtractor},
};

/// A universal text extractor that supports multiple file formats.
///
/// It delegates the extraction process to specific extractors based on the `FileKind`.
pub struct UniversalExtractor<E: OcrEngine> {
    engine: Arc<E>,
}

impl<E: OcrEngine> UniversalExtractor<E> {
    /// Creates a new `UniversalExtractor` with the specified OCR engine.
    pub fn new(engine: Arc<E>) -> Self {
        Self { engine }
    }
}

impl<E: OcrEngine> TextExtractor for UniversalExtractor<E> {
    /// Extracts text from the given `SupportedFile`.
    ///
    /// It chooses the appropriate extractor (PDF or Image) based on the file's kind.
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
