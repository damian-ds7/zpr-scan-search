use std::{path::PathBuf, sync::Arc};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    config::{FsScanConfig, ScanSearchConfig},
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
    paths: Vec<PathBuf>,
    config: &ScanSearchConfig,
    detector: D,
    loader: L,
) -> Result<Vec<Arc<TextFile>>>
where
    D: MimeDetector,
    L: FileLoader,
{
    let supported_files = get_fts_from_paths(paths, &config.fs_scan, &detector);

    supported_files
        .into_par_iter()
        .map(|file| {
            let text_file = loader.load(file, config.search_config.sem_search)?;
            Ok(Arc::new(text_file))
        })
        .collect()
}
