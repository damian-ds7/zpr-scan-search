use std::{path::PathBuf, sync::Arc};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    config::FsScanConfig,
    dir_utils::get_fts_from_paths,
    error::Result,
    file::{FileLoader, TextFile},
    supported_file::MimeDetector,
};

/// Scans the given paths for supported files and loads their text content in parallel.
///
/// Paths may be individual files or directories, directories are traversed according
/// to the rules defined in `config`. Each discovered file is passed to `loader`,
/// which handles extraction and caching.
///
/// # Arguments
/// - `paths`    – File and/or directory paths to process.
/// - `config`   – Controls filesystem traversal.
/// - `detector` – Determines which files are supported / how they are classified.
/// - `loader`   – Extracts text from a supported file, with optional caching.
///
/// # Errors
/// Returns the first extraction error encountered, or an I/O error from traversal.
pub fn process_files<D, L>(
    paths: Vec<String>,
    config: &FsScanConfig,
    detector: D,
    loader: L,
) -> Result<Vec<Arc<TextFile>>>
where
    D: MimeDetector,
    L: FileLoader,
{
    let path_bufs: Vec<PathBuf> = paths.into_iter().map(PathBuf::from).collect();

    let supported_files = get_fts_from_paths(path_bufs, config, &detector);

    supported_files
        .into_par_iter()
        .map(|file| {
            let text_file = loader.load(file, true)?;
            Ok(Arc::new(text_file))
        })
        .collect()
}
