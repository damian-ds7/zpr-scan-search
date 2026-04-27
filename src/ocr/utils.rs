use std::error::Error;
use std::path::PathBuf;
use tesseract_rs::TesseractAPI;

fn get_default_tessdata_dir() -> PathBuf {
    if cfg!(target_os = "macos") {
        let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
        PathBuf::from(home_dir)
            .join("Library")
            .join("Application Support")
            .join("tesseract-rs")
            .join("tessdata")
    } else if cfg!(target_os = "linux") {
        let home_dir = std::env::var("HOME").expect("HOME environment variable not set");
        PathBuf::from(home_dir)
            .join(".tesseract-rs")
            .join("tessdata")
    } else if cfg!(target_os = "windows") {
        PathBuf::from(std::env::var("APPDATA").expect("APPDATA environment variable not set"))
            .join("tesseract-rs")
            .join("tessdata")
    } else {
        panic!("Unsupported operating system");
    }
}

pub fn get_tessdata_dir() -> PathBuf {
    match std::env::var("TESSDATA_PREFIX") {
        Ok(dir) => {
            let path = PathBuf::from(dir);
            #[cfg(debug_assertions)]
            println!("Using TESSDATA_PREFIX directory: {:?}", path);
            path
        }
        Err(_) => {
            let default_dir = get_default_tessdata_dir();
            #[cfg(debug_assertions)]
            println!(
                "TESSDATA_PREFIX not set, using default directory: {:?}",
                default_dir
            );
            default_dir
        }
    }
}
