use std::{
    collections::HashMap,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
    sync::{
        Arc,
        mpsc::{self, Sender},
    },
    thread,
};

use crate::{constants::DELIMITER, error::Result};

pub enum Msg {
    Write(Job),
    Flush(Sender<()>),
}

pub struct CacheWriter {
    tx: Sender<Msg>,
}

pub struct Job {
    pub text: Arc<String>,
    pub map: Arc<HashMap<String, Vec<i32>>>,
    pub path: PathBuf,
}

impl CacheWriter {
    pub fn new() -> Self {
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

    pub fn submit(&self, msg: Msg) {
        let _ = self.tx.send(msg);
    }

    pub fn flush(&self) {
        let (done_tx, done_rx) = mpsc::channel();
        let _ = self.tx.send(Msg::Flush(done_tx));
        let _ = done_rx.recv();
    }
}

pub fn save_to_disk(text: &str, map: &HashMap<String, Vec<i32>>, path: &Path) -> Result<()> {
    let mut cache_path = path.to_path_buf();
    if let Some(file_name) = cache_path.file_name().and_then(|f| f.to_str()) {
        cache_path.set_file_name(format!("{}.cache", file_name));
    }

    let mut file = File::create(cache_path)?;
    serde_json::to_writer(&mut file, map)?;
    file.write_all(&[DELIMITER])?;
    file.write_all(text.as_bytes())?;
    Ok(())
}
