#[cfg(test)]
pub mod tests;

use ndarray::Array1;
use crate::file::TextFile;
use crate::text_encoder::TextEncoder;
use crate::searcher::{Search, SearchableIterator};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::BufRead;
use std::str::Lines;

struct CosinedEmbedding{
    similarity: f32,
    location: i32,
}
impl PartialEq for CosinedEmbedding {
    fn eq(&self, other: &Self) -> bool {
        self.similarity == other.similarity && self.location == other.location
    }
}

impl Eq for CosinedEmbedding {}

impl PartialOrd for CosinedEmbedding {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CosinedEmbedding {
    fn cmp(&self, other: &Self) -> Ordering {
        self.similarity
            .partial_cmp(&other.similarity)
            .unwrap_or(Ordering::Equal)
    }
}
fn cosine_similarity(a: &Array1<f32>, b: &Array1<f32>) -> f32 {
    let dot = a.dot(b);
    let norm = (a.dot(a) * b.dot(b)).sqrt();
    dot / norm
}
struct SemSearcher<'a, E: TextEncoder> {
    file: &'a TextFile,
    encoder: E,
}
impl<'a, E: TextEncoder> SemSearcher<'a, E> {
    fn new(file: &'a mut TextFile, encoder: E) -> Self {
        file.set_embeddings(&encoder);
        SemSearcher { file, encoder }
    }
}


struct SemSearcherIterator<'a> {
    iterator:  Lines<'a>,
    locations: Vec<i32>
}

impl <'a> SemSearcherIterator<'a>{
    fn new(file: &'a TextFile, locations: Vec<i32>) -> Self {
        let iterator = file.text().lines();
        SemSearcherIterator{iterator, locations}
    }
}

impl <'a> SearchableIterator<'a> for SemSearcherIterator<'a>{
    fn nth(&mut self, index: usize) -> Option<&'a str> {
        if index < self.locations.len() {
            let val = self.locations.iter().nth(index)?;
            Some(self.iterator.clone().nth(*val as usize)?)
        } else {
            None
        }
    }
}

impl<'a, E: TextEncoder> Search for SemSearcher<'a, E> {
    fn search(&self, query: &str) -> impl SearchableIterator<'_> {
        let mut heap = BinaryHeap::new();
        let encoded = self.encoder.encode(&vec![query]).unwrap();
        let query_vec: Array1<f32> = Array1::from(encoded[0].clone());

        self.file.embeddings
            .as_deref()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(i, line)| {
                let line_vec: Array1<f32> = Array1::from_vec(line.clone());
                heap.push(CosinedEmbedding {
                    similarity: cosine_similarity(&query_vec, &line_vec),
                    location: i as i32,
                })
            });
        let mut locations = Vec::new();
        for _ in 0..10 {
            if let Some(embedding) = heap.pop() {
                locations.push(embedding.location);
            } else {
                break;
            }
        }
        SemSearcherIterator::new(self.file, locations)
    }
}