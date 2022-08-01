// use reqwest::blocking::get;
// use serde:: {Deserialize, Serialize};
// use serde_json as sj;
use pyo3::prelude::*;

#[pyfunction]
fn say_hello() -> PyResult<String> {
    Ok("Hello".to_string())
}

#[pymodule]
fn mlbapi(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    Ok(())
}

