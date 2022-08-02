use pyo3::prelude::*;
use serde::{Deserialize,Serialize};

use crate::schemas::{
    generics::{Position, TeamGeneric}, 
    team::LeagueGeneric
};

use super::generic::GenericStats;

#[pyclass]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatsResponse {
    #[pyo3(get,set)]
    pub stats: Vec<StatEntry>
}

#[pyclass]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatEntry {
    #[serde(rename = "type")]
    _type: GroupType,
    #[serde(rename = "group")]
    _group: GroupType,
    #[pyo3(get,set)]
    splits: Vec<Split>,

}

#[pymethods]
impl StatEntry {
    pub fn get_group(&self) -> String {
        self._group.display_name.clone()
    }

    pub fn get_type(&self) -> String {
        self._type.display_name.clone()
    }
}

#[pyclass]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Split {
    #[pyo3(get,set)]
    pub season: String,
    #[pyo3(get,set)]
    pub stat: GenericStats,
    #[pyo3(get,set)]
    pub team: TeamGeneric,
    #[pyo3(get,set)]
    pub game_type: Option<String>,
    #[pyo3(get,set)]
    pub league: LeagueGeneric,
    #[pyo3(get,set)]
    pub position: Option<Position>,

}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupType {
    pub display_name: String
}

pub mod types {
    use pyo3::prelude::*;
    use serde::{Deserialize, Serialize};
    use crate::schemas::generics::Position;

    #[pyclass]
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct HittingStats {
        #[pyo3(get,set)]
        pub games_played: usize,
        #[pyo3(get,set)]
        pub runs: usize,
        #[pyo3(get,set)]
        pub doubles: usize,
        #[pyo3(get,set)]
        pub triples: usize,
        #[pyo3(get,set)]
        pub home_runs: usize,
        #[pyo3(get,set)]
        pub strike_outs: usize,
        #[pyo3(get,set)]
        pub base_on_balls: usize,
        #[pyo3(get,set)]
        pub hits: usize,
        #[pyo3(get,set)]
        pub hit_by_pitch: usize,
        #[pyo3(get,set)]
        pub avg: String,
        #[pyo3(get,set)]
        pub at_bats: usize,
        #[pyo3(get,set)]
        pub obp: String,
        #[pyo3(get,set)]
        pub slg: String,
        #[pyo3(get,set)]
        pub ops: String,
        #[pyo3(get,set)]
        pub caught_stealing: usize,
        #[pyo3(get,set)]
        pub stolen_bases: usize,
        #[pyo3(get,set)]
        pub stolen_base_percentage: String,
        #[pyo3(get,set)]
        pub plate_appearances: usize,
        #[pyo3(get,set)]
        pub total_bases: usize,
        #[pyo3(get,set)]
        pub rbi: usize,
        #[pyo3(get,set)]
        pub sac_bunts: usize,
        #[pyo3(get,set)]
        pub babip: String,
        #[pyo3(get,set)]
        pub at_bats_per_home_run: String,
        #[pyo3(get,set)]
        pub ground_outs: Option<usize>,
        #[pyo3(get,set)]
        pub air_outs: Option<usize>,
        #[pyo3(get,set)]
        pub intentional_walks: Option<usize>,
        #[pyo3(get,set)]
        pub ground_into_double_play: Option<usize>,
        #[pyo3(get,set)]
        pub number_of_pitches: Option<usize>,
        #[pyo3(get,set)]
        pub left_on_base: Option<usize>,
        #[pyo3(get,set)]
        pub sac_flies: Option<usize>,
        #[pyo3(get,set)]
        pub ground_outs_to_airouts: Option<String>,
        #[pyo3(get,set)]
        pub catchers_interference: Option<usize>,
    }

