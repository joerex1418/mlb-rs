use people::Person;
use pyo3::{prelude::*};
use rosters::RosterResponse;

mod functions;
mod schemas;
mod rosters;
mod people;
mod stats;

type ScheduleResponse = schemas::schedule::ScheduleResponse;

#[allow(unused)]
#[allow(dead_code)]

#[pyfunction]
fn testing() {

}

// Get bio data for a specific person
#[pyfunction]
fn get_person(person_id:usize) -> Py<PyAny> {
    let python: GILGuard = Python::acquire_gil();
    let py: Python = python.python();

    let person: Option<Person> = functions::get_person(person_id);

    person.into_py(py)
}

// Get the entire active roster for a team
#[pyfunction]
fn get_roster(team_id:usize) -> Py<PyAny> {
    let python: GILGuard = Python::acquire_gil();
    let py: Python = python.python();

    let roster: Option<RosterResponse> = functions::get_roster(team_id);

    roster.into_py(py)
}

// Get the schedule & scores of all games for the current date
#[pyfunction]
fn get_schedule() -> Py<PyAny> {
    let python: GILGuard = Python::acquire_gil();
    let py: Python = python.python();

    let sched: Option<ScheduleResponse> = functions::get_schedule(None);

    sched.into_py(py)
}

#[pymodule]
fn mlbapi(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Person>()?;
    // m.add_class::<Person>;
    m.add_function(wrap_pyfunction!(testing, m)?)?;
    m.add_function(wrap_pyfunction!(get_person, m)?)?;
    m.add_function(wrap_pyfunction!(get_roster, m)?)?;
    m.add_function(wrap_pyfunction!(get_schedule, m)?)?;
    Ok(())
}

