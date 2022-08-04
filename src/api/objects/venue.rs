#[allow(unused_imports)]
use {serde::{Serialize, Deserialize}};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Venue {
    pub id: usize,
    pub name: Option<String>,
    pub link: Option<String>,
    pub location: VenueLocation,
    pub field_info: FieldInfo,
    #[serde(rename = "timeZone")]
    pub timezone: TimeZone,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VenueLocation {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub state: Option<String>,
    pub state_abbrev: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub phone: Option<String>,
    pub default_coordinates: Option<VenueCoords>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FieldInfo {
    pub capacity: Option<usize>,
    pub turf_type: Option<String>,
    pub roof_type: Option<String>,
    pub left_line: Option<usize>,
    pub left_center: Option<usize>,
    pub center: Option<usize>,
    pub right_center: Option<usize>,
    pub right_line: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct VenueCoords {
    pub latitude: f64,
    pub longitude: f64
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VenueGeneric {
    pub id: usize,
    pub name: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TimeZone {
    pub id: String,
    pub offset: isize,
    pub tz: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpringVenue {
    pub id: usize,
    pub link: String,
}