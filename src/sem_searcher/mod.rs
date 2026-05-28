#[cfg(test)]
pub mod tests;
use crate::error::Result;
use crate::error::ScanSearchError::Embedding;
use crate::file::TextFile;
use crate::searcher::{ArcStrSlice, Query, Search, SearchContext, SearchResult};
use crate::searcher_utils::{build_context_ranges, collect_context_fragments};
use crate::text_encoder::TextEncoder;
use ndarray::Array1;
use ordered_float::OrderedFloat;
use std::collections::BinaryHeap;
use std::string::String;
use std::sync::Arc;

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

pub struct SemSearcher<E: TextEncoder> {
    file: Arc<TextFile>,
    encoder: E,
    queue_size: usize,
}

impl<E: TextEncoder> SemSearcher<E> {
    pub fn new(file: Arc<TextFile>, encoder: E, queue_size: usize) -> Self {
        SemSearcher {
            file,
            encoder,
            queue_size,
        }
    }
}

/// SearchableIterator allowing access to most similar lines in the file
struct SemSearcherIterator {
    text: Arc<str>,
    locations: Vec<i32>,
    pos: usize,
    word_pos: usize,
    byte_pos: usize,
    context: SearchContext,
}

impl SemSearcherIterator {
    fn new(text: Arc<str>, locations: Vec<i32>, context: SearchContext) -> Self {
        SemSearcherIterator {
            text,
            locations,
            context,
            pos: 0,
            word_pos: 0,
            byte_pos: 0,
        }
    }
}

impl Iterator for SemSearcherIterator {
    type Item = SearchResult;

    fn next(&mut self) -> Option<Self::Item> {
        let target_index = *self.locations.get(self.pos)? as usize;
        self.pos += 1;

        let fetch_from = target_index.saturating_sub(self.context.before);
        let fetch_to = target_index + self.context.after;

        if self.word_pos > fetch_from {
            self.word_pos = 0;
            self.byte_pos = 0;
        }

        let lines = collect_context_fragments(
            &self.text,
            self.text[self.byte_pos..].lines(),
            fetch_from,
            fetch_to,
            &mut self.word_pos,
            &mut self.byte_pos,
        )?;

        let mid = target_index - fetch_from;
        let (before_range, matched_range, after_range) = build_context_ranges(&lines, mid, 1);

        Some(SearchResult {
            before: ArcStrSlice::new(Arc::clone(&self.text), before_range),
            matched: ArcStrSlice::new(Arc::clone(&self.text), matched_range),
            after: ArcStrSlice::new(Arc::clone(&self.text), after_range),
        })
    }
}

/// Searcher which uses cosine similarity between sentence(line) embeddings
impl<E: TextEncoder> Search for SemSearcher<E> {
    fn search(&self, query: &Query) -> Result<impl Iterator<Item = SearchResult>> {
        let context = query.context.clone();
        if query.term.is_empty() || self.file.text().is_empty() {
            return Ok(SemSearcherIterator::new(
                self.file.text_arc(),
                vec![],
                context,
            ));
        }
        let mut heap = BinaryHeap::new();
        let encoded = self.encoder.encode(&[&query.term]);
        let query_vec = match encoded {
            Ok(encoded) => {
                let query_vec: Array1<f32> = Array1::from(encoded[0].clone());
                query_vec
            }
            Err(_) => {
                return Ok(SemSearcherIterator::new(
                    self.file.text_arc(),
                    vec![],
                    context,
                ));
            }
        };

        match self.file.embeddings.as_deref() {
            Some(embeddings) => {
                embeddings.iter().enumerate().for_each(|(i, line)| {
                    let line_vec: Array1<f32> = Array1::from_vec(line.clone());
                    heap.push(CosinedEmbedding {
                        similarity: OrderedFloat::from(cosine_similarity(&query_vec, &line_vec)),
                        location: i as i32,
                    })
                });
            }
            None => {
                return Err(Embedding(String::from(
                    "Error encountered while reading embeddings",
                )));
            }
        }
        let mut locations = Vec::new();
        for _ in 0..self.queue_size {
            if let Some(embedding) = heap.pop() {
                locations.push(embedding.location);
            } else {
                break;
            }
        }
        Ok(SemSearcherIterator::new(
            self.file.text_arc(),
            locations,
            context,
        ))
    }
}
