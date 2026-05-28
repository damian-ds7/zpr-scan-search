use std::path::PathBuf;

use pyo3::{
    Bound, PyResult, pyclass, pymethods,
    types::{PyAnyMethods, PyList, PyListMethods},
};

use crate::{cli::Client, config::ScanSearchConfig, searcher::Query};

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

    pub fn search(&self, query: &str) -> PyResult<Vec<(PathBuf, Vec<String>)>> {
        let query = Query {
            term: query.into(),
            ..Default::default()
        };
        let results = self.inner.search(&query)?;
        Ok(results
            .into_iter()
            .map(|(path, texts)| {
                (
                    path,
                    texts.iter().map(|s| s.matched().to_string()).collect(),
                )
            })
            .collect())
    }

    pub fn sem_search(&self, query: &str) -> PyResult<Vec<(PathBuf, Vec<String>)>> {
        let query = Query {
            term: query.into(),
            ..Default::default()
        };
        let results = self.inner.sem_search(&query)?;
        Ok(results
            .into_iter()
            .map(|(path, texts)| {
                (
                    path,
                    texts.iter().map(|s| s.matched().to_string()).collect(),
                )
            })
            .collect())
    }
}
