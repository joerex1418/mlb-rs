mod objects;
mod url;
#[allow(unused_imports)]
use {
    reqwest,
    chrono,
    serde::{Serialize, Deserialize},
    serde_json::{Value}
};

use crate::api::url::BASE;
use crate::api::objects::{
    team,
};


#[allow(unused)]
pub fn get_teams() {
    let url: String = format!(
        "{base}/api/v1/teams?sportId=1", base = BASE
    );

    let response = reqwest::blocking::get(url);

    if let Ok(response) = response {
        let teams: reqwest::Result<team::TeamsResponse> = response.json();
        if let Ok(teams) = teams {
            println!("{:#?}", teams);
        }
    }
}

pub mod rosters {

    use crate::api::url::BASE;
    use crate::api::objects::{
        roster::RosterResponse
    };

    #[allow(unused)]
    pub fn get_active(team_id:usize) {
        let url: String = format!(
            "{base}/api/v1/teams/{team_id}/roster",
            base = BASE,
            team_id = team_id.to_string()
        );
    }

    #[allow(unused)]
    pub fn get_forty_man(team_id:usize) {
        let url: String = format!(
            "{base}/api/v1/teams/{team_id}/roster?rosterType=40Man",
            base = BASE,
            team_id = team_id.to_string()
        );

        if let Ok(response) = reqwest::blocking::get(url) {
            let roster_response: reqwest::Result<RosterResponse> = response.json();
            if let Ok(roster_response) = roster_response {
                println!("{}", roster_response);
            } else {
                println!("{:#?}", roster_response);
            }
        }
    }

    #[allow(unused)]
    pub fn get_full_season(team_id:usize) {

    }

}