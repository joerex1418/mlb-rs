pub mod generics {
    use pyo3::prelude::*;
    use pyo3::types as types;

    use serde::{Deserialize, Serialize};
    // use pyo3::types::*;

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
    pub struct MlbGeneric {
        #[pyo3(get, set)]
        pub id: usize,
        #[pyo3(get, set)]
        pub name: Option<String>,
        #[pyo3(get, set)]
        pub link: Option<String>
    }
}
    
pub mod team {
    use pyo3::prelude::*;
    use serde::{Deserialize, Serialize};
    
    #[pyclass]
    #[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
    pub struct TeamResponse {
        #[pyo3(get, set)]
        pub copyright: String,
        #[pyo3(get, set)]
        pub teams: Vec<Team>
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Team {
        #[pyo3(get, set)]
        pub id: u16,
        #[pyo3(get, set)]
        pub name: String,
        #[pyo3(get, set)]
        pub link: String,
        #[pyo3(get, set)]
        pub season: usize,
        #[pyo3(get, set)]
        pub venue: super::generics::MlbGeneric,
        #[pyo3(get, set)]
        pub league: LeagueGeneric,
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



}

pub mod schedule {
    use pyo3::prelude::*;
    use serde::{Deserialize, Serialize};
    use crate::schemas::generics::TeamGeneric;

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

pub mod games {
    use pyo3::prelude::*;
    use serde::{Deserialize, Serialize};


    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Game {

    }
}