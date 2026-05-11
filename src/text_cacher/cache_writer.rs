use std::{
    io::Write,
    path::PathBuf,
    sync::{
        OnceLock,
        mpsc::{self, Sender},
    },
    thread,
};

use crate::error::{Result, ScanSearchError};

/// Represents a raw write task sent to the background writer.
pub(crate) struct WriteTask {
    pub path: PathBuf,
    pub data: Vec<u8>,
}

/// Messages sent to the background cache writer thread.
pub enum Msg {
    /// Write a new cache job to disk.
    Write(WriteTask),
    /// Block until all previous messages are processed.
    Shutdown(Sender<()>),
}

/// A background worker that handles non-blocking cache persistence.
pub struct CacheWriter {
    tx: Sender<Msg>,
}

impl CacheWriter {
    /// Spawns the background thread and returns the writer handle.
    fn new() -> Self {
        let (tx, rx) = mpsc::channel::<Msg>();

        thread::spawn(move || {
            while let Ok(msg) = rx.recv() {
                match msg {
                    Msg::Write(task) => {
                        if let Err(e) = write(&task) {
                            eprintln!("Cache error: {}", e);
                        }
                    }

                    Msg::Shutdown(done_tx) => {
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
        let _ = self.tx.send(Msg::Shutdown(done_tx));
        let _ = done_rx.recv();
    }
}

/// Saves given `WriteTask` using a temporary file and atomic persist to prevent corrupt data.
pub(crate) fn write(task: &WriteTask) -> Result<()> {
    let dir = task
        .path
        .parent()
        .ok_or_else(|| ScanSearchError::NoParentDir(task.path.clone()))?;
    let mut tmp = tempfile::NamedTempFile::new_in(dir)?;

    tmp.write_all(&task.data)?;

    tmp.persist(&task.path)?;
    Ok(())
}
