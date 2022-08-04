use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
#[serde(rename_all = "camelCase")]
pub struct PersonGeneric {
    pub id: usize,
    pub full_name: String
}