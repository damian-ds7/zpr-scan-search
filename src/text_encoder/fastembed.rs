use std::sync::Arc;
use crate::text_encoder::TextEncoder;
use fastembed::{TextEmbedding, InitOptions, EmbeddingModel};
use crate::error::Result;
pub struct FastEmbed{}


impl TextEncoder for FastEmbed {
    fn encode(&self, text: &Vec<&str>) -> Result<Vec<Vec<f32>>> {
        let mut model = TextEmbedding::try_new(InitOptions::new(EmbeddingModel::AllMiniLML6V2))?;

        let embeddings: Vec<Vec<f32>> = model.embed(text, None)?;

        Ok(embeddings)
    }
}