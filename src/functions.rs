use chrono::Datelike;
#[allow(unused)]
#[allow(dead_code)]

use pyo3::IntoPy;

use crate::{
    objects::people::{PersonResponse, Person},
    objects::schemas::{
        team::{TeamResponse,Team},
        standings::StandingsResponse,
        schedule::ScheduleResponse
    },
    objects::rosters::RosterResponse,
    utils::BASE,
};

#[allow(unused)]
pub fn get_next_game(team_id: usize) {
    let url: String = format!(
        "https://statsapi.mlb.com/api/v1/teams/{team_id}?hydrate={hydrations}",
        team_id = team_id.to_string(),
        hydrations = "nextSchedule(limit=1),game(tickets,atBatTickets)"
    );

    let response = reqwest::blocking::get(url);
}

#[allow(unused)]
pub fn get_team(team_id: usize) -> Option<Team> {
    let url: String = format!(
        "https://statsapi.mlb.com/api/v1/teams/{team_id}?hydrate={hydrations}",
        team_id = team_id.to_string(),
        hydrations = "venue(location,fieldInfo),nextSchedule(limit=1),game(tickets,atBatTickets)"
    );

    let response = reqwest::blocking::get(url);

    if let Ok(response) = response {
        let team_response: reqwest::Result<TeamResponse> = response.json();

        if let Ok(team_response) = team_response {
            let team = team_response.teams.get(0).unwrap();
            return Some(team.to_owned());
        };
    };

    None
}

#[allow(unused)]
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

#[allow(unused)]
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

#[allow(unused)]
pub fn get_schedule(date: Option<String>) -> Option<ScheduleResponse> {
    let hydrations: &str = "game(tickets,atBatTickets)";

    let url: String = match date {
        Some(date) => {
            format!(
                "https://statsapi.mlb.com/api/v1/schedule?sportId=1&date={}&hydrate={}",
                date.to_string(),
                hydrations
            )
        }
        None => {
            format!(
                "https://statsapi.mlb.com/api/v1/schedule?sportId=1&hydrate={}",
                hydrations
            )
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

#[allow(unused)]
pub fn get_league_standings(season:Option<usize>) {
    // pub fn get_league_standings(season:Option<usize>) -> Option<LeagueStandingsResponse> {
    // hydrate=league,division
    let season: String = match season {
        Some(season) => {
            season.to_string()
        }
        None => {
            chrono::Utc::now().year().to_string()
        }
    };

    let url: String = format!(
        "{base}/api/v1/standings?season={season}&leagueId=103,104&standingsType=byLeague&hydrate=league,division",
        base = BASE,
        season = season
    );

    let response = reqwest::blocking::get(url);

    if let Ok(resp) = response {
        let standings_json: reqwest::Result<StandingsResponse> = resp.json();
        if let Ok(standings_resp) = standings_json {
            println!("{:#?}",standings_resp.records);
        }
    }


}

#[allow(unused)]
pub fn get_division_standings(season:Option<usize>) {
    // pub fn get_league_standings(season:Option<usize>) -> Option<LeagueStandingsResponse> {
    // hydrate=league,division
    let season: String = match season {
        Some(season) => {
            season.to_string()
        }
        None => {
            chrono::Utc::now().year().to_string()
        }
    };

    let url: String = format!(
        "{base}/api/v1/standings?season={season}&leagueId=103,104&standingsType=byDivision&hydrate=league,division",
        base = BASE,
        season = season
    );

    let response = reqwest::blocking::get(url);

    if let Ok(resp) = response {
        let standings_json: reqwest::Result<StandingsResponse> = resp.json();
        if let Ok(standings_resp) = standings_json {
            println!("{:#?}",standings_resp.records);
        }
    }


}
