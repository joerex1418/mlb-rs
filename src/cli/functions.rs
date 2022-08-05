use colored::Colorize;
use reqwest;

use crate::{
    api,
    utils::BASE,
    functions::stat_abbrv,
};

type APIStatResponse = crate::api::objects::stats::StatsResponse;
type APIGameResponseGeneric = crate::api::objects::game::GameResponse;

pub fn person(mlb_id: Option<String>) {
    if let Some(mlb_id) = mlb_id {                
        let mlb_id = mlb_id.parse::<usize>();
        if let Ok(mlb_id) = mlb_id {
            let person = api::get_person(mlb_id);

            if let Some(person) = person {
                let mut status = String::from("Not Active").magenta();
                if let Some(is_active) = person.active {
                    if is_active { status = String::from("Active").green() }
                }
                println!("\n{:<8} {}\t{:<8} {}",
                    "MLB ID:".cyan().bold(),
                    person.id.to_string().yellow(),
                    "STATUS:".cyan().bold(),
                    status
                );
                println!(
                    "{:<8} {:<20} {} {:<3}",
                    "NAME:".cyan().bold(),
                    person.full_fml_name,
                    "POS:".cyan().bold(),
                    person.primary_position.expect("").abbreviation
                );
                println!("{:<8} {:<20} {:<8} {}",
                    "BATS:".cyan().bold(),
                    person.bat_side.description,
                    "THROWS:".cyan().bold(),
                    person.pitch_hand.description);
                println!(
                    "{:<8} {} in {}, {}",
                    "BORN:".cyan().bold(),
                    person.birth_date,
                    person.birth_city,
                    person.birth_country
                );
            }
        }
    }
}

pub fn game(mlb_id: Option<String>) {
    if let Some(mlb_id) = mlb_id {
        let mlb_id = mlb_id.parse::<usize>();
        if let Ok(mlb_id) = mlb_id {
            let url = format!(
                "https://statsapi.mlb.com/api/v1.1/game/{}/feed/live", mlb_id
            );

            let response = reqwest::blocking::get(url);
            if let Ok(response) = response {
                let game_resp: reqwest::Result<APIGameResponseGeneric> = response.json();
                if let Ok(game_resp) = game_resp {
                    println!("{}",game_resp);
                } else {
                    eprintln!("{:?}",game_resp);
                }
            } else {
                eprintln!("{:?}", response);
            }
        }
    }
}

pub fn person_stats(mlb_id: Option<String>, stat_group: String, stat_type: Option<String>) {
    let stat_type = match stat_type {
        Some(st) => { st },
        None => "yearByYear".to_string()
    };
    if let Some(mlb_id) = mlb_id {
        let query_path: String = format!(
            "group={group}&stats={stat_type}",
            group = stat_group,
            stat_type = stat_type
        );
        let url: String = format!(
            "{base}/api/v1/people/{person_id}/stats?{query_path}",
            base = BASE,
            person_id = mlb_id,
            query_path = query_path,
        );

        let response = reqwest::blocking::get(url);

        if let Ok(response) = response {
            let stats: reqwest::Result<APIStatResponse> = response.json();
            if let Ok(stats) = stats {
                println!("{:#?}", stats.get_pitching(false));
                // println!("{:#}",serde_json::to_string_pretty(&stats).unwrap());
            } else {
                println!("{:?}", stats)
            }
        }
    }
}

