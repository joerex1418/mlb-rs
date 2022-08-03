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

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StreakGeneric {
        #[pyo3(get,set)]
        pub streak_type: String,
        #[pyo3(get,set)]
        pub streak_number: usize,
        #[pyo3(get,set)]
        pub streak_code: String,
    }

    pub mod records {
        use pyo3::prelude::*;
        use serde::{Deserialize, Serialize};
        use super::{LeagueGeneric};

        #[pyclass]
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        pub struct RecordGeneric {
            #[pyo3(get, set)]
            pub wins: usize,
            #[pyo3(get, set)]
            pub losses: usize,
            #[pyo3(get, set)]
            pub pct: String
        }
    
        #[pyclass]
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        pub struct RecordType {
            #[pyo3(get, set)]
            pub wins: usize,
            #[pyo3(get, set)]
            pub losses: usize,
            #[pyo3(get, set)]
            pub r#type: String,
            #[pyo3(get, set)]
            pub pct: String
        }
    
        #[pyclass]
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        pub struct RecordLeague {
            #[pyo3(get,set)]
            pub wins: usize,
            #[pyo3(get,set)]
            pub losses: usize,
            #[pyo3(get,set)]
            pub pct: String,
            #[pyo3(get,set)]
            pub league: LeagueGeneric,
        }
    
        #[pyclass]
        #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
        pub struct RecordDivision {
            #[pyo3(get,set)]
            pub wins: usize,
            #[pyo3(get,set)]
            pub losses: usize,
            #[pyo3(get,set)]
            pub pct: String,
            #[pyo3(get,set)]
            pub division: LeagueGeneric,
        }
        
    }


}
    
pub mod team {
    use pyo3::prelude::*;
    use serde::{Deserialize, Serialize};
    use super::{
        venue::Venue,
        generics::LeagueGeneric,
    };
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
        #[pyo3(get,set)]
        pub venue: Venue,
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
    use super::generics::{
        TeamGeneric, 
        records::RecordGeneric
    };

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ScheduleResponse {
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
        pub teams: Teams,
        #[pyo3(get, set)]
        pub tickets: Option<Vec<GameTicket>>,
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
        pub league_record: RecordGeneric,
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
    #[serde(rename_all = "camelCase")]
    pub struct GameTicket {
        #[pyo3(get, set)]
        pub ticket_type: String,
        #[pyo3(get, set)]
        pub ticket_links: Option<TicketLinks>,
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct TicketLinks {
        #[pyo3(get, set)]
        pub home: Option<String>,
        #[pyo3(get, set)]
        pub away: Option<String>,
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct NextGameInfo {

    }
}

pub mod standings {
    use pyo3::prelude::*;
    use serde::{Deserialize, Serialize};
    use super::generics::{LeagueCompact, TeamGeneric, StreakGeneric};
    use super::generics::records::{RecordType, RecordDivision, RecordLeague};

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StandingsResponse {
        #[pyo3(get,set)]
        pub records: Vec<Record>
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Record {
        #[pyo3(get,set)]
        pub standings_type: String,
        #[pyo3(get,set)]
        pub league: LeagueCompact,
        #[pyo3(get,set)]
        pub division: Option<LeagueCompact>,
        #[pyo3(get,set)]
        pub last_updated: String,
        #[pyo3(get,set)]
        pub team_records: Vec<TeamRecord>, 
        
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct TeamRecord {
        #[pyo3(get,set)]
        pub streak: StreakGeneric,
        #[pyo3(get,set)]
        pub team: TeamGeneric,
        #[pyo3(get,set)]
        pub games_played: usize,
        #[pyo3(get,set)]
        pub games_back: String,
        #[pyo3(get,set)]
        pub league_games_back: String,
        #[pyo3(get,set)]
        pub wild_card_games_back: String,
        #[pyo3(get,set)]
        pub runs_allowed: usize,
        #[pyo3(get,set)]
        pub runs_scored: usize,
        #[pyo3(get,set)]
        pub records: RecordSplits,
        #[pyo3(get,set)]
        pub league_record: StandingsRecord,
        #[pyo3(get,set)]
        pub wins: usize,
        #[pyo3(get,set)]
        pub losses: usize,
        #[pyo3(get,set)]
        pub winning_percentage: String,
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct RecordSplits {
        #[pyo3(get,set)]
        pub split_records: Vec<RecordType>,
        #[pyo3(get, set)]
        pub division_records: Option<Vec<RecordDivision>>,
        #[pyo3(get,set)]
        pub overall_records: Vec<RecordType>,
        #[pyo3(get, set)]
        pub league_records: Vec<RecordLeague>,
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct StandingsRecord {
        #[pyo3(get,set)]
        pub wins: usize,
        #[pyo3(get,set)]
        pub losses: usize,
        #[pyo3(get,set)]
        pub ties: usize,
        #[pyo3(get,set)]
        pub pct: String
    }

}

pub mod venue {
    use pyo3::prelude::*;
    use serde::{Deserialize,Serialize};

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct VenueGeneric {
        #[pyo3(get, set)]
        pub id: usize,
        #[pyo3(get, set)]
        pub name: Option<String>,
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct Venue {
        #[pyo3(get, set)]
        pub id: usize,
        #[pyo3(get, set)]
        pub name: Option<String>,
        #[pyo3(get, set)]
        pub link: Option<String>,
        #[pyo3(get, set)]
        pub location: VenueLocation,
        #[pyo3(get, set)]
        pub field_info: FieldInfo,
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct VenueLocation {
        #[pyo3(get, set)]
        pub address1: Option<String>,
        #[pyo3(get, set)]
        pub address2: Option<String>,
        #[pyo3(get, set)]
        pub state: Option<String>,
        #[pyo3(get, set)]
        pub state_abbrev: Option<String>,
        #[pyo3(get, set)]
        pub postal_code: Option<String>,
        #[pyo3(get, set)]
        pub country: Option<String>,
        #[pyo3(get, set)]
        pub phone: Option<String>,
        #[pyo3(get, set)]
        pub default_coordinates: Option<VenueCoords>,
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct FieldInfo {
        #[pyo3(get, set)]
        pub capacity: Option<usize>,
        #[pyo3(get, set)]
        pub turf_type: Option<String>,
        #[pyo3(get, set)]
        pub roof_type: Option<String>,
        #[pyo3(get, set)]
        pub left_line: Option<usize>,
        #[pyo3(get, set)]
        pub left_center: Option<usize>,
        #[pyo3(get, set)]
        pub center: Option<usize>,
        #[pyo3(get, set)]
        pub right_center: Option<usize>,
        #[pyo3(get, set)]
        pub right_line: Option<usize>,
    }

    #[pyclass]
    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct VenueCoords {
        #[pyo3(get, set)]
        pub latitude: f64,
        #[pyo3(get, set)]
        pub longitude: f64
    }

}