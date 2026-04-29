use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    sync::{
        Arc, OnceLock,
        mpsc::{self, Sender},
    },
    thread,
};

use crate::{constants::DELIMITER, error::Result};

/// Messages sent to the background cache writer thread.
pub enum Msg {
    /// Write a new cache job to disk.
    Write(Job),
    /// Block until all previous messages are processed.
    Flush(Sender<()>),
}

/// A background worker that handles non-blocking cache persistence.
pub struct CacheWriter {
    tx: Sender<Msg>,
}

/// Represents a single cache write task.
pub struct Job {
    pub text: Arc<String>,
    pub map: Arc<HashMap<String, Vec<i32>>>,
    pub path: PathBuf,
}

impl CacheWriter {
    /// Spawns the background thread and returns the writer handle.
    fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Msg>();

        thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                match msg {
                    Msg::Write(job) => {
                        if let Err(e) = save_to_disk(&job.text, &job.map, &job.path) {
                            eprintln!("Cache error: {}", e);
                        }
                    }

                    Msg::Flush(done_tx) => {
                        let _ = done_tx.send(());
                    }
                }
            }
        });

        Self { tx }
    }

    /// Returns the global singleton instance of the CacheWriter.
    pub fn get() -> &'static CacheWriter {
        static INSTANCE: OnceLock<CacheWriter> = OnceLock::new();
        INSTANCE.get_or_init(Self::new)
    }

    /// Submits a message to the background thread without blocking.
    pub fn submit(&self, msg: Msg) {
        let _ = self.tx.send(msg);
    }

    /// Blocks until all pending write jobs have been processed.
    /// Call once before process exit to ensure no writes are lost.
    pub fn shutdown(&self) {
        let (done_tx, done_rx) = mpsc::channel();
        let _ = self.tx.send(Msg::Flush(done_tx));
        let _ = done_rx.recv();
    }
}

/// Atomically saves the word map and text to a .cache file using a temporary file.
fn save_to_disk(text: &str, map: &HashMap<String, Vec<i32>>, path: &Path) -> Result<()> {
    let cache_path = path.with_file_name(
        path.file_name()
            .and_then(|f| f.to_str())
            .map(|name| format!("{}.cache", name))
            .unwrap_or_default(),
    );

    let dir = path.parent().unwrap_or(Path::new("."));
    let mut tmp = tempfile::NamedTempFile::new_in(dir)?;

    serde_json::to_writer(&mut tmp, map)?;
    tmp.write_all(&[DELIMITER])?;
    tmp.write_all(text.as_bytes())?;

    tmp.persist(&cache_path)?;
    Ok(())
}
