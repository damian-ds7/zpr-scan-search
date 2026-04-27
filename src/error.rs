use std::ffi::OsString;

use pyo3::{PyErr, exceptions::PyRuntimeError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScanSearchError {
    #[error("{0}")]
    Pdf(#[from] pdf_oxide::error::Error),

    #[error("{0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Ocr(#[from] tesseract_rs::TesseractError),

    #[error("{0}")]
    Json(#[from] serde_json::Error),

    #[error("Path is not valid UTF-8: {0}")]
    InvalidPath(String),
}

impl From<ScanSearchError> for PyErr {
    fn from(value: ScanSearchError) -> Self {
        PyRuntimeError::new_err(value.to_string())
    }
}

pub type Result<T> = std::result::Result<T, ScanSearchError>;
