pub mod people;
pub mod team;
pub mod league;
pub mod venue;
pub mod roster;

use serde::{Serialize,Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonStatus {
    pub code: String,
    pub description: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub code: String,
    pub name: String,
    pub r#type: String,
    pub abbreviation: String
}