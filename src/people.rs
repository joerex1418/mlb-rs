use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schemas::generics::{Position, Dexterity};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonResponse {
    pub people: Vec<Person>
}

#[pyclass]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[pyo3(get,set)]
    pub id: usize,
    #[pyo3(get,set)]
    pub full_name: String,
    #[pyo3(get,set)]
    pub link: String,
    #[pyo3(get,set)]
    pub first_name: String,
    #[pyo3(get,set)]
    pub last_name: String,
    #[pyo3(get,set)]
    pub primary_number: Option<String>,
    #[pyo3(get,set)]
    pub birth_date: String,
    #[pyo3(get,set)]
    pub current_age: i64,
    #[pyo3(get,set)]
    pub birth_city: Option<String>,
    #[pyo3(get,set)]
    pub birth_state_province: Option<String>,
    #[pyo3(get,set)]
    pub birth_country: Option<String>,
    #[pyo3(get,set)]
    pub height: String,
    #[pyo3(get,set)]
    pub weight: usize,
    #[pyo3(get,set)]
    pub active: Option<bool>,
    #[pyo3(get,set)]
    pub primary_position: Option<Position>,
    #[pyo3(get,set)]
    pub use_name: Option<String>,
    #[pyo3(get,set)]
    pub middle_name: Option<String>,
    #[pyo3(get,set)]
    pub boxscore_name: Option<String>,
    #[pyo3(get,set)]
    pub nick_name: Option<String>,
    #[pyo3(get,set)]
    pub gender: String,
    #[pyo3(get,set)]
    pub is_player: bool,
    #[pyo3(get,set)]
    pub is_verified: bool,
    #[pyo3(get,set)]
    pub draft_year: usize,
    #[pyo3(get,set)]
    pub pronunciation: String,
    #[pyo3(get,set)]
    pub mlb_debut_date: String,
    #[pyo3(get,set)]
    pub bat_side: Dexterity,
    #[pyo3(get,set)]
    pub pitch_hand: Dexterity,
    #[pyo3(get,set)]
    pub name_first_last: String,
    #[pyo3(get,set)]
    pub name_slug: String,
    #[pyo3(get,set)]
    pub first_last_name: String,
    #[pyo3(get,set)]
    pub last_first_name: String,
    #[pyo3(get,set)]
    pub last_init_name: String,
    #[pyo3(get,set)]
    pub init_last_name: String,
    #[pyo3(get,set)]
    #[serde(rename = "fullFMLName")]
    pub full_fmlname: String,
    #[pyo3(get,set)]
    #[serde(rename = "fullLFMName")]
    pub full_lfmname: String,
    #[pyo3(get,set)]
    pub strike_zone_top: f64,
    #[pyo3(get,set)]
    pub strike_zone_bottom: f64,
}

#[pyclass]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonGeneric {
    #[pyo3(get,set)]
    pub id: usize,
    #[pyo3(get,set)]
    pub full_name: String,
    #[pyo3(get,set)]
    pub link: String

}