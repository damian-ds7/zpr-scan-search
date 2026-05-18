pub mod scanner;
#[cfg(test)]
mod tests;

pub use scanner::{ScannerConfig, get_fts_from_paths};
