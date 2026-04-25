#[cfg(test)]
mod tests;

use image::DynamicImage;
use pdf_oxide::PdfDocument;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::path::Path;

use crate::error::Result;

pub trait OcrEngine: Sync {
    fn extract_text_from_image(&self, image_data: DynamicImage) -> Result<String>;
}

pub struct PdfExtractor {
    document: PdfDocument,
}

impl PdfExtractor {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let document = PdfDocument::open(path.as_ref().to_str().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid path")
        })?)?;
        Ok(Self { document })
    }

    pub fn extract_all_text<E: OcrEngine>(&mut self, ocr_engine: &E) -> Result<String> {
        let mut full_text = String::new();
        let page_count = self.document.page_count()?;

        for i in 0..page_count {
            if let Ok(text) = self.document.extract_text(i) {
                full_text.push_str(text.trim());
                full_text.push('\n');
            }

            let images = self.document.extract_images(i)?;

            let ocr_results: Result<Vec<String>> = images
                .into_par_iter()
                .map(|img| {
                    let dynamic_image = img.to_dynamic_image()?;
                    ocr_engine.extract_text_from_image(dynamic_image)
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
