pub mod game;
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
    #[serde(rename = "type")]
    pub position_type: String,
    pub abbreviation: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Dexterity {
    pub code: String,
    pub description: String
}