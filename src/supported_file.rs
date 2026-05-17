use std::path::PathBuf;

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
    pub fn from_path(path: PathBuf) -> Option<Self> {
        let extension = path.extension()?.to_str()?;
        let kind = match extension {
            "pdf" => FileKind::Pdf,
            "png" | "jpeg" | "jpg" | "webp" => FileKind::Image,
            _ => return None,
        };

        Some(Self { path, kind })
    }
}

#[cfg(test)]
mod tests {
    use crate::supported_file::{FileKind, SupportedFile};
    use std::path::PathBuf;

    macro_rules! case {
        (@path $ext:expr) => { PathBuf::from(concat!("file.", $ext)) };
        (pdf, $ext:expr) => { ($ext, Some(SupportedFile { path: case!(@path $ext), kind: FileKind::Pdf })) };
        (img, $ext:expr) => { ($ext, Some(SupportedFile { path: case!(@path $ext), kind: FileKind::Image })) };
        (none, $ext:expr) => { ($ext, None) };
    }

    #[test]
    fn test_file_type_from_path() {
        let cases = [
            case!(pdf, "pdf"),
            case!(img, "png"),
            case!(img, "jpeg"),
            case!(img, "jpg"),
            case!(img, "webp"),
            case!(none, "tar.gz"),
            case!(none, ""),
        ];
        for (ext, expected) in cases {
            let path = PathBuf::from(format!("file.{ext}"));
            assert_eq!(expected, SupportedFile::from_path(path));
        }
    }
}
