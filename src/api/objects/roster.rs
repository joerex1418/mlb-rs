use std::fmt;

use serde::{Deserialize, Serialize};

use super::people::PersonGeneric;


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterResponse {
    pub roster: Vec<RosterEntry>,
    pub roster_type: String,
}

impl fmt::Display for RosterResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let roster_title: String = format!("{} Roster\n",self.roster_type);
        let mut result: String = String::from(roster_title.as_str());
        for entry in &self.roster {
            let string: String = format!(
                "{abbrev}\t{person}\n", 
                abbrev = entry.position.abbreviation,
                person = entry.person.full_name
            );
            result.push_str(string.as_str());
        }
        write!(f,"{}",result)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RosterEntry {
    pub person: PersonGeneric,
    pub jersey_number: Option<String>,
    pub position: super::Position,
    pub status: super::PersonStatus,
    pub parent_team_id: usize
}

impl fmt::Display for RosterEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}\t\t{}", self.position.code ,self.person.full_name)
    }
}