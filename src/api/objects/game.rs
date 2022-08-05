#[allow(unused)]
use colored::Colorize;
use std::fmt;
use serde::{Deserialize,Serialize};

type APIPersonGeneric = crate::api::objects::people::PersonGeneric;
type APITeamGeneric = crate::api::objects::team::TeamGeneric;
type APIVenueGeneric = crate::api::objects::venue::VenueGeneric;

#[derive(Deserialize,Serialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameResponse {
    // pub meta_data: MetaData,
    pub live_data: LiveData,
    pub game_data: GameData,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameData {
    pub datetime: Datetime,
    pub status: GameStatusDetails,
    pub venue: APIVenueGeneric,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiveData {
    pub linescore: Linescore,
    pub boxscore: Boxscore,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Linescore {
    pub current_inning: Option<usize>,
    pub current_inning_ordinal: Option<String>,
    pub inning_state: Option<String>,
    pub inning_half: Option<String>,
    pub defense: Option<Defense>,
    pub offense: Option<Offense>,
    pub teams: LinescoreTeams,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinescoreTeams {
    pub away: TeamLinescoreData,
    pub home: TeamLinescoreData,
}

#[allow(unused)]
impl Linescore {
    pub fn get_away_score(&self) -> usize {
        self.teams.away.runs.clone()
    }

    pub fn get_away_hits(&self) -> usize {
        self.teams.away.hits.clone()
    }

    pub fn get_away_errors(&self) -> usize {
        self.teams.away.errors.clone()
    }

    pub fn get_home_score(&self) -> usize {
        self.teams.home.runs.clone()
    }

    pub fn get_home_hits(&self) -> usize {
        self.teams.home.hits.clone()
    }

    pub fn get_home_errors(&self) -> usize {
        self.teams.home.errors.clone()
    }
}

#[derive(Deserialize,Serialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Boxscore {
    pub teams: BoxscoreTeams,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct BoxscoreTeams {
    pub away: TeamBoxscoreData,
    pub home: TeamBoxscoreData,
}

#[allow(unused)]
impl Boxscore {
    pub fn get_away(&self) -> APITeamGeneric {
        self.teams.away.team.clone()
    }

    pub fn get_home(&self) -> APITeamGeneric {
        self.teams.home.team.clone()
    }
}

#[derive(Deserialize,Serialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Defense {
    pub pitcher: Option<APIPersonGeneric>,
    pub catcher: Option<APIPersonGeneric>,
    pub first: Option<APIPersonGeneric>,
    pub second: Option<APIPersonGeneric>,
    pub third: Option<APIPersonGeneric>,
    pub shortstop: Option<APIPersonGeneric>,
    pub left: Option<APIPersonGeneric>,
    pub center: Option<APIPersonGeneric>,
    pub right: Option<APIPersonGeneric>,
    pub batter: Option<APIPersonGeneric>,
    pub on_deck: Option<APIPersonGeneric>,
    pub in_hole: Option<APIPersonGeneric>,
    pub batting_order: Option<usize>,
    pub team: Option<APITeamGeneric>,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Offense {
    pub batter: Option<APIPersonGeneric>,
    pub on_deck: Option<APIPersonGeneric>,
    pub in_hole: Option<APIPersonGeneric>,
    pub pitcher: Option<APIPersonGeneric>,
    pub batting_order: Option<usize>,
    pub team: Option<APITeamGeneric>,
}

impl fmt::Display for GameResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bs: Boxscore = self.live_data.boxscore.clone();
        let ls: Linescore = self.live_data.linescore.clone();

        let away: String = bs.get_away().name;
        let home: String = bs.get_home().name;
        let date: String = self.game_data.datetime.official_date.clone();

        let date = chrono::NaiveDate::parse_from_str(date.as_str(), "%Y-%m-%d");
        if let Ok(date) = date {
            let date = date.format("%B %e, %Y");
            println!("{}",date);
        } else {
            eprintln!("{:?}",date);
        }
        
        let header_display = format!("{:<22} | {:^3} {:^3} {:^3}","","R","H","E");
        let away_display = format!(
            "{:<22} | {:^3} {:^3} {:^3}",
            away,
            ls.get_away_score(),
            ls.get_away_hits(),
            ls.get_away_errors());
        let home_display = format!(
            "{:<22} | {:^3} {:^3} {:^3}",
            home,
            ls.get_home_score(),
            ls.get_home_hits(),
            ls.get_home_errors());

        let result: String = format!("{}\n{}\n{}", header_display, away_display, home_display);
        write!(f,"{}\n",result)
    }
}

#[derive(Deserialize,Serialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Datetime {
    pub date_time: Option<String>, // iso time format
    pub original_date: String, // yyyy-mm-dd
    pub official_date: String, // yyyy-mm-dd
    pub day_night: Option<String>,
    pub time: Option<String>,
    pub ampm: Option<String>,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameStatusDetails {
    pub abstract_game_state: String,
    pub abstract_game_code: String,
    pub coded_game_state: String,
    pub detailed_state: String,
    pub status_code: String,
    #[serde(rename = "startTimeTBD")]
    pub start_time_tbd: bool,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamBoxscoreData {
    pub team: APITeamGeneric,
}

#[derive(Deserialize,Serialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamLinescoreData {
    pub runs: usize,
    pub hits: usize,
    pub errors: usize,
    pub left_on_base: usize,
}