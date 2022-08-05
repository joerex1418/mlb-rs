mod utils;
mod functions;
mod objects;
mod stats;
mod api;

use std::vec;
use std::time::{Instant};
use colored::Colorize;
use reqwest;
use reqwest::Client;
use clap::Parser;
// use std::thread::sleep;

#[allow(unused_imports)]
use objects::schemas::{
    team::{TeamResponse,Team}
};

#[derive(Parser)]
struct Cli {
    function: String,
    param: Option<String>,
    arg3: Option<String>,
}

fn main() {
    let args = Cli::parse();

    println!("\n{}\n", args.arg3.expect("nothing to see here"));
    
    match args.function.as_str() {
        "person" | "player" => {
            if let Some(param) = args.param {                
                let param = param.parse::<usize>();
                if let Ok(param) = param { 
                    let person = api::get_person(param);
                    if let Some(person) = person {
                        println!("{} {}",
                            "MLB ID:".cyan().bold(),
                            person.id.to_string());
                        println!(
                            "{} {:<20} {} {:<3}",
                            "NAME:".cyan().bold(),
                            person.full_fml_name,
                            "POS:".cyan().bold(),
                            person.primary_position.expect("").abbreviation
                        );
                        println!("{} {}\t{} {}",
                            "BATS:".cyan().bold(),
                            person.bat_side.description,
                            "THROWS:".cyan().bold(),
                            person.pitch_hand.description);
                        println!(
                            "{} {} in {}, {}",
                            "BORN:".cyan().bold(),
                            person.birth_date,
                            person.birth_city,
                            person.birth_country
                        );
                        let mut status = String::from("Not Active").magenta();
                        if let Some(is_active) = person.active {
                            if is_active { status = String::from("Active").green() }
                        }
                        println!(
                            "{} {}",
                            "STATUS:".cyan().bold(),
                            status
                        )
                    }
                }
            } else {
                println!("ERROR: No \"person ID\" provided");
            }
        },
        "game" => {
        },
        "team" => {

        },
        _ => println!("ERROR: No function provided")
    };


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
