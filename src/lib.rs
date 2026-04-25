use pyo3::prelude::*;

mod constants;
mod file;
// TODO: get rid of pub later
mod text_cacher;

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
