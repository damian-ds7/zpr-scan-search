use image::DynamicImage;

use crate::error::Result;

pub trait OcrEngine: Sync {
    fn extract_text_from_image(&self, image_data: DynamicImage) -> Result<String>;
}

