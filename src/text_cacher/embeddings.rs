use std::ops::Deref;

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
