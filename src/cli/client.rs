use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{
    cacher::LocalCache,
    cli::utils::process_files,
    config::ScanSearchConfig,
    encoder::fastembed::FastEmbed,
    error::Result,
    extractor::UniversalExtractor,
    file::{TextFile, TextFileLoader},
    index_searcher::IndexSearcher,
    ocr::TesseractEngine,
    searcher::{Query, Search, SearchResult},
    sem_searcher::SemSearcher,
    supported_file::InferDetector,
};

/// Client for running searches across a collection of files.
///
/// On construction, all provided paths are loaded, extracted, and encoded
/// so that repeated queries against the same file set are cheap. Both
/// exact-match text search and semantic search are supported, and all
/// searches are parallelised across files via Rayon.
pub struct Client {
    files: Vec<Arc<TextFile>>,
    config: ScanSearchConfig,
}

impl Client {
    /// Build a `Client` from a scan/search configuration and a list of paths.
    ///
    /// Each path is processed:
    /// file-type detection -> text extraction -> embedding encoding -> caching.
    /// Unsupported files are skipped, no error is returned if one is encountered
    /// The resulting [`TextFile`] handles are stored in an `Arc` so they can be
    /// shared cheaply across parallel workers.
    ///
    /// # Errors
    /// Returns an error if Tesseract fails to initialise, or if any file
    /// cannot be loaded or encoded.
    pub fn new(config: ScanSearchConfig, paths: Vec<PathBuf>, reload_cache: bool) -> Result<Self> {
        let detector = InferDetector;
        let cache = LocalCache {
            reload: reload_cache,
        };
        let engine = Arc::new(TesseractEngine::new(&config.ocr)?);
        let extractor = UniversalExtractor::new(engine);
        let encoder = FastEmbed {
            model: config.sem_search.model.clone(),
        };
        let loader = TextFileLoader::new(extractor, cache, encoder);
        let files = process_files(paths, &config, detector, loader)?;
        Ok(Self { files, config })
    }

    /// Run an exact-match text search for `query` across all loaded files.
    ///
    /// Files are searched in parallel. The returned vector contains one entry
    /// per file, pairing the file's path with every [`SearchResult`] found
    /// within it. Files that produce no hits are still included with an empty
    /// result list.
    ///
    /// # Errors
    /// Propagates any I/O or search error encountered while processing a file.
    pub fn search(&self, query: &Query) -> Result<Vec<(PathBuf, Vec<SearchResult>)>> {
        self.files
            .par_iter()
            .map(|file| collect_search(IndexSearcher::new(file.clone()), query, file.path()))
            .collect()
    }

    /// Run a semantic search for `query` across all loaded files.
    ///
    /// Returns an empty list immediately if semantic search is disabled in the
    /// configuration. Otherwise behaves like [`search`](Self::search): files are
    /// queried in parallel and every file gets an entry in the output regardless
    /// of whether it produced any hits.
    ///
    /// # Errors
    /// Propagates any encoding or search error encountered while processing a file.
    pub fn sem_search(&self, query: &Query) -> Result<Vec<(PathBuf, Vec<SearchResult>)>> {
        if !self.config.search.sem_search {
            return Ok(vec![]);
        }
        self.files
            .par_iter()
            .map(|file| {
                collect_search(
                    SemSearcher::new(
                        file.clone(),
                        FastEmbed {
                            model: self.config.sem_search.model.clone(),
                        },
                        self.config.sem_search.queue_size,
                    ),
                    query,
                    file.path(),
                )
            })
            .collect()
    }
}

/// Drive a single [`Search`] to completion and pair the results with the file path.
///
/// This is a thin adapter that collects the lazy iterator returned by
/// [`Search::search`] into a `Vec` and bundles it with an owned copy of `path`,
/// matching the `(PathBuf, Vec<SearchResult>)` shape expected by the parallel
/// map in [`Client::search`] and [`Client::sem_search`].
fn collect_search<S: Search>(
    searcher: S,
    query: &Query,
    path: &Path,
) -> Result<(PathBuf, Vec<SearchResult>)> {
    Ok((path.to_owned(), searcher.search(query)?.collect::<Vec<_>>()))
}
