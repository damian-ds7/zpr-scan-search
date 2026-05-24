use std::sync::Arc;

use crate::error::Result;

/// Interface for structs implementing searching of some kind in the document
pub trait Search {
    fn search(&self, query: &str) -> Result<impl Iterator<Item = Arc<str>>>;
}
