pub mod generics {
    use pyo3::prelude::*;
    use serde::{Deserialize, Serialize};
    
    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Position {
        #[pyo3(get, set)]
        pub code: String,
        #[pyo3(get, set)]
        pub name: String,
        #[pyo3(get, set)]
        pub r#type: String,
        #[pyo3(get, set)]
        pub abbreviation: String
    }

    #[pyclass]
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Dexterity {
        #[pyo3(get, set)]
        pub code: String,
        #[pyo3(get, set)]
        pub description: String
    }
    
    #[pyclass]
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct PersonStatus {
        #[pyo3(get,set)]
        pub code: String,
        #[pyo3(get,set)]
        pub description: String
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct TeamGeneric {
        #[pyo3(get, set)]
        pub id: usize,
        #[pyo3(get, set)]
        pub name: String,
        #[pyo3(get, set)]
        pub link: String
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct LeagueCompact {
        id: usize,
        name: Option<String>
    }
    impl LeagueCompact {
        pub fn get_league_name(&self) -> String {
            match self.id {
                103 => {
                    return "American League".to_string();
                }
                104 => {
                    return "National League".to_string();
                }
                200 => {
                    return "American League West".to_string();
                }
                201 => {
                    return "American League East".to_string();
                }
                202 => {
                    return "American League Central".to_string();
                }
                203 => {
                    return "National League West".to_string();
                }
                204 => {
                    return "National League East".to_string();
                }
                205 => {
                    return "National League Central".to_string();
                }
                _ => {
                    return "".to_string();
                }
            }
        }
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct LeagueGeneric {
        #[pyo3(get, set)]
        pub id: usize,
        #[pyo3(get, set)]
        pub name: String,
        #[pyo3(get, set)]
        pub link: String,
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct MlbCompact {
        #[pyo3(get, set)]
        pub id: usize,
        #[pyo3(get, set)]
        pub name: Option<String>,
        #[pyo3(get, set)]
        pub link: Option<String>
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct MlbGeneric {
        #[pyo3(get, set)]
        pub id: usize,
        #[pyo3(get, set)]
        pub name: String,
        #[pyo3(get, set)]
        pub link: String
    }
}
    
pub mod team {
    use pyo3::prelude::*;
    use serde::{Deserialize, Serialize};
    use crate::utils::BASE;

    #[pyclass]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TeamResponse {
        #[pyo3(get, set)]
        pub teams: Vec<Team>
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Team {
        #[pyo3(get, set)]
        pub id: usize,
        #[pyo3(get, set)]
        pub name: String,
        #[pyo3(get, set)]
        pub link: String,
        #[pyo3(get, set)]
        pub season: usize,
        #[pyo3(get, set)]
        pub venue: super::generics::MlbCompact,
        #[pyo3(get, set)]
        pub league: super::generics::LeagueGeneric,
        #[pyo3(get, set)]
        pub team_code: String,
        #[pyo3(get, set)]
        pub file_code: String,
        #[pyo3(get, set)]
        pub club_name: String,
        #[pyo3(get, set)]
        pub abbreviation: String,
        #[pyo3(get, set)]
        pub short_name: String,
        #[pyo3(get, set)]
        pub location_name: String,
        #[pyo3(get, set)]
        pub franchise_name: String,
        #[pyo3(get, set)]
        pub active: bool,
        #[pyo3(get, set)]
        pub first_year_of_play: String,
    }

    #[pymethods]
    impl Team {
        pub fn get_roster_hitting(&self,season:Option<usize>) {

            let query_path: String = match season {
                Some(season) => {
                    format!(
                        "stats=season&season={season}&group=hitting",
                        season = season
                    )
                }
                None => {
                    format!(
                        "stats=season&season={season}&group=hitting",
                        season = self.season.to_string()
                    )
                }
            };

            let url: String = format!(
                "{base}/api/v1/teams/{team_id}/stats?{query_path}",
                base = BASE,
                team_id = self.id.to_string(),
                query_path = query_path,
            );

            println!("{}",url);

        }
    }

}

pub mod schedule {
    use pyo3::prelude::*;
    use serde::{Deserialize, Serialize};
    use super::generics::TeamGeneric;

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ScheduleResponse {
        #[pyo3(get, set)]
        pub copyright: String,
        #[pyo3(get, set)]
        pub total_items: usize,
        #[pyo3(get, set)]
        pub total_events: usize,
        #[pyo3(get, set)]
        pub total_games: usize,
        #[pyo3(get, set)]
        pub total_games_in_progress: usize,
        #[pyo3(get, set)]
        pub dates: Vec<Date>

    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Date {
        #[pyo3(get, set)]
        pub date: String,
        #[pyo3(get, set)]
        pub total_items: usize,
        #[pyo3(get, set)]
        pub total_events: usize,
        #[pyo3(get, set)]
        pub total_games: usize,
        #[pyo3(get, set)]
        pub total_games_in_progress: usize,
        #[pyo3(get, set)]
        pub games: Vec<Game>
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Game {
        #[pyo3(get, set)]
        pub game_pk: usize,
        #[pyo3(get, set)]
        pub game_type: String,
        #[pyo3(get, set)]
        pub season: String,
        #[pyo3(get, set)]
        pub game_date: String,
        #[pyo3(get, set)]
        pub official_date: String,
        #[pyo3(get, set)]
        pub teams: Teams

    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct Teams {
        #[pyo3(get, set)]
        pub away: Team,
        #[pyo3(get, set)]
        pub home: Team,
    }

    #[pymethods]
    impl Teams {
        pub fn get_away_score(&self) -> usize {
            if let Some(score) = self.away.score {
                score
            } else {
                0
            }
        }

        pub fn get_home_score(&self) -> usize {
            if let Some(score) = self.home.score {
                score
            } else {
                0
            }
        }
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Team {
        #[pyo3(get, set)]
        pub score: Option<usize>,
        #[pyo3(get, set)]
        pub league_record: LeagueRecord,
        #[pyo3(get, set)]
        pub is_winner: Option<bool>,
        #[pyo3(get, set)]
        pub split_squad: Option<bool>,
        #[pyo3(get, set)]
        pub series_number: Option<usize>,
        #[pyo3(get, set)]
        pub team: TeamGeneric
    }

    impl Team {
        pub fn get_name(&self) -> String {
            self.team.name.to_string()
        }

        pub fn get_score(&self) -> usize {
            if let Some(score) = self.score {
                score
            } else {
                0
            }
        }
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    pub struct LeagueRecord {
        #[pyo3(get, set)]
        pub wins: usize,
        #[pyo3(get, set)]
        pub losses: usize,
        #[pyo3(get, set)]
        pub pct: String
    }
    

}

pub mod standings {
    use pyo3::prelude::*;
    use serde::{Deserialize, Serialize};

    use super::generics::LeagueCompact;

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StandingsResponse {
        records: Vec<Record>
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Record {
        #[pyo3(get,set)]
        standings_type: String,
        #[pyo3(get,set)]
        league: LeagueCompact,
        #[pyo3(get,set)]
        division: Option<LeagueCompact>,
        #[pyo3(get,set)]
        last_updated: String,

    }

}

pub mod games {
    use pyo3::prelude::*;
    use serde::{Deserialize, Serialize};


    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Game {

    }
}