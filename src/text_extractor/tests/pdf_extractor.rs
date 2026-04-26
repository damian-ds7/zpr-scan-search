use crate::{
    ocr::OcrEngine,
    text_extractor::{PdfExtractor, Result},
};
use image::DynamicImage;

struct MockOcr;
impl OcrEngine for MockOcr {
    fn extract_text_from_image(&self, _data: DynamicImage) -> Result<String> {
        Ok("mocked ocr text".to_string())
    }
}

#[test]
fn test_pdf_extraction_with_text() {
    let path = format!("{}/resources/text.pdf", env!("CARGO_MANIFEST_DIR"));
    let ocr = MockOcr;
    let mut extractor = PdfExtractor::open(&path, &ocr).unwrap();
    let text = extractor.extract_all_text(&ocr).unwrap();

    assert!(text.contains("This is a test pdf"));
}

#[test]
fn test_pdf_extraction_with_text_and_image() {
    let path = format!(
        "{}/resources/text_and_image.pdf",
        env!("CARGO_MANIFEST_DIR")
    );
    let ocr = MockOcr;
    let mut extractor = PdfExtractor::open(&path, &ocr).unwrap();
    let text = extractor.extract_all_text(&ocr).unwrap();

    assert!(text.contains("This is a test pdf with an image\nmocked ocr text\n"));
}

#[test]
fn test_open_non_existent_file() {
    let ocr = MockOcr;
    let result = PdfExtractor::open("non_existent_file.pdf", &ocr);
    assert!(result.is_err());
}
