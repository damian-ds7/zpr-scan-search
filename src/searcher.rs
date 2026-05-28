use std::{ops::Range, sync::Arc};

use crate::error::Result;

#[derive(Debug, Default, Clone)]
pub struct SearchContext {
    pub before: Option<usize>,
    pub after: Option<usize>,
}

#[derive(Debug, Default, Clone)]
pub struct Query {
    pub term: String,
    pub context: SearchContext,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ArcStrSlice {
    text: Arc<str>,
    range: Range<usize>,
}

impl ArcStrSlice {
    pub fn new(text: Arc<str>, range: Range<usize>) -> Self {
        Self { text, range }
    }

    pub fn as_str(&self) -> &str {
        &self.text[self.range.clone()]
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SearchResult {
    pub before: ArcStrSlice,
    pub matched: ArcStrSlice,
    pub after: ArcStrSlice,
}

impl SearchResult {
    pub fn before(&self) -> &str {
        self.before.as_str()
    }
    pub fn matched(&self) -> &str {
        self.matched.as_str()
    }
    pub fn after(&self) -> &str {
        self.after.as_str()
    }
}

/// Interface for structs implementing searching of some kind in the document
pub trait Search {
    fn search(&self, query: &Query) -> Result<impl Iterator<Item = SearchResult>>;
}
