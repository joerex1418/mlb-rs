mod utils;
mod functions;
mod objects;
mod stats;
mod api;
mod cli;

use std::vec;
use std::time::{Instant};
// use colored::Colorize;
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
    mlb_id: Option<String>,
    
    #[clap(short,long)]
    group: Option<String>,
    #[clap(long)]
    search: Option<String>,
    #[clap(short,long)]
    season: Option<String>,
    #[clap(long)]
    stats: Option<String>,
    #[clap(short = 't', long = "type")]
    stattype: Option<String>,
}

fn main() {
    let args: Cli = Cli::parse();

    // println!("{:<20} {:?}", "FUNCTION".yellow().bold(), &args.function);
    // println!("{:<20} {:?}", "MLB ID".yellow().bold(), &args.mlb_id.clone());
    // println!("{:<20} {:?}", "GROUP -g, --group".magenta(), &args.group.clone());
    // println!("{:<20} {:?}", "SEARCH --search".magenta(), &args.search.clone());
    // println!("{:<20} {:?}", "SEASON -s, --season".magenta(), &args.season.clone());
    // println!("{:<20} {:?}", "STATS --stats".magenta(), &args.stats.clone());
    // println!("{:<20} {:?}", "STAT TYPE --stattype".magenta(), &args.stats.clone());
    // println!("");

    let i_feel_like_it: bool = true;

    if i_feel_like_it {
        match args.function.as_str() {
            "person" | "player" => {
                if let Some(search) = args.search {
                    println!("Searching for: \"{}\"", search);
                }
                else if let Some(stat_group) = args.stats {
                    crate::cli::functions::person_stats(args.mlb_id, stat_group, args.stattype);
                } else {
                    crate::cli::functions::person(args.mlb_id);
                }
            },
            "game" => {
                crate::cli::functions::game(args.mlb_id);
            },
            "team" => {
    
            },
            _ => println!("ERROR: No function provided")
        };
    }

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
