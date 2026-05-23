use crate::error::Result;
use crate::error::ScanSearchError;
use crate::text_cacher::Embeddings;
use crate::text_encoder::TextEncoder;
use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
/// Encoder using the fastembed lib
pub struct FastEmbed;

impl TextEncoder for FastEmbed {
    fn encode(&self, text: &[&str]) -> Result<Embeddings> {
        let mut model = TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2))?;

        let embeddings: Vec<Vec<f32>> = model.embed(text, None)?;

        Ok(Embeddings::from(embeddings))
    }
}

impl From<fastembed::Error> for ScanSearchError {
    fn from(e: fastembed::Error) -> Self {
        ScanSearchError::Embedding(e.to_string())
    }
}
