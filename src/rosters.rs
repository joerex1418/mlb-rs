use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::people::PersonGeneric;
use crate::schemas::generics::{Position};

#[pyclass]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterEntry {
    pub person: PersonGeneric,
    pub jersey_number: Option<String>,
    pub position: Option<Position>,

}