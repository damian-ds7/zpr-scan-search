use crate::text_encoder::TextEncoder;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
};
pub struct RustBert{}


impl TextEncoder for RustBert {
    fn encode(&self, text: &str) -> crate::error::Result<Vec<Vec<f32>>> {
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
            .create_model()?;
        let embeddings: Vec<Vec<f32>> = model.encode(&text.lines())?;
        Ok(embeddings)
    }
}