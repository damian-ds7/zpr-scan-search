use std::rc::Rc;

use crate::error::Result;

/// Interface for structs implementing searching of some kind in the document
pub trait Search {
    fn search(&self, query: &str) -> Result<impl Iterator<Item = Rc<str>>>;
}
