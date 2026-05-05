mod tesseract_engine;
#[cfg(test)]
mod tests;
mod utils;

use image::DynamicImage;
use thread_local::ThreadLocal;

use crate::{
    error::{Result, ScanSearchError},
    ocr::utils::get_tessdata_dir,
};

pub use tesseract_engine::TesseractEngine;

/// Interface for OCR engines capable of extracting text from images.
pub trait OcrEngine: Sync {
    /// Extracts text from the provided image.
    fn extract_text_from_image(&self, image_data: DynamicImage) -> Result<String>;
}
