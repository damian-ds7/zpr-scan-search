pub mod fastembed;
#[cfg(test)]
pub mod tests;

use crate::{cacher::Embeddings, error::Result};

/// Interface for text encoders
pub trait TextEncoder: Sync + Send {
    fn encode(&self, text: &[&str]) -> Result<Embeddings>;
}
