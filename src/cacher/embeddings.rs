use serde::{Deserialize, Serialize};
use std::ops::Deref;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Embeddings(Vec<Vec<f32>>);

impl Embeddings {
    pub fn new() -> Self {
        Embeddings(vec![])
    }
}

impl Deref for Embeddings {
    type Target = Vec<Vec<f32>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<Vec<f32>>> for Embeddings {
    fn from(vec: Vec<Vec<f32>>) -> Self {
        Embeddings(vec)
    }
}

impl FromIterator<Vec<f32>> for Embeddings {
    fn from_iter<I: IntoIterator<Item = Vec<f32>>>(iter: I) -> Self {
        Embeddings(iter.into_iter().collect())
    }
}
