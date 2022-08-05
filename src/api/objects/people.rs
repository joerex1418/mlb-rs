use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonResponse {
    pub people: Vec<Person>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    pub id: usize,
    pub full_name: String,
    pub link: String,
    pub first_name: String,
    pub last_name: String,
    pub primary_number: Option<String>,
    pub birth_date: String,
    pub death_date: Option<String>,
    pub current_age: usize,
    pub birth_city: String,
    pub birth_state_province: Option<String>,
    pub birth_country: String,
    pub death_city: Option<String>,
    pub death_state_province: Option<String>,
    pub death_country: Option<String>,
    pub height: String,
    pub weight: usize,
    pub active: Option<bool>,
    pub primary_position: Option<super::Position>,
    pub use_name: Option<String>,
    pub middle_name: Option<String>,
    pub boxscore_name: Option<String>,
    pub nick_name: Option<String>,
    pub gender: String,
    pub is_player: bool,
    pub is_verified: bool,
    pub draft_year: Option<usize>,
    pub pronunciation: String,
    pub mlb_debut_date: String,
    pub bat_side: super::Dexterity,
    pub pitch_hand: super::Dexterity,
    pub name_first_last: String,
    pub name_slug: String,
    pub first_last_name: String,
    pub last_first_name: String,
    pub last_init_name: String,
    pub init_last_name: String,
    #[serde(rename = "fullFMLName")]
    pub full_fml_name: String,
    #[serde(rename = "fullLFMName")]
    pub full_lfm_name: String,
    pub strike_zone_top: f64,
    pub strike_zone_bottom: f64,
    // pub roster_entries: Option<Vec<RosterEntry>>,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonGeneric {
    pub id: usize,
    pub full_name: String
}

#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterEntry {
    pub jersey_number: String,
    pub position: super::Position,
    pub status: super::PersonStatus,
    pub team: RosterEntryTeam,
    pub is_active: bool,
    pub start_date: String,
    pub status_date: String,
    pub is_active_forty_man: bool,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterEntryTeam {
    pub id: usize,
    pub name: String,
    pub link: String,
    pub abbreviation: String,
    pub team_name: String,
    pub location_name: String,
}

impl fmt::Display for Person {
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,"Person - {} ({})", self.full_name, self.id)
    }
}