mod pdf_extractor;
#[cfg(test)]
mod tests;

pub use pdf_extractor::PdfExtractor;

use image::DynamicImage;
use pdf_oxide::PdfDocument;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::path::Path;

use crate::{error::Result, ocr::OcrEngine};

pub trait TextExtractor {
    fn extract_from(&self, path: &Path) -> Result<String>;
}
