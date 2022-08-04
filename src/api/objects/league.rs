#[allow(unused_imports)]
use {
    chrono,
    serde::{Serialize, Deserialize},
};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct League {
    pub id: usize,
    pub name: String,
    pub link: String,
    pub abbreviation: String,
    pub name_short: String,
    pub season_state: String,
    pub has_wild_card: bool,
    pub has_split_season: bool,
    pub has_playoff_points: bool,
    pub num_games: usize,
    pub num_teams: usize,
    pub num_wildcard_teams: usize,
    pub season: String,
    pub org_code: String,
    pub divisions_in_use: String,
    pub active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LeagueGeneric {
    pub id: usize,
    pub name: String,
    pub link: String,
}