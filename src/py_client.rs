use std::path::PathBuf;

use pyo3::{
    Bound, PyResult, pyclass, pymethods,
    types::{PyAnyMethods, PyList, PyListMethods},
};

use crate::{cli::Client, config::ScanSearchConfig};

#[pyclass(name = "Client")]
pub struct PyClient {
    inner: Client,
}

#[pymethods]
impl PyClient {
    #[new]
    pub fn new(paths: &Bound<'_, PyList>) -> PyResult<Self> {
        let paths = paths
            .iter()
            .map(|p| p.extract::<PathBuf>())
            .collect::<PyResult<Vec<PathBuf>>>()?;

        let config = ScanSearchConfig::default();
        let inner = Client::new(config, paths)?;
        Ok(Self { inner })
    }

    pub fn test(&self) {
        self.inner.test();
    }
}
