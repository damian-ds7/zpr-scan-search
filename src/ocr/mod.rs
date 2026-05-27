mod tesseract_engine;
#[cfg(test)]
mod tests;
mod utils;

use image::DynamicImage;

use crate::error::Result;

pub use tesseract_engine::TesseractEngine;
pub use utils::get_tessdata_dir;

/// Interface for OCR engines capable of extracting text from images.
pub trait OcrEngine: Sync + Send {
    /// Extracts text from the provided image.
    fn extract_text_from_image(&self, image_data: DynamicImage) -> Result<String>;
}
