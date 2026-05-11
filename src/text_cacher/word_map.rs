use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct WordMap(HashMap<String, Vec<i32>>);

impl WordMap {
    pub fn new() -> Self {
        WordMap(HashMap::new())
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
