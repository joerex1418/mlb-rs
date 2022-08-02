use people::Person;
use pyo3::{prelude::*};
use rosters::RosterResponse;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use pythonize::*;

mod utils;
mod functions;
mod schemas;
mod rosters;
mod people;
mod stats;

type ScheduleResponse = schemas::schedule::ScheduleResponse;

#[allow(unused)]
#[allow(dead_code)]

#[derive(Deserialize,Serialize)]
pub struct Human {
    name: String,
    age: usize
}

#[pyfunction]
fn testing() -> Py<PyAny> {
    let python: GILGuard = Python::acquire_gil();
    let py: Python = python.python();

    let human: Human = Human { name: "Joe Rechenmacher".to_string(), age: 26 };

    let py_person = pythonize(py, &human);

    py_person.unwrap()

}

// Get bio data for a specific person
#[pyfunction]
fn rs_get_person(person_id:usize) -> Option<Py<PyAny>> {
    let python: GILGuard = Python::acquire_gil();
    let py: Python = python.python();

    let person: Option<Person> = functions::get_person(person_id);

    // let person_dict = pythonize(py, &person);

    if let Ok(person) = pythonize(py, &person) {
        return Some(person)
    }

    None
    // person.into_py(py)
}

// Get the entire active roster for a team
#[pyfunction]
fn rs_get_roster(team_id:usize) -> Option<Py<PyAny>> {
    let python: GILGuard = Python::acquire_gil();
    let py: Python = python.python();

    let roster: Option<RosterResponse> = functions::get_roster(team_id);

    if let Ok(roster) = pythonize(py, &roster) {
        return Some(roster)
    }

    None
    // roster.into_py(py)
}

// Get the schedule & scores of all games for the current date
#[pyfunction]
fn rs_get_schedule() -> Option<Py<PyAny>> {
    let python: GILGuard = Python::acquire_gil();
    let py: Python = python.python();

    let sched: Option<ScheduleResponse> = functions::get_schedule(None);

    if let Ok(sched) = pythonize(py, &sched) {
        return Some(sched)
    }

    None
    // sched.into_py(py)
}

#[pymodule]
fn mlbapi(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Person>()?;
    // m.add_class::<Person>;
    m.add_function(wrap_pyfunction!(testing, m)?)?;
    m.add_function(wrap_pyfunction!(rs_get_person, m)?)?;
    m.add_function(wrap_pyfunction!(rs_get_roster, m)?)?;
    m.add_function(wrap_pyfunction!(rs_get_schedule, m)?)?;
    Ok(())
}

