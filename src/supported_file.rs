use std::path::{Path, PathBuf};

pub trait MimeDetector {
    fn detect(&self, path: &Path) -> Option<&'static str>;
}

pub struct InferDetector;

impl MimeDetector for InferDetector {
    fn detect(&self, path: &Path) -> Option<&'static str> {
        Some(infer::get_from_path(path).ok()??.mime_type())
    }
}

#[derive(PartialEq, Debug)]
pub enum FileKind {
    Pdf,
    Image,
}

#[derive(PartialEq, Debug)]
pub struct SupportedFile {
    pub path: PathBuf,
    pub kind: FileKind,
}

impl SupportedFile {
    pub fn from_path<D: MimeDetector>(path: PathBuf, detector: &D) -> Option<Self> {
        let kind = match detector.detect(&path)? {
            "application/pdf" => FileKind::Pdf,
            s if s.starts_with("image/") => FileKind::Image,
            _ => return None,
        };
        Some(Self { path, kind })
    }
}
