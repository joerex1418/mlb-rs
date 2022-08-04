use std::fmt;

use serde::{Deserialize, Serialize};

use super::people::PersonGeneric;


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterResponse {
    pub roster: Vec<RosterEntry>,
    pub roster_type: String,
}

#[allow(unused)]
impl RosterResponse {
    pub fn get_infielders(&self) -> Vec<RosterEntry> {
        let mut filtered_roster: Vec<RosterEntry> = Vec::new();
        for entry in self.roster.iter() {
            let lc_type = entry.position.position_type.clone();
            if lc_type.to_lowercase() == "infielder" {
                filtered_roster.push(entry.clone());
            }
        }
        filtered_roster
    }

    pub fn get_outfielders(&self) -> Vec<RosterEntry> {
        let mut filtered_roster: Vec<RosterEntry> = Vec::new();
        for entry in self.roster.iter() {
            let lc_type = entry.position.position_type.clone();
            if lc_type.to_lowercase() == "outfielder" {
                filtered_roster.push(entry.clone());
            }
        }
        filtered_roster
    }

    pub fn get_pitchers(&self) -> Vec<RosterEntry> {
        let mut filtered_roster: Vec<RosterEntry> = Vec::new();
        for entry in self.roster.iter() {
            let lc_type = entry.position.position_type.clone();
            if lc_type.to_lowercase() == "pitcher" {
                filtered_roster.push(entry.clone());
            }
        }
        filtered_roster
    }

    pub fn get_inactive(&self) -> Vec<RosterEntry> {
        let mut filtered_roster: Vec<RosterEntry> = Vec::new();
        for entry in self.roster.iter() {
            if entry.status.description.as_str() != "Active" {
                filtered_roster.push(entry.clone());
            }
        }
        filtered_roster
    }


}

impl fmt::Display for RosterResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"RosterResponse - [{}]",self.roster_type)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterEntry {
    pub person: PersonGeneric,
    pub jersey_number: Option<String>,
    pub position: super::Position,
    pub status: super::PersonStatus,
}

impl fmt::Display for RosterEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}\t{}", self.position.code ,self.person.full_name)
    }
}