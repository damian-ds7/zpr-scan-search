use crate::text_encoder::TextEncoder;
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
pub struct FastEmbed{}


impl TextEncoder for FastEmbed {
    fn encode(&self, text: Vec<&str>) -> crate::error::Result<Vec<Vec<f32>>> {
        let mut model = TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2))?;
        let embeddings: Vec<Vec<f32>> = model.embed(text, None)?;
        Ok(embeddings)
    }
}