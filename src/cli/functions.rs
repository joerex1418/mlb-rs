use colored::Colorize;
use reqwest;

use crate::{
    api,
    utils::BASE,
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
                let mut divider = String::new();

                if stat_group.to_lowercase() == "hitting" {
                    let season = "Season".bright_yellow();
                    let header = format!(
                        "{:<8} | {:^4} {:^4} {:>4} {:^4} {:>4} {:^4} {:^4} {:^4} {:^4} {:>4} {:>4} {:>4}",
                        season,   "G", "AB", "RBI", "H", "AVG", "2B", "HR", "K", "BB", "OBP","SLG", "OPS"
                    );
                    for _ in 0..header.to_string().len() {
                        divider.push_str("-");
                    }
                    
                    let stats = stats.get_hitting(false);
                    
                    // Player Name
                    if let Some(split) = stats.get(0) {
                        if let Some(player) = &split.player {
                            println!(
                                "{:<25} {} {}",
                                player.full_name.bold(),
                                stat_group.to_uppercase().bold(),
                                "STATS".bold());
                        }
                    }
                    // Header & Divider
                    println!("{}\n{}",header,divider);
                    // Stats
                    for entry in stats {
                        let stat = entry.stat;
                        println!(
                        "{:<8} | {:^4} {:^4} {:^4} {:^4} {:^4} {:^4} {:^4} {:^4} {:^4} {:^4} {:^4} {:^4}",
                            entry.season.expect("-").cyan(),
                            stat.games_played,
                            stat.at_bats,
                            stat.rbi,
                            stat.hits,
                            stat.avg,
                            stat.doubles,
                            stat.home_runs,
                            stat.strike_outs,
                            stat.base_on_balls,
                            stat.obp,
                            stat.slg,
                            stat.ops,
                        );
                    }
                }

                if stat_group.to_lowercase() == "pitching" {
                    let season = "Season".bright_yellow();
                    let header = format!(
                        "{:<8} | {:<4} {:<4} {:<4} {:<5} {:<5} {:<4} {:<4} {:<4}",
                        season,   "G",  "W", "L", "ERA", "WHIP", "K",  "BB", "H"
                    );
                    for _ in 0..header.to_string().len() {
                        divider.push_str("-");
                    }
                    
                    let stats = stats.get_pitching(false);

                    // Player Name
                    if let Some(split) = stats.get(0) {
                        if let Some(player) = &split.player {
                            println!(
                                "{:<25} {} {}",
                                player.full_name.bold(),
                                stat_group.to_uppercase().bold(),
                                "STATS".bold());
                        }
                    }
                    // Header & Divider
                    println!("{}\n{}",header,divider);
                    // Stats
                    for entry in stats {
                        let stat = entry.stat;
                        println!(
                        "{:<8} | {:<4} {:<4} {:<4} {:<5} {:<5} {:<4} {:<4} {:<4}",
                            entry.season.expect("-").cyan(),
                            stat.games_played,
                            stat.wins,
                            stat.losses,
                            stat.era,
                            stat.whip,
                            stat.strike_outs,
                            stat.base_on_balls,
                            stat.hits
                        );
                    }
    
                }

                if stat_group.to_lowercase() == "fielding" {
                    let season = "Season".bright_yellow();
                    let header = format!(
                        "{:<8} | {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<5} {:<4} {:<4}",
                        season, "Pos".yellow(), "G", "PO", "2B", "3B", "A", "Ch", "thE", "E"
                    );
                    for _ in 0..header.to_string().len() {
                        divider.push_str("-");
                    }
                    
                    let stats = stats.get_fielding(false);

                    // Player Name
                    if let Some(split) = stats.get(0) {
                        if let Some(player) = &split.player {
                            println!(
                                "{:<25} {} {}",
                                player.full_name.bold(),
                                stat_group.to_uppercase().bold(),
                                "STATS".bold());
                        }
                    }
                    // Header & Divider
                    println!("{}\n{}",header,divider);
                    // Stats
                    for entry in stats {
                        let stat = entry.stat;
                        println!(
                        "{:<8} | {:<4} {:<4} {:<4} {:<4} {:<4} {:<4} {:<5} {:<4} {:<4}",
                            entry.season.expect("-").cyan(),
                            stat.position.abbreviation.yellow(),
                            stat.games_played,
                            stat.put_outs,
                            stat.double_plays,
                            stat.triple_plays,
                            stat.assists,
                            stat.chances.expect("-"),
                            stat.throwing_errors.expect("-"),
                            stat.errors,
                        );
                    }
    
                }


            } else {
                println!("{:?}", stats)
            }
        }
    }
}

