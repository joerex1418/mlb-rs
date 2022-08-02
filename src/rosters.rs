
// pub mod rosters {
    use pyo3::prelude::*;
    use serde::{Deserialize, Serialize};
    
    use crate::people::PersonGeneric;
    use crate::schemas::generics::{Position, PersonStatus};
    
    #[pyclass]
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct RosterResponse {
        #[pyo3(get, set)]
        pub copyright: String,
        #[pyo3(get, set)]
        pub roster: Vec<RosterEntry>
    }
    
    #[pyclass]
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct RosterEntry {
        #[pyo3(get, set)]
        pub person: PersonGeneric,
        #[pyo3(get, set)]
        pub jersey_number: Option<String>,
        #[pyo3(get, set)]
        pub position: Option<Position>,
        #[pyo3(get,set)]
        pub status: PersonStatus,
        #[pyo3(get,set)]
        pub parent_team_id: usize
    }
// }
