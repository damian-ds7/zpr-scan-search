#[cfg(test)]
mod tests;
mod utils;

use image::DynamicImage;
use tesseract_rs::TesseractAPI;
use thread_local::ThreadLocal;

use crate::{
    error::{Result, ScanSearchError},
    ocr::utils::get_tessdata_dir,
};

pub trait OcrEngine: Sync {
    fn extract_text_from_image(&self, image_data: DynamicImage) -> Result<String>;
}

pub struct TesseractEngine {
    tess_pool: ThreadLocal<TesseractAPI>,
    tessdata_path: String,
    lang: String,
}

impl TesseractEngine {
    pub fn new(lang: &str) -> Result<Self> {
        let tessdata_path = get_tessdata_dir()
            .into_os_string()
            .into_string()
            .map_err(|e| ScanSearchError::InvalidPath(e.to_string_lossy().into_owned()))?;

        Ok(Self {
            tess_pool: ThreadLocal::new(),
            tessdata_path,
            lang: lang.to_string(),
        })
    }
}

impl OcrEngine for TesseractEngine {
    fn extract_text_from_image(&self, image_data: DynamicImage) -> Result<String> {
        let api = self.tess_pool.get_or_try(|| -> Result<TesseractAPI> {
            let api = TesseractAPI::new();
            api.init(&self.tessdata_path, &self.lang)?;
            Ok(api)
        })?;

        let rgb = image_data.to_rgb8();
        let (w, h) = rgb.dimensions();

        api.set_image(&rgb, w as i32, h as i32, 3, (3 * w) as i32)?;

        #[cfg(test)]
        {
            api.set_variable("tessedit_char_whitelist", "0123456789")?;
            api.set_variable("classify_bln_numeric_mode", "1")?;
        }

        let text = api.get_utf8_text()?;

        Ok(text)
    }
}
