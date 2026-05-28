use std::sync::Arc;

use crate::error::Result;

#[derive(Debug, Default)]
pub struct Context {
    pub before: Option<usize>,
    pub after: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Query {
    pub term: String,
    pub context: Context,
}

/// Interface for structs implementing searching of some kind in the document
pub trait Search {
    fn search(&self, query: &Query) -> Result<impl Iterator<Item = Arc<str>>>;
}
