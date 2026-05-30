use std::path::PathBuf;

use pyo3::{
    Bound, PyResult, pyclass, pymethods,
    types::{PyAnyMethods, PyList, PyListMethods},
};

use crate::{
    cli::Client,
    config::ScanSearchConfig,
    searcher::{Query, SearchContext, SearchResult},
};

#[pyclass(name = "Client")]
pub struct PyClient {
    inner: Client,
}

#[pymethods]
impl PyClient {
    #[new]
    pub fn new(
        config: ScanSearchConfig,
        paths: &Bound<'_, PyList>,
        reload_cache: bool,
    ) -> PyResult<Self> {
        let paths = paths
            .iter()
            .map(|p| p.extract::<PathBuf>())
            .collect::<PyResult<Vec<PathBuf>>>()?;
        let inner = Client::new(config, paths, reload_cache)?;
        Ok(Self { inner })
    }

    pub fn search(&self, query: PyQuery) -> PyResult<Vec<(PathBuf, Vec<PySearchResult>)>> {
        let results = self.inner.search(&query.into())?;
        Ok(results
            .into_iter()
            .map(|(path, search_res)| (path, search_res.into_iter().map(|s| s.into()).collect()))
            .collect())
    }

    pub fn sem_search(&self, query: PyQuery) -> PyResult<Vec<(PathBuf, Vec<PySearchResult>)>> {
        let results = self.inner.sem_search(&query.into())?;
        Ok(results
            .into_iter()
            .map(|(path, search_res)| (path, search_res.into_iter().map(|s| s.into()).collect()))
            .collect())
    }
}

#[pyclass(name = "Query", from_py_object)]
#[derive(Clone)]
pub struct PyQuery {
    inner: Query,
}

#[pymethods]
impl PyQuery {
    #[new]
    #[pyo3(signature = (term, before=0, after=0))]
    pub fn new(term: String, before: usize, after: usize) -> Self {
        Self {
            inner: Query {
                term,
                context: SearchContext { before, after },
            },
        }
    }
}

impl From<PyQuery> for Query {
    fn from(value: PyQuery) -> Self {
        value.inner
    }
}

#[pyclass(name = "SearchResult")]
pub struct PySearchResult {
    inner: SearchResult,
}

#[pymethods]
impl PySearchResult {
    pub fn before(&self) -> &str {
        self.inner.before()
    }
    pub fn matched(&self) -> &str {
        self.inner.matched()
    }
    pub fn after(&self) -> &str {
        self.inner.after()
    }
}

impl From<SearchResult> for PySearchResult {
    fn from(value: SearchResult) -> Self {
        Self { inner: value }
    }
}
