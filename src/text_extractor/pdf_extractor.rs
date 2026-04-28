use crate::text_extractor::TextExtractor;
use image::DynamicImage;
use pdf_oxide::PdfDocument;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::path::Path;

use crate::{error::Result, ocr::OcrEngine};

pub struct PdfExtractor<'a, E: OcrEngine> {
    ocr_engine: &'a E,
}

impl<'a, E: OcrEngine> PdfExtractor<'a, E> {
    pub fn new(ocr_engine: &'a E) -> Self {
        Self { ocr_engine }
}
}

impl<'a, E: OcrEngine> TextExtractor for PdfExtractor<'a, E> {
    fn extract_from(&self, path: &Path) -> Result<String> {
        let mut document = PdfDocument::open(path.to_str().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path")
        })?)?;
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
