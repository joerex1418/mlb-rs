// use reqwest::Response;
// use reqwest::Error;
#[allow(unused)]
#[allow(dead_code)]

use pyo3::IntoPy;
use reqwest::Request;

use crate::people::{PersonResponse, Person};

use crate::schemas::team::TeamResponse;
use crate::{schemas::schedule::ScheduleResponse};
use crate::rosters::rosters::RosterResponse;

pub fn get_team(team_id: usize) {
    let url: String = format!(
        "https://statsapi.mlb.com/api/v1/teams/{team_id}",
        team_id = team_id.to_string()
    );

    let response = reqwest::blocking::get(url);

    if let Ok(response) = response {
        let json_resp: reqwest::Result<TeamResponse> = response.json();

        if let Ok(json_resp) = json_resp {
            println!("{:#?}", json_resp);
        }
    }
}

pub fn get_person(person_id: usize) -> Option<Person> {
    let url: String = format!(
        "https://statsapi.mlb.com/api/v1/people/{person_id}",
        person_id = person_id.to_string()
    );

    let response = reqwest::blocking::get(url);

    

    if let Ok(response) = response {
        let json_resp: reqwest::Result<PersonResponse> = response.json();

        if let Ok(person_resp_obj) = json_resp {
            if let Some(person_obj) = person_resp_obj.people.get(0) {
                return Some(person_obj.to_owned());
            };
        };
    };

    None



}

pub fn get_roster(team_id: usize) -> Option<RosterResponse> {
    
    let url: String = format!(
        "https://statsapi.mlb.com/api/v1/teams/{team_id}/roster",
        team_id = team_id.to_string()
    );

    let response = reqwest::blocking::get(url);

    if let Ok(response) = response {
        let json_resp: reqwest::Result<RosterResponse> = response.json();
        if let Ok(roster_obj) = json_resp {
            return Some(roster_obj)
        }

    }

    None

}

pub fn get_schedule(date: Option<String>) -> Option<ScheduleResponse> {

    let url = match date {
        Some(date) => {
            format!(
                "https://statsapi.mlb.com/api/v1/schedule?sportId=1&date={}",
                date.to_string()
            )
        }
        None => {
            "https://statsapi.mlb.com/api/v1/schedule?sportId=1".to_string()
        }
    };
    
    let response = reqwest::blocking::get(url);

    if let Ok(response) = response {
        
        let json_resp: reqwest::Result<ScheduleResponse> = response.json();
        if let Ok(json_resp) = json_resp {
            return Some(json_resp)
        } 
    } 

    return None
    

}
