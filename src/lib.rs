// TODO: remove later
#![allow(unused)]
use pyo3::prelude::*;

mod constants;
mod error;
mod file;
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
