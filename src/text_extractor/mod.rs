mod image_extractor;
mod pdf_extractor;
#[cfg(test)]
mod tests;

pub use pdf_extractor::PdfExtractor;

use crate::{error::Result, supported_file::SupportedFile};

/// Interface for extracting text from document files.
pub trait TextExtractor {
    /// Extracts text from the file at the specified path.
    fn extract_from(&self, file: &SupportedFile) -> Result<String>;
}
