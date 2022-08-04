#[allow(unused_imports)]
use {serde::{Serialize, Deserialize}};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub id: usize,
    pub name: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpringVenue {
    pub id: usize,
    pub link: String,
}