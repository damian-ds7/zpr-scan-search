use std::path::PathBuf;

#[derive(PartialEq, Debug)]
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

#[cfg(test)]
mod tests {
    use crate::filetype::FileType;
    use std::path::PathBuf;

    macro_rules! case {
    (@path $ext:expr) => { PathBuf::from(concat!("file.", $ext)) };

    (pdf, $ext:expr) => { ("pdf", Some(FileType::Pdf(case!(@path $ext)))) };
    (img, $ext:expr) => { ($ext, Some(FileType::Image(case!(@path $ext)))) };
    (none, $ext:expr) => { ($ext, None) };
}

    #[test]
    fn test_file_type_from_path() {
        let name = "file";
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
            let path = PathBuf::from(name.to_string() + "." + ext);
            let ft = FileType::from_path(path);
            assert_eq!(expected, ft)
        }
    }
}
