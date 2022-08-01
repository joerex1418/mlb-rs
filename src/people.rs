use pyo3::prelude::*;
use serde::{Deserialize, Serialize};



#[pyclass]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonGeneric {
    pub id: usize,
    pub full_name: String,
    pub link: String
}