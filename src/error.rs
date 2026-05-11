
use pyo3::{PyErr, exceptions::PyRuntimeError};

/// Central error type for the scan-search project, wrapping external library errors.
#[derive(Debug, thiserror::Error)]
pub enum ScanSearchError {
    #[error("Path is not valid UTF-8: {0}")]
    InvalidPath(String),

    #[error("Image load failed: {0}")]
    Image(String),

    #[error("OCR processing failed: {0}")]
    Ocr(String),

    #[error("PDF processing failed: {0}")]
    Pdf(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Failed to persist temp file: {0}")]
    PersistError(#[from] tempfile::PersistError),
}

impl From<ScanSearchError> for PyErr {
    fn from(value: ScanSearchError) -> Self {
        PyRuntimeError::new_err(value.to_string())
    }
}

/// A specialized Result type for scan-search operations.
pub type Result<T> = std::result::Result<T, ScanSearchError>;
