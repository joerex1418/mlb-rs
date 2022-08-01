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
    details: Details
}

#[pymethods]
impl NameInfo {

}

#[pyclass]
#[derive(Clone)]
struct Details {
    #[pyo3(get, set)]
    first: String,
    #[pyo3(get, set)]
    last: String
}


#[pyfunction]
fn testing() -> Py<PyAny> {
    let python = Python::acquire_gil();
    let py = python.python();
    let details = Details {
        first: "Joe".to_string(),
        last: "Rechenmacher".to_string(),
    };
    // println!("Rust object:");
    // println!("{:#?}", team);

    // println!("Python object:");

    details.into_py(py)


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

