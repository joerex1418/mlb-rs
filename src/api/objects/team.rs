
#[allow(unused_imports)]
use {
    chrono,
    serde::{Serialize, Deserialize},
    serde_json::{Value}
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamsResponse {
    pub teams: Vec<Team>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: usize,
    pub name: String,
    pub link: String,
    pub season: usize,
    pub venue: super::venue::Venue,
    pub spring_venue: super::venue::SpringVenue,
    pub team_code: String,
    pub file_code: String,
    pub abbreviation: String,
    pub team_name: String,
    pub location_name: String,
    pub first_year_of_play: String,
    pub short_name: String,
    pub franchise_name: String,
    pub club_name: String,
    pub active: bool,
    pub league: super::league::LeagueGeneric,
    pub division: Option<super::league::LeagueGeneric>,
}