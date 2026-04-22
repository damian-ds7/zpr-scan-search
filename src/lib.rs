use pyo3::prelude::*;

// TODO: get rid of pub later
pub mod text_cacher;
mod file;

/// A Python module implemented in Rust.
#[pymodule]
mod scan_search {
    use pyo3::prelude::*;

    /// Formats the sum of two numbers as string.
    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }
}
