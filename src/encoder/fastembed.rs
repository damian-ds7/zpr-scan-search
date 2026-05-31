use crate::cacher::Embeddings;
use crate::encoder::TextEncoder;
use crate::error::Result;
use crate::error::ScanSearchError;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

/// Encoder using the fastembed lib
pub struct FastEmbed {
    pub model: EmbeddingModel,
}

impl Default for FastEmbed {
    fn default() -> Self {
        Self {
            model: EmbeddingModel::AllMiniLML6V2,
        }
    }
}

impl TextEncoder for FastEmbed {
    fn encode(&self, text: &[&str]) -> Result<Embeddings> {
        let mut model = TextEmbedding::try_new(InitOptions::new(self.model.clone()))?;

        let embeddings: Vec<Vec<f32>> = model.embed(text, None)?;

        Ok(Embeddings::from(embeddings))
    }
}

impl From<fastembed::Error> for ScanSearchError {
    fn from(e: fastembed::Error) -> Self {
        ScanSearchError::Embedding(e.to_string())
    }
}
