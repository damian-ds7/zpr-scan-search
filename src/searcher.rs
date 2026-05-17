/// Allows access to elements found by Search
pub trait SearchableIterator<'a> {
    fn get_at(&mut self, index: usize) -> Option<&'a str>;
}

/// Interface for structs implementing searching of some kind in the document
pub trait Search {
    fn search(&self, query: &str) -> impl SearchableIterator<'_>;
}
