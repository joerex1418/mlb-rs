// use reqwest::blocking::get;
// use serde:: {Deserialize, Serialize};
// use serde_json as sj;
use pyo3::{prelude::*};
use pyo3::types::{PyDict};
// use serde_json;
// use pythonize::{pythonize};

mod functions;
mod schemas;

type ScheduleResponse = schemas::schedule::ScheduleResponse;

#[pyclass]
#[derive(Clone)]
struct NameInfo {
    #[pyo3(get,set)]
    pub details: Details
}

#[pymethods]
impl NameInfo {
    pub fn get_first_name(&self) -> String {
        self.details.first.clone()
    }
}

#[pyclass]
#[derive(Clone)]
struct Details {
    #[pyo3(get, set)]
    pub first: String,
    #[pyo3(get, set)]
    pub last: String
}

#[pyfunction]
fn testing() -> Py<PyAny> {
    let python = Python::acquire_gil();
    let py = python.python();
    let details = Details {
        first: "Joe".to_string(),
        last: "Rechenmacher".to_string(),
    };

    let name_info = NameInfo {
        details: details
    };

    name_info.into_py(py)


}

// Get the schedule & scores of all games for the current date
#[pyfunction]
fn get_schedule() -> Py<PyAny> {
    let python = Python::acquire_gil();
    let py = python.python();

    let sched: Option<ScheduleResponse> = functions::get_schedule(None);

    sched.into_py(py)
}

#[pymodule]
fn mlbapi(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(testing, m)?)?;
    m.add_function(wrap_pyfunction!(get_schedule, m)?)?;
    Ok(())
}

