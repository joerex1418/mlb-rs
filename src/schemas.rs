
pub mod generics {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MlbGeneric {
        id: usize,
        name: Option<String>,
        link: Option<String>
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
        id: usize,
        name: String,
        link: String,
    }



}
