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

    use serde_json::json;

    use crate::schemas::games::schedule::ScheduleResponse;

    pub fn get_schedule(date: Option<String>) {

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

        println!("{}",url);

        // let url = "https://statsapi.mlb.com/api/v1/schedule?sportId=1";
        
        let response = reqwest::blocking::get(url);

        if let Ok(response) = response {
            
            let json_resp: reqwest::Result<ScheduleResponse> = response.json();
            if let Ok(json_resp) = json_resp {
                let today = json_resp.dates.get(0);
                if let Some(today) = today {

                    // all unwrapped
                    for gm in &today.games {
                        println!("{:<25} {:^4}{:^4}{:^4}","","R","H","E");
                        println!(
                            "{:<25} {:^4}{:^4}{:^4}",
                            gm.teams.away.get_name(),
                            gm.teams.away.get_score(),
                            "",
                            "",
                        );
                        println!(
                            "{:<25} {:^4}{:^4}{:^4}",
                            gm.teams.home.get_name(),
                            gm.teams.home.get_score(),
                            "",
                            "",
                        );
                        println!("\n");
                    }


                }
            }
        }

    }
}