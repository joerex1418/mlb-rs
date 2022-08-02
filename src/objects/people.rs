use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::{Datelike};

// use crate::stats;
use crate::utils::*;
use crate::objects::schemas::generics::{Position, Dexterity};
use crate::stats::stats::StatsResponse;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonResponse {
    pub people: Vec<Person>
}


#[pyclass]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Person {
    #[pyo3(get,set)]
    pub id: usize,
    #[pyo3(get,set)]
    pub full_name: String,
    #[pyo3(get,set)]
    pub link: String,
    #[pyo3(get,set)]
    pub first_name: String,
    #[pyo3(get,set)]
    pub last_name: String,
    #[pyo3(get,set)]
    pub primary_number: Option<String>,
    #[pyo3(get,set)]
    pub birth_date: String,
    #[pyo3(get,set)]
    pub current_age: usize,
    #[pyo3(get,set)]
    pub birth_city: Option<String>,
    #[pyo3(get,set)]
    pub birth_state_province: Option<String>,
    #[pyo3(get,set)]
    pub birth_country: Option<String>,
    #[pyo3(get,set)]
    pub height: String,
    #[pyo3(get,set)]
    pub weight: usize,
    #[pyo3(get,set)]
    pub active: Option<bool>,
    #[pyo3(get,set)]
    pub primary_position: Option<Position>,
    #[pyo3(get,set)]
    pub use_name: Option<String>,
    #[pyo3(get,set)]
    pub middle_name: Option<String>,
    #[pyo3(get,set)]
    pub boxscore_name: Option<String>,
    #[pyo3(get,set)]
    pub nick_name: Option<String>,
    #[pyo3(get,set)]
    pub gender: String,
    #[pyo3(get,set)]
    pub is_player: bool,
    #[pyo3(get,set)]
    pub is_verified: bool,
    #[pyo3(get,set)]
    pub draft_year: Option<usize>,
    #[pyo3(get,set)]
    pub pronunciation: String,
    #[pyo3(get,set)]
    pub mlb_debut_date: String,
    #[pyo3(get,set)]
    pub bat_side: Dexterity,
    #[pyo3(get,set)]
    pub pitch_hand: Dexterity,
    #[pyo3(get,set)]
    pub name_first_last: String,
    #[pyo3(get,set)]
    pub name_slug: String,
    #[pyo3(get,set)]
    pub first_last_name: String,
    #[pyo3(get,set)]
    pub last_first_name: String,
    #[pyo3(get,set)]
    pub last_init_name: String,
    #[pyo3(get,set)]
    pub init_last_name: String,
    #[pyo3(get,set)]
    #[serde(rename = "fullFMLName")]
    pub full_fml_name: String,
    #[pyo3(get,set)]
    #[serde(rename = "fullLFMName")]
    pub full_lfm_name: String,
    #[pyo3(get,set)]
    pub strike_zone_top: f64,
    #[pyo3(get,set)]
    pub strike_zone_bottom: f64,
}

#[pymethods]
impl Person {
    pub fn get_stats(
        &self,
        stat_type:&str,
        season:Option<usize>,
        groups:Option<Vec<String>>) -> Option<StatsResponse> {

        let group_param: String = match groups {
            Some(groups) => {
                groups.join(",")
            }
            None => {
                String::from("hitting,pitching,fielding")
            }
        };

        let query_path: String = match season {
            Some(season) => {
                format!(
                    "season={season}&stats={stat_type}&group={group_param}",
                    season = season.to_string(),
                    stat_type = stat_type,
                    group_param = group_param
                )
            }
            None => {
                format!(
                    "season={season}&stats={stat_type}&group={group_param}",
                    season = chrono::Utc::now().year(),
                    stat_type = stat_type,
                    group_param = group_param,
                )
            }
        };

        let url: String = format!(
            "{base}/api/v1/people/{person_id}/stats?{query_path}",
            base = BASE,
            person_id = self.id.to_string(),
            query_path = query_path,
        );

        let response = reqwest::blocking::get(url);
        
        if let Ok(response) = response {
            let stat_resp: reqwest::Result<StatsResponse> = response.json();
            if let Ok(stat_resp) = stat_resp {
                return Some(stat_resp)
            };
        };

        None

    }   
}

#[pyclass]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonGeneric {
    #[pyo3(get,set)]
    pub id: usize,
    #[pyo3(get,set)]
    pub full_name: String,
    #[pyo3(get,set)]
    pub link: String

}