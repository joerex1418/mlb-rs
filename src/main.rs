
mod utils;
mod functions;
mod objects;
mod stats;

use std::vec;
use std::time::{Instant};
use reqwest;
use reqwest::Client;
// use std::thread::sleep;

#[allow(unused_imports)]
use objects::schemas::{
    team::{TeamResponse,Team}
};

fn main() {
    // let team:Option<Team> = functions::get_team(145);
    // println!("{:#?}",team);
    // functions::get_league_standings(Some(2022));
    functions::get_division_standings(Some(2022));

}

#[allow(unused)]
#[tokio::main]
async fn multiple_requests() {
    let now = Instant::now();

    let team_id: u16 = 145;
    let team_id2: u16 = 112;

    let _url = format!(
        "https://statsapi.mlb.com/api/v1/teams/{team_id}",
        team_id = team_id.to_string());

    let _url2 = format!(
        "https://statsapi.mlb.com/api/v1/teams/{team_id}",
        team_id = team_id2.to_string());

    let mut teams: Vec<String> = Vec::new();
    let mut urls: Vec<String> = Vec::new();
    
    for id in vec![145,144,112,110,133,134,136,137,118,119] {
        let new_url: String = format!(
            "https://statsapi.mlb.com/api/v1/teams/{team_id}",
            team_id = id.to_string()
        );
        urls.push(new_url);
    }


    let client = Client::new();
    
    for u in urls {
        let req = client.get(u).build();

        if let Ok(req) = req {
            let response = client.execute(req).await;
    
            if let Ok(response) = response {
                let resp_obj: reqwest::Result<TeamResponse> = response.json().await;
                
                if let Ok(resp_obj) = resp_obj {
                    if let Some(team) = resp_obj.teams.get(0) {
                        let json_string = serde_json::to_string(team);
                        teams.push(json_string.unwrap());
                    }
                }
                
            }
        }
    }
    // sleep(Duration::new(2,0));
    println!("Completed in {}", now.elapsed().as_secs_f32());
    println!("\n{:#?}",teams.get(0).unwrap());


}
