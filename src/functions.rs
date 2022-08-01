// use reqwest;
// use std:: vec;
#[allow(unused)]


pub mod team {
    use crate::TeamResponse;

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

        };
    }
}

pub mod schedule {

    use crate::schemas::games::schedule::ScheduleResponse;

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
}