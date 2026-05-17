use crate::{
    error::{Result, ScanSearchError},
    ocr::OcrEngine,
    supported_file::{FileKind, SupportedFile},
    text_extractor::{TextExtractor, universal_extractor::UniversalExtractor},
};
use image::DynamicImage;
use std::{path::PathBuf, sync::Arc};

struct MockOcr;
impl OcrEngine for MockOcr {
    fn extract_text_from_image(&self, _image_data: DynamicImage) -> Result<String> {
        Ok("mocked text".to_string())
    }
}

#[test]
fn test_universal_extractor_dispatch_pdf() {
    let engine = Arc::new(MockOcr);
    let extractor = UniversalExtractor::new(engine);
    let file = SupportedFile {
        path: PathBuf::from("non_existent.pdf"),
        kind: FileKind::Pdf,
    };

    let result = extractor.extract_from(&file);
    assert!(result.is_err());
    match result {
        Err(ScanSearchError::Io(_)) => {}
        _ => panic!("Expected IO error for non-existent PDF, got {:?}", result),
    }
}

#[test]
fn test_universal_extractor_dispatch_image() {
    let engine = Arc::new(MockOcr);
    let extractor = UniversalExtractor::new(engine);
    let file = SupportedFile {
        path: PathBuf::from("non_existent.png"),
        kind: FileKind::Image,
    };

    let result = extractor.extract_from(&file);
    match result {
        Err(ScanSearchError::Image(_)) => {}
        _ => panic!(
            "Expected Image error for non-existent image, got {:?}",
            result
        ),
    }
}

#[test]
fn test_universal_extractor_real_pdf() {
    let engine = Arc::new(MockOcr);
    let extractor = UniversalExtractor::new(engine);
    let path = format!("{}/resources/text.pdf", env!("CARGO_MANIFEST_DIR"));
    let file = SupportedFile {
        path: PathBuf::from(path),
        kind: FileKind::Pdf,
    };

    let result = extractor.extract_from(&file);
    assert!(result.is_ok());
    assert!(result.unwrap().contains("This is a test pdf"));
}
