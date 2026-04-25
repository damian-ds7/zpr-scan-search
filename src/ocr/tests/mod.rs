use crate::ocr::utils::get_tessdata_dir;

use super::*;
use image::{Rgb, RgbImage};
use rayon::prelude::*;

fn gen_digit_9() -> DynamicImage {
    let width = 24;
    let height = 24;

    let mut img = RgbImage::from_pixel(width, height, Rgb([255, 255, 255]));

    let mut set_black = |x: i32, y: i32| {
        if x >= 0 && x < width as i32 && y >= 0 && y < height as i32 {
            img.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]));
        }
    };

    for y in 4..19 {
        for x in 7..17 {
            // Top bar
            if y == 4 && (8..=15).contains(&x) {
                set_black(x, y);
            }

            // Top curve left side
            if (4..=10).contains(&y) && x == 7 {
                set_black(x, y);
            }

            // Top curve right side
            if (4..=11).contains(&y) && x == 16 {
                set_black(x, y);
            }

            // Middle bar
            if y == 11 && (8..=15).contains(&x) {
                set_black(x, y);
            }

            // Bottom right vertical line
            if (11..=18).contains(&y) && x == 16 {
                set_black(x, y);
            }

            // Bottom bar
            if y == 18 && (8..=15).contains(&x) {
                set_black(x, y);
            }
        }
    }

    DynamicImage::ImageRgb8(img)
}

#[test]
fn test_multithreaded_tesseract_order_and_safety() -> Result<()> {
    let tessdata_path = get_tessdata_dir();
    let engine = TesseractEngine::new(tessdata_path.to_str().unwrap(), "eng");
    let mut tasks = Vec::new();
    for _ in 0..15 {
        tasks.push((gen_digit_9(), '9'));
    }

    let results: Result<Vec<(String, char)>> = tasks
        .into_par_iter()
        .map(|(img, expected)| {
            let text = engine.extract_text_from_image(img)?;
            Ok((text, expected))
        })
        .collect();

    for (actual_text, expected_char) in results? {
        let cleaned = actual_text.trim();
        assert!(
            cleaned.contains(expected_char),
            "OCR failed: expected '{}', got '{}'",
            expected_char,
            cleaned
        );
    }
    Ok(())
}
