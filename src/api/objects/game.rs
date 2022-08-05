use std::fmt;
use serde::{Deserialize,Serialize};

#[derive(Deserialize,Serialize)]
#[serde(rename = "camelCase")]
pub struct GameResponse {

}