    #[pyclass]
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct PitchingStats {
        #[pyo3(get,set)]
        pub games_played: usize,
        #[pyo3(get,set)]
        pub games_started: usize,
        #[pyo3(get,set)]
        pub ground_outs: Option<usize>,
        #[pyo3(get,set)]
        pub air_outs: Option<usize>,
        #[pyo3(get,set)]
        pub runs: usize,
        #[pyo3(get,set)]
        pub doubles: Option<usize>,
        #[pyo3(get,set)]
        pub triples: Option<usize>,
        #[pyo3(get,set)]
        pub home_runs: usize,
        #[pyo3(get,set)]
        pub strike_outs: usize,
        #[pyo3(get,set)]
        pub base_on_balls: usize,
        #[pyo3(get,set)]
        pub intentional_walks: Option<usize>,
        #[pyo3(get,set)]
        pub hits: usize,
        #[pyo3(get,set)]
        pub hit_by_pitch: usize,
        #[pyo3(get,set)]
        pub avg: String,
        #[pyo3(get,set)]
        pub at_bats: usize,
        #[pyo3(get,set)]
        pub obp: String,
        #[pyo3(get,set)]
        pub slg: Option<String>,
        #[pyo3(get,set)]
        pub ops: Option<String>,
        #[pyo3(get,set)]
        pub caught_stealing: Option<usize>,
        #[pyo3(get,set)]
        pub stolen_bases: Option<usize>,
        #[pyo3(get,set)]
        pub stolen_base_percentage: Option<String>,
        #[pyo3(get,set)]
        pub ground_into_double_play: Option<usize>,
        #[pyo3(get,set)]
        pub number_of_pitches: Option<usize>,
        #[pyo3(get,set)]
        pub era: String,
        #[pyo3(get,set)]
        pub innings_pitched: String,
        #[pyo3(get,set)]
        pub wins: usize,
        #[pyo3(get,set)]
        pub losses: usize,
        #[pyo3(get,set)]
        pub saves: usize,
        #[pyo3(get,set)]
        pub save_opportunities: Option<usize>,
        #[pyo3(get,set)]
        pub holds: Option<usize>,
        #[pyo3(get,set)]
        pub blown_saves: Option<usize>,
        #[pyo3(get,set)]
        pub earned_runs: usize,
        #[pyo3(get,set)]
        pub whip: String,
        #[pyo3(get,set)]
        pub batters_faced: usize,
        #[pyo3(get,set)]
        pub outs: usize,
        #[pyo3(get,set)]
        pub games_pitched: usize,
        #[pyo3(get,set)]
        pub complete_games: usize,
        #[pyo3(get,set)]
        pub shutouts: usize,
        #[pyo3(get,set)]
        pub strikes: Option<usize>,
        #[pyo3(get,set)]
        pub strike_percentage: Option<String>,
        #[pyo3(get,set)]
        pub hit_batsmen: usize,
        #[pyo3(get,set)]
        pub balks: usize,
        #[pyo3(get,set)]
        pub wild_pitches: usize,
        #[pyo3(get,set)]
        pub pickoffs: Option<usize>,
        #[pyo3(get,set)]
        pub total_bases: Option<usize>,
        #[pyo3(get,set)]
        pub ground_outs_to_airouts: Option<String>,
        #[pyo3(get,set)]
        pub win_percentage: String,
        #[pyo3(get,set)]
        pub pitches_per_inning: Option<String>,
        #[pyo3(get,set)]
        pub games_finished: usize,
        #[pyo3(get,set)]
        pub strikeout_walk_ratio: String,
        #[pyo3(get,set)]
        #[serde(rename = "strikeoutsPer9Inn")]
        pub strikeouts_per9inn: String,
        #[pyo3(get,set)]
        #[serde(rename = "walksPer9Inn")]
        pub walks_per9inn: String,
        #[pyo3(get,set)]
        #[serde(rename = "hitsPer9Inn")]
        pub hits_per9inn: String,
        #[pyo3(get,set)]
        pub runs_scored_per9: String,
        #[pyo3(get,set)]
        pub home_runs_per9: String,
        #[pyo3(get,set)]
        pub inherited_runners: Option<usize>,
        #[pyo3(get,set)]
        pub inherited_runners_scored: Option<usize>,
        #[pyo3(get,set)]
        pub catchers_interference: Option<usize>,
        #[pyo3(get,set)]
        pub sac_bunts: Option<usize>,
        #[pyo3(get,set)]
        pub sac_flies: Option<usize>,
    }

    #[pyclass]
    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct FieldingStats {
        #[pyo3(get, set)]
        pub games_played: usize,
        #[pyo3(get, set)]
        pub games_started: usize,
        #[pyo3(get, set)]
        pub assists: usize,
        #[pyo3(get, set)]
        pub put_outs: usize,
        #[pyo3(get, set)]
        pub errors: usize,
        #[pyo3(get, set)]
        pub chances: Option<usize>,
        #[pyo3(get, set)]
        pub fielding: String,
        #[pyo3(get, set)]
        pub position: Position,
        #[pyo3(get, set)]
        pub range_factor_per_game: String,
        #[pyo3(get, set)]
        #[serde(rename = "rangeFactorPer9Inn")]
        pub range_factor_per9inn: String,
        #[pyo3(get, set)]
        pub innings: String,
        #[pyo3(get, set)]
        pub games: usize,
        #[pyo3(get, set)]
        pub double_plays: usize,
        #[pyo3(get, set)]
        pub triple_plays: usize,
        #[pyo3(get, set)]
        pub throwing_errors: Option<usize>,
    }


}