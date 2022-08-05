pub mod objects;
pub mod url;
// #[allow(unused_imports)]
use {
    reqwest,
    // chrono,
    // serde::{Serialize, Deserialize},
    // serde_json::{Value},
    futures::{
        // stream,
        // StreamExt,
        future::join_all}
};
// use chrono::Datelike;
use reqwest::{Client};
use tokio::task::JoinHandle;

use crate::api::url::BASE;
use crate::api::objects::{
    team,
    people::{PersonResponse, Person}
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
pub fn get_person(person_id: usize) -> Option<Person> {
    let url: String = format!(
        "{base}/api/v1/people/{person_id}?hydrate={hydrations}",
        base = BASE,
        person_id = person_id.to_string(),
        hydrations = "education,rosterEntries,currentTeam,transactions"
    );

    let response: Result<reqwest::blocking::Response,reqwest::Error> = reqwest::blocking::get(url);

    if let Ok(response) = response {
        let person_resp: reqwest::Result<PersonResponse> = response.json();
        if let Ok(person_resp) = person_resp {
            let person = person_resp.people.get(0);
            if let Some(person) = person {
                return Some(person.to_owned());
            }
        } else {
            // error with deserialization of json
            println!("{:#?}", person_resp)
        }
    }

    None
}

#[allow(unused)]
#[tokio::main]
pub async fn get_team_yby_standings(team_id:usize) {
    let now = std::time::Instant::now();

    let mut urls: Vec<String> = Vec::new();
    for year in 1901..=2022 {
        let url: String = format!(
            "{base}/api/v1/teams/{team_id}?hydrate=standings&season={year}",
            base = BASE,
            team_id = team_id,
            year = year
        );
        // println!("{}",&url);
        urls.push(url);
    }
    let client: Client = Client::new();

    let mut tasks: Vec<JoinHandle<Result<(),()>>> = Vec::new();
    
    for url in urls {
        let url = url.clone();

        tasks.push(tokio::spawn(async move {
            match reqwest::get(&url).await {
                Ok(resp) => {
                    match resp.text().await {
                        Ok(text) => {
                            println!("RESPONSE: {} bytes from {}",text.len(),url);
                        }
                        Err(_) => println!("ERROR reading {}", url),
                    }
                }
                Err(_) => println!("ERROR downloading {}", url)
            }
            Ok(())
        }));
    }

    println!("Started {} tasks. Waiting...", tasks.len());
    join_all(tasks).await;
    println!("Completed in {}", now.elapsed().as_millis());


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
        roster::RosterResponse,
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