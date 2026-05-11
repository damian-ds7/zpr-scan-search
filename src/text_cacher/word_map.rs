use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

/// A map that stores words and their occurrences (indices) in a document.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WordMap(HashMap<String, Vec<i32>>);

impl WordMap {
    /// Creates a new, empty WordMap.
    pub fn new() -> Self {
        WordMap(HashMap::new())
    }
}

impl<T> From<T> for WordMap
where
    T: AsRef<str>,
{
    fn from(text: T) -> Self {
        text.as_ref()
            .split_whitespace()
            .enumerate()
            .fold(Self::new(), |mut acc, (i, word)| {
                acc.entry(word.to_string()).or_default().push(i as i32);
                acc
            })
    }
}

impl DerefMut for WordMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for WordMap {
    type Target = HashMap<String, Vec<i32>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
