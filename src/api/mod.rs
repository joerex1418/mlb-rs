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

#[allow(unused)]
pub fn get_person(person_id:usize) {

}



/// # rosters
/// Module with various functions to access team roster data
/// 
/// ## Examples
/// ```rust
/// let roster = get_forty_man(team_id: 145)
/// ```
pub mod rosters {
    use crate::api::url::BASE;
    use crate::api::objects::{
        roster::RosterResponse
    };

    /// Gets roster data with more customized url query parameters
    #[allow(unused)]
    pub fn get_roster(team_id:usize,roster_type:&str,season:usize) -> Option<RosterResponse> {
        let url: String = format!(
            "{base}/api/v1/teams/{team_id}/roster?rosterType={roster_type}&season={season}",
            base = BASE,
            team_id = team_id.to_string(),
            roster_type = roster_type,
            season = season.to_string()
        );

        println!("{}",&url);

        if let Ok(response) = reqwest::blocking::get(url) {
            let roster_response: reqwest::Result<RosterResponse> = response.json();
            if let Ok(roster_response) = roster_response {
                return Some(roster_response);
            } else {
                println!("{:#?}", &roster_response);
            }
        }
    None
    }

    #[allow(unused)]
    pub fn get_active(team_id:usize) -> Option<RosterResponse> {
        let url: String = format!(
            "{base}/api/v1/teams/{team_id}/roster",
            base = BASE,
            team_id = team_id.to_string()
        );

        if let Ok(response) = reqwest::blocking::get(url) {
            let roster_response: reqwest::Result<RosterResponse> = response.json();
            if let Ok(roster_response) = roster_response {
                return Some(roster_response);
            } else {
                println!("{:#?}", roster_response);
            }
        }
    None
    }

    #[allow(unused)]
    pub fn get_forty_man(team_id:usize) -> Option<RosterResponse> {
        let url: String = format!(
            "{base}/api/v1/teams/{team_id}/roster?rosterType=40Man",
            base = BASE,
            team_id = team_id.to_string()
        );

        if let Ok(response) = reqwest::blocking::get(url) {
            let roster_response: reqwest::Result<RosterResponse> = response.json();
            if let Ok(roster_response) = roster_response {
                return Some(roster_response);
            } else {
                println!("{:#?}", roster_response);
            }
        }
    None
    }

    #[allow(unused)]
    pub fn get_full_season(team_id:usize) -> Option<RosterResponse> {
        let url: String = format!(
            "{base}/api/v1/teams/{team_id}/roster?rosterType=fullSeason",
            base = BASE,
            team_id = team_id.to_string()
        );

        if let Ok(response) = reqwest::blocking::get(url) {
            let roster_response: reqwest::Result<RosterResponse> = response.json();
            if let Ok(roster_response) = roster_response {
                return Some(roster_response);
            } else {
                println!("{:#?}", roster_response);
            }
        }
    None
    }

    #[allow(unused)]
    pub fn get_full_roster(team_id:usize) -> Option<RosterResponse> {
        let url: String = format!(
            "{base}/api/v1/teams/{team_id}/roster?rosterType=fullRoster",
            base = BASE,
            team_id = team_id.to_string()
        );

        if let Ok(response) = reqwest::blocking::get(url) {
            let roster_response: reqwest::Result<RosterResponse> = response.json();
            if let Ok(roster_response) = roster_response {
                return Some(roster_response);
            } else {
                println!("{:#?}", roster_response);
            }
        }
    None
    }

    #[allow(unused)]
    pub fn get_all_time(team_id:usize) -> Option<RosterResponse> {
        let url: String = format!(
            "{base}/api/v1/teams/{team_id}/roster?rosterType=allTime",
            base = BASE,
            team_id = team_id.to_string()
        );

        if let Ok(response) = reqwest::blocking::get(url) {
            let roster_response: reqwest::Result<RosterResponse> = response.json();
            if let Ok(roster_response) = roster_response {
                return Some(roster_response);
            } else {
                println!("{:#?}", roster_response);
            }
        }
    None
    }

}