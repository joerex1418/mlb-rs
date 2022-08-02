use pyo3::prelude::*;
use serde::{Deserialize,Serialize};

use crate::schemas::generics::Position;

#[pyclass]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GenericStats {
    #[pyo3(get,set)]
    pub games_played: usize,

    #[pyo3(get,set)]
    pub games_started: Option<usize>,

    #[pyo3(get,set)]
    pub runs: Option<usize>,

    #[pyo3(get,set)]
    pub home_runs: Option<usize>,

    #[pyo3(get,set)]
    pub strike_outs: Option<usize>,

    #[pyo3(get,set)]
    pub base_on_balls: Option<usize>,

    #[pyo3(get,set)]
    pub hits: Option<usize>,

    #[pyo3(get,set)]
    pub hit_by_pitch: Option<usize>,

    #[pyo3(get,set)]
    pub avg: Option<String>,

    #[pyo3(get,set)]
    pub at_bats: Option<usize>,

    #[pyo3(get,set)]
    pub obp: Option<String>,

    #[pyo3(get,set)]
    pub era: Option<String>,

    #[pyo3(get,set)]
    pub innings_pitched: Option<String>,

    #[pyo3(get,set)]
    pub wins: Option<usize>,

    #[pyo3(get,set)]
    pub losses: Option<usize>,

    #[pyo3(get,set)]
    pub saves: Option<usize>,

    #[pyo3(get,set)]
    pub earned_runs: Option<usize>,

    #[pyo3(get,set)]
    pub whip: Option<String>,

    #[pyo3(get,set)]
    pub batters_faced: Option<usize>,

    #[pyo3(get,set)]
    pub outs: Option<usize>,

    #[pyo3(get,set)]
    pub games_pitched: Option<usize>,

    #[pyo3(get,set)]
    pub complete_games: Option<usize>,

    #[pyo3(get,set)]
    pub shutouts: Option<usize>,

    #[pyo3(get,set)]
    pub hit_batsmen: Option<usize>,

    #[pyo3(get,set)]
    pub balks: Option<usize>,

    #[pyo3(get,set)]
    pub wild_pitches: Option<usize>,

    #[pyo3(get,set)]
    pub win_percentage: Option<String>,

    #[pyo3(get,set)]
    pub games_finished: Option<usize>,

    #[pyo3(get,set)]
    pub strikeout_walk_ratio: Option<String>,

    #[pyo3(get,set)]
    #[serde(rename = "strikeoutsPer9Inn")]
    pub strikeouts_per9inn: Option<String>,

    #[pyo3(get,set)]
    #[serde(rename = "walksPer9Inn")]
    pub walks_per9inn: Option<String>,

    #[pyo3(get,set)]
    #[serde(rename = "hitsPer9Inn")]
    pub hits_per9inn: Option<String>,

    #[pyo3(get,set)]
    pub runs_scored_per9: Option<String>,

    #[pyo3(get,set)]
    pub home_runs_per9: Option<String>,

    #[pyo3(get,set)]
    pub ground_outs: Option<usize>,

    #[pyo3(get,set)]
    pub air_outs: Option<usize>,

    #[pyo3(get,set)]
    pub doubles: Option<usize>,

    #[pyo3(get,set)]
    pub triples: Option<usize>,

    #[pyo3(get,set)]
    pub intentional_walks: Option<usize>,

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
    pub save_opportunities: Option<usize>,

    #[pyo3(get,set)]
    pub holds: Option<usize>,

    #[pyo3(get,set)]
    pub blown_saves: Option<usize>,

    #[pyo3(get,set)]
    pub strikes: Option<usize>,

    #[pyo3(get,set)]
    pub strike_percentage: Option<String>,

    #[pyo3(get,set)]
    pub pickoffs: Option<usize>,

    #[pyo3(get,set)]
    pub total_bases: Option<usize>,

    #[pyo3(get,set)]
    pub ground_outs_to_airouts: Option<String>,

    #[pyo3(get,set)]
    pub pitches_per_inning: Option<String>,

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

    #[pyo3(get,set)]
    pub plate_appearances: Option<usize>,

    #[pyo3(get,set)]
    pub rbi: Option<usize>,

    #[pyo3(get,set)]
    pub left_on_base: Option<usize>,

    #[pyo3(get,set)]
    pub babip: Option<String>,

    #[pyo3(get,set)]
    pub at_bats_per_home_run: Option<String>,

    #[pyo3(get,set)]
    pub assists: Option<usize>,

    #[pyo3(get,set)]
    pub put_outs: Option<usize>,

    #[pyo3(get,set)]
    pub errors: Option<usize>,

    #[pyo3(get,set)]
    pub chances: Option<usize>,

    #[pyo3(get,set)]
    pub fielding: Option<String>,

    #[pyo3(get,set)]
    pub position: Option<Position>,

    #[pyo3(get,set)]
    pub range_factor_per_game: Option<String>,

    #[pyo3(get,set)]
    #[serde(rename = "rangeFactorPer9Inn")]
    pub range_factor_per9inn: Option<String>,

    #[pyo3(get,set)]
    pub innings: Option<String>,

    #[pyo3(get,set)]
    pub games: Option<usize>,

    #[pyo3(get,set)]
    pub double_plays: Option<usize>,

    #[pyo3(get,set)]
    pub triple_plays: Option<usize>,

    #[pyo3(get,set)]
    pub throwing_errors: Option<usize>,
}
