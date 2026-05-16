#[cfg(test)]
pub mod tests;

pub trait SearchableIterator<'a> {
    fn get_at(&mut self, index: usize) -> Option<&'a str>;
}
pub trait Search {
    fn search(&self, query: &str) -> impl SearchableIterator<'_>;
}
