
pub mod generics {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct TeamGeneric {
        pub id: usize,
        pub name: String,
        pub link: String,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MlbGeneric {
        pub id: usize,
        pub name: Option<String>,
        pub link: Option<String>
    }
}
    
pub mod team {
    use serde::{Deserialize, Serialize};
    
    #[derive(Deserialize, Serialize, Debug)]
    pub struct TeamResponse {
        pub copyright: String,
        pub teams: Vec<Team>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Team {
       pub id: u16,
       pub name: String,
       pub link: String,
       pub season: usize,
       pub venue: super::generics::MlbGeneric,
       pub league: LeagueGeneric,
       pub team_code: String,
       pub file_code: String,
       pub club_name: String,
       pub abbreviation: String,
       pub short_name: String,
       pub location_name: String,
       pub franchise_name: String,
       pub active: bool,
       pub first_year_of_play: String,
    }


    #[derive(Serialize, Deserialize, Debug)]
    
    pub struct LeagueGeneric {
        pub id: usize,
        pub name: String,
        pub link: String,
    }



}

pub mod games {
    use serde::{Deserialize, Serialize};

    pub mod schedule {
        use crate::schemas::generics::TeamGeneric;

        #[derive(super::Serialize, super::Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct ScheduleResponse {
           pub copyright: String,
           pub total_items: usize,
           pub total_events: usize,
           pub total_games: usize,
           pub total_games_in_progress: usize,
           pub dates: Vec<Date>,
    
        }

        #[derive(super::Serialize, super::Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Date {
           pub date: String,
           pub total_items: usize,
           pub total_events: usize,
           pub total_games: usize,
           pub total_games_in_progress: usize,
           pub games: Vec<Game>
        }

        #[derive(super::Serialize, super::Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Game {
           pub game_pk: usize,
           pub game_type: String,
           pub season: String,
           pub game_date: String,
           pub official_date: String,
           pub teams: Teams,

        }

        #[derive(super::Serialize, super::Deserialize, Debug)]
        pub struct Teams {
            pub away: Team,
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

        #[derive(super::Serialize, super::Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        pub struct Team {
            pub score: Option<usize>,
            pub league_record: LeagueRecord,
            pub is_winner: Option<bool>,
            pub split_squad: Option<bool>,
            pub series_number: Option<usize>,
            pub team: TeamGeneric,
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

        #[derive(super::Serialize, super::Deserialize, Debug)]
        pub struct LeagueRecord {
            pub wins: usize,
            pub losses: usize,
            pub pct: String,
        }
        

    }



    #[derive(Serialize, Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub struct Game {

    }
}