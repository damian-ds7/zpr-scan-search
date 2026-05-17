#[cfg(test)]
pub mod tests;

use crate::file::TextFile;
use crate::searcher::{Search, SearchableIterator};
use crate::text_encoder::TextEncoder;
use ndarray::Array1;
use std::collections::BinaryHeap;
use std::str::Lines;

use ordered_float::OrderedFloat;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CosinedEmbedding {
    similarity: OrderedFloat<f32>,
    location: i32,
}
fn cosine_similarity(a: &Array1<f32>, b: &Array1<f32>) -> f32 {
    let dot = a.dot(b);
    let norm = (a.dot(a) * b.dot(b)).sqrt();
    dot / norm
}
struct SemSearcher<'a, E: TextEncoder> {
    file: &'a TextFile,
    encoder: E,
    queue_size: usize,
}
impl<'a, E: TextEncoder> SemSearcher<'a, E> {
    fn new(file: &'a mut TextFile, encoder: E, queue_size: usize) -> Self {
        file.set_embeddings(&encoder);
        SemSearcher {
            file,
            encoder,
            queue_size,
        }
    }
}

struct SemSearcherIterator<'a> {
    iterator: Lines<'a>,
    locations: Vec<i32>,
}

impl<'a> SemSearcherIterator<'a> {
    fn new(file: &'a TextFile, locations: Vec<i32>) -> Self {
        let iterator = file.text().lines();
        SemSearcherIterator {
            iterator,
            locations,
        }
    }
}

impl<'a> SearchableIterator<'a> for SemSearcherIterator<'a> {
    fn get_at(&mut self, index: usize) -> Option<&'a str> {
        if index < self.locations.len() {
            let val = self.locations.get(index)?;
            Some(self.iterator.clone().nth(*val as usize)?)
        } else {
            None
        }
    }
}

impl<'a, E: TextEncoder> Search for SemSearcher<'a, E> {
    fn search(&self, query: &str) -> impl SearchableIterator<'_> {
        if query.is_empty() || self.file.text().is_empty(){
            return SemSearcherIterator::new(self.file, vec![]);
        }
        let mut heap = BinaryHeap::new();
        let encoded = self.encoder.encode(&[query]);
        let query_vec = match encoded {
            Ok(encoded) => {
                let query_vec: Array1<f32> = Array1::from(encoded[0].clone());
                query_vec
            }
            Err(_) => return SemSearcherIterator::new(self.file, vec![]),
        };

        self.file
            .embeddings
            .as_deref()
            .unwrap()
            .iter()
            .enumerate()
            .for_each(|(i, line)| {
                let line_vec: Array1<f32> = Array1::from_vec(line.clone());
                heap.push(CosinedEmbedding {
                    similarity: OrderedFloat::from(cosine_similarity(&query_vec, &line_vec)),
                    location: i as i32,
                })
            });
        let mut locations = Vec::new();
        for _ in 0..self.queue_size {
            if let Some(embedding) = heap.pop() {
                locations.push(embedding.location);
            } else {
                break;
            }
        }
        SemSearcherIterator::new(self.file, locations)
    }
}
