use crate::{error::ScanSearchError, supported_file::SupportedFile, text_extractor::TextExtractor};
use pdf_oxide::PdfDocument;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::sync::Arc;

use crate::{error::Result, ocr::OcrEngine};

impl From<pdf_oxide::Error> for ScanSearchError {
    fn from(e: pdf_oxide::Error) -> Self {
        Self::Pdf(e.to_string())
    }
}

/// Extractor specifically designed for PDF documents.
pub struct PdfExtractor<E: OcrEngine> {
    ocr_engine: Arc<E>,
}

impl<E: OcrEngine> PdfExtractor<E> {
    /// Creates a new PdfExtractor using the provided OCR engine.
    pub fn new(ocr_engine: Arc<E>) -> Self {
        Self { ocr_engine }
    }
}

impl<E: OcrEngine> TextExtractor for PdfExtractor<E> {
    fn extract_from(&self, file: &SupportedFile) -> Result<String> {
        let mut document = PdfDocument::open(file.path.to_str().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path")
        })?)
        .map_err(|e| match e {
            pdf_oxide::Error::Io(io_err) => ScanSearchError::Io(io_err),
            other => ScanSearchError::from(other),
        })?;

        let mut full_text = String::new();
        let page_count = document.page_count()?;

        for i in 0..page_count {
            if let Ok(text) = document.extract_text(i) {
                full_text.push_str(text.trim());
                full_text.push('\n');
            }

            let images = document.extract_images(i)?;
            let ocr_results: Result<Vec<String>> = images
                .into_par_iter()
                .map(|img| {
                    let dynamic_image = img.to_dynamic_image()?;
                    self.ocr_engine.extract_text_from_image(dynamic_image)
                })
                .collect();
            for ocr_text in ocr_results? {
                full_text.push_str(&ocr_text);
                full_text.push('\n');
            }
        }
        Ok(full_text)
    }
}
