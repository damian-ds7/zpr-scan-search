use std::path::PathBuf;

pub enum FileType {
    Pdf(PathBuf),
    Image(PathBuf),
}

impl FileType {
    pub fn from_path(path: PathBuf) -> Option<Self> {
        let extension = path.extension()?.to_str()?;
        match extension {
            "pdf" => Some(Self::Pdf(path)),
            "png" | "jpeg" | "jpg" | "webp" => Some(Self::Image(path)),
            _ => None,
        }
    }
}
