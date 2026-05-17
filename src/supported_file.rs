use std::path::{Path, PathBuf};

/// Trait for detecting the MIME type of a file.
pub trait MimeDetector {
    /// Detects the MIME type of the file at the given path.
    fn detect(&self, path: &Path) -> Option<&'static str>;
}

/// A MIME detector that uses the `infer` crate to determine the file type.
pub struct InferDetector;

impl MimeDetector for InferDetector {
    fn detect(&self, path: &Path) -> Option<&'static str> {
        Some(infer::get_from_path(path).ok()??.mime_type())
    }
}

/// Supported file kinds for text extraction.
#[derive(PartialEq, Debug)]
pub enum FileKind {
    Pdf,
    Image,
}

/// Represents a file that is supported for text extraction.
#[derive(PartialEq, Debug)]
pub struct SupportedFile {
    pub path: PathBuf,
    pub kind: FileKind,
}

impl SupportedFile {
    /// Creates a new `SupportedFile` from a path if it's of a supported type.
    ///
    /// Uses the provided `detector` to determine the MIME type and maps it to a `FileKind`.
    pub fn from_path<D: MimeDetector>(path: PathBuf, detector: &D) -> Option<Self> {
        let kind = match detector.detect(&path)? {
            "application/pdf" => FileKind::Pdf,
            s if s.starts_with("image/") => FileKind::Image,
            _ => return None,
        };
        Some(Self { path, kind })
    }
}
