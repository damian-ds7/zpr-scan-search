use std::ffi::OsString;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanSearchError {
    #[error("Pdf error: {0}")]
    Pdf(#[from] pdf_oxide::error::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("OCR error: {0}")]
    Ocr(#[from] tesseract_rs::TesseractError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Path is not valid UTF-8: {0}")]
    InvalidPath(String),
}

pub type Result<T> = std::result::Result<T, ScanSearchError>;
