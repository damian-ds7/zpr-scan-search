mod image_extractor;
mod pdf_extractor;
#[cfg(test)]
mod tests;
mod universal_extractor;

pub use pdf_extractor::PdfExtractor;

use crate::{error::Result, supported_file::SupportedFile};

/// Interface for extracting text from document files.
pub trait TextExtractor {
    /// Extracts text from the given `SupportedFile`.
    fn extract_from(&self, file: &SupportedFile) -> Result<String>;
}
