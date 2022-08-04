mod utils;
mod functions;
mod objects;
mod stats;
mod api;

use pyo3::{prelude::*};

use serde::{Deserialize, Serialize};
use pythonize::*;

use {
    objects::people::Person,
    objects::schemas::schedule::ScheduleResponse,
    objects::rosters::RosterResponse
};

#[allow(unused)]
#[allow(dead_code)]

#[pyclass]
#[derive(Deserialize,Serialize)]
struct Player {
    name: String,
    pos: String
}

pub mod rust {
    use crate::objects::people::Person;
    use crate::functions::{
        get_person as _get_person,
        get_schedule as _get_schedule,
        get_roster as _get_roster,
        get_team as _get_team
    };
    use crate::objects::rosters::RosterResponse;
    use crate::objects::schemas::{
        team,
        schedule::ScheduleResponse,
    };

    pub fn get_person(person_id:usize) -> Option<Person> {
        _get_person(person_id)
    }

    pub fn get_team(team_id: usize) -> Option<team::Team> {
        _get_team(team_id)
    }

    pub fn get_schedule(date: Option<String>) -> Option<ScheduleResponse> {
        _get_schedule(date)
    }
    
    pub fn get_roster(team_id: usize) -> Option<RosterResponse> {
        _get_roster(team_id)
    }

}

#[pyfunction]
fn testing() -> Py<PyAny> {
    let python: GILGuard = Python::acquire_gil();
    let py: Python = python.python();

    let player: Player = Player { name:"Joe".to_string(),pos:"CF".to_string() };

    player.into_py(py)
    // py_person.unwrap()

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
fn get_roster(team_id:usize) -> Option<Py<PyAny>> {
    let python: GILGuard = Python::acquire_gil();
    let py: Python = python.python();

    let roster: Option<RosterResponse> = functions::get_roster(team_id);

    if let Ok(roster) = pythonize(py, &roster) {
        return Some(roster)
    }

    None
}

// Get the schedule & scores of all games for the current date
#[pyfunction]
fn get_schedule() -> Option<Py<PyAny>> {
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
    // m.add_class::<Person>()?;
    m.add_function(wrap_pyfunction!(testing, m)?)?;
    m.add_function(wrap_pyfunction!(get_person, m)?)?;
    m.add_function(wrap_pyfunction!(get_roster, m)?)?;
    m.add_function(wrap_pyfunction!(get_schedule, m)?)?;
    
    Ok(())
}

