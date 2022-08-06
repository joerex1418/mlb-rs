use serde::{Deserialize,Serialize};

use crate::objects::schemas::{
    generics::{Position, TeamGeneric,LeagueGeneric}
};

use super::people::PersonGeneric;

type GenericStats = generic::GenericStats;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatsResponse {
    pub stats: Vec<StatEntry>
}

#[allow(unused)]
impl StatsResponse {
    
    pub fn get_hitting(&self, get_advanced: bool) -> Vec<HittingSplits> {
        let mut splits_vec: Vec<HittingSplits> = Vec::new();
        for stat in self.stats.iter() {
            if stat.get_group() == "hitting" {
                for split in stat.splits.iter() {
                    let string = serde_json::to_string(split);
                    if let Ok(string) = string {
                        let splits: serde_json::Result<HittingSplits> = serde_json::from_str(string.as_str());
                        if let Ok(splits) = splits {
                            splits_vec.push(splits);
                        } else {
                            println!("ERROR converting string to HittingSplits object: {:?}", splits);
                        }
                    } else {
                        println!("ERROR converting object to string: {:?}", string);
                    }
                
                }

            }
        }
        splits_vec.sort_by(|lhs,rhs| rhs.season.cmp(&lhs.season));
        splits_vec
    }

    pub fn get_pitching(&self, get_advanced: bool) -> Vec<PitchingSplits> {
        let mut splits_vec: Vec<PitchingSplits> = Vec::new();
        
        for stat in self.stats.iter() {
            if stat.get_group() == "pitching" {
                for split in stat.splits.iter() {
                    let string = serde_json::to_string(split);
                    if let Ok(string) = string {
                        let splits: serde_json::Result<PitchingSplits> = serde_json::from_str(string.as_str());
                        if let Ok(splits) = splits {
                            splits_vec.push(splits);
                        } else {
                            println!("ERROR converting string to PitchingSplits object: {:?}", splits);
                        }
                    } else {
                        println!("ERROR converting object to string: {:?}", string);
                    }
                
                }

            }
        }
        
        splits_vec.sort_by(|lhs,rhs| rhs.season.cmp(&lhs.season));
        splits_vec
    }

    pub fn get_fielding(&self, get_advanced: bool) -> Vec<FieldingSplits> {
        let mut splits_vec: Vec<FieldingSplits> = Vec::new();
        
        for stat in self.stats.iter() {
            if stat.get_group() == "fielding" {
                for split in stat.splits.iter() {
                    let string = serde_json::to_string(split);
                    if let Ok(string) = string {
                        let splits: serde_json::Result<FieldingSplits> = serde_json::from_str(string.as_str());
                        if let Ok(splits) = splits {
                            splits_vec.push(splits);
                        } else {
                            println!("ERROR converting string to FieldingSplits object: {:?}", splits);
                        }
                    } else {
                        println!("ERROR converting object to string: {:?}", string);
                    }
                
                }

            }
        }
        
        splits_vec.sort_by(|lhs,rhs| rhs.season.cmp(&lhs.season));
        splits_vec
    }

}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct StatEntry {
    #[serde(rename = "type")]
    _type: GroupType,
    #[serde(rename = "group")]
    _group: GroupType,
    pub splits: Vec<Split>,
}

#[allow(unused)]
impl StatEntry {
    pub fn get_group(&self) -> String {
        self._group.display_name.clone()
    }

    pub fn get_type(&self) -> String {
        self._type.display_name.clone()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Split {
    pub season: Option<String>,
    pub stat: GenericStats,
    pub team: TeamGeneric,
    pub game_type: Option<String>,
    pub league: LeagueGeneric,
    pub position: Option<Position>,
    pub player: Option<PersonGeneric>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HittingSplits {
    pub season: Option<String>,
    pub stat: types::HittingStats,
    pub team: TeamGeneric,
    pub game_type: Option<String>,
    pub league: LeagueGeneric,
    pub position: Option<Position>,
    pub player: Option<PersonGeneric>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PitchingSplits {
    pub season: Option<String>,
    pub stat: types::PitchingStats,
    pub team: TeamGeneric,
    pub game_type: Option<String>,
    pub league: LeagueGeneric,
    pub position: Option<Position>,
    pub player: Option<PersonGeneric>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FieldingSplits {
    pub season: Option<String>,
    pub stat: types::FieldingStats,
    pub team: TeamGeneric,
    pub game_type: Option<String>,
    pub league: LeagueGeneric,
    pub position: Option<Position>,
    pub player: Option<PersonGeneric>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupType {
    pub display_name: String
}

pub mod types {
    use serde::{Deserialize, Serialize};
    use crate::objects::schemas::generics::Position;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct HittingStats {
        pub games_played: usize,
        pub runs: usize,
        pub doubles: usize,
        pub triples: usize,
        pub home_runs: usize,
        pub strike_outs: usize,
        pub base_on_balls: usize,
        pub hits: usize,
        pub hit_by_pitch: usize,
        pub avg: String,
        pub at_bats: usize,
        pub obp: String,
        pub slg: String,
        pub ops: String,
        pub caught_stealing: usize,
        pub stolen_bases: usize,
        pub stolen_base_percentage: String,
        pub plate_appearances: usize,
        pub total_bases: usize,
        pub rbi: usize,
        pub sac_bunts: usize,
        pub babip: String,
        pub at_bats_per_home_run: String,
        pub ground_outs: Option<usize>,
        pub air_outs: Option<usize>,
        pub intentional_walks: Option<usize>,
        pub ground_into_double_play: Option<usize>,
        pub number_of_pitches: Option<usize>,
        pub left_on_base: Option<usize>,
        pub sac_flies: Option<usize>,
        pub ground_outs_to_airouts: Option<String>,
        pub catchers_interference: Option<usize>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct PitchingStats {
        pub games_played: usize,
        pub games_started: usize,
        pub ground_outs: Option<usize>,
        pub air_outs: Option<usize>,
        pub runs: usize,
        pub doubles: Option<usize>,
        pub triples: Option<usize>,
        pub home_runs: usize,
        pub strike_outs: usize,
        pub base_on_balls: usize,
        pub intentional_walks: Option<usize>,
        pub hits: usize,
        pub hit_by_pitch: usize,
        pub avg: String,
        pub at_bats: usize,
        pub obp: String,
        pub slg: Option<String>,
        pub ops: Option<String>,
        pub caught_stealing: Option<usize>,
        pub stolen_bases: Option<usize>,
        pub stolen_base_percentage: Option<String>,
        pub ground_into_double_play: Option<usize>,
        pub number_of_pitches: Option<usize>,
        pub era: String,
        pub innings_pitched: String,
        pub wins: usize,
        pub losses: usize,
        pub saves: usize,
        pub save_opportunities: Option<usize>,
        pub holds: Option<usize>,
        pub blown_saves: Option<usize>,
        pub earned_runs: usize,
        pub whip: String,
        pub batters_faced: usize,
        pub outs: usize,
        pub games_pitched: usize,
        pub complete_games: usize,
        pub shutouts: usize,
        pub strikes: Option<usize>,
        pub strike_percentage: Option<String>,
        pub hit_batsmen: usize,
        pub balks: usize,
        pub wild_pitches: usize,
        pub pickoffs: Option<usize>,
        pub total_bases: Option<usize>,
        pub ground_outs_to_airouts: Option<String>,
        pub win_percentage: String,
        pub pitches_per_inning: Option<String>,
        pub games_finished: usize,
        pub strikeout_walk_ratio: String,
        #[serde(rename = "strikeoutsPer9Inn")]
        pub strikeouts_per9inn: String,
        #[serde(rename = "walksPer9Inn")]
        pub walks_per9inn: String,
        #[serde(rename = "hitsPer9Inn")]
        pub hits_per9inn: String,
        pub runs_scored_per9: String,
        pub home_runs_per9: String,
        pub inherited_runners: Option<usize>,
        pub inherited_runners_scored: Option<usize>,
        pub catchers_interference: Option<usize>,
        pub sac_bunts: Option<usize>,
        pub sac_flies: Option<usize>,
    }

    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct FieldingStats {
        pub games_played: usize,
        pub games_started: usize,
        pub assists: usize,
        pub put_outs: usize,
        pub errors: usize,
        pub chances: Option<usize>,
        pub fielding: String,
        pub position: Position,
        pub range_factor_per_game: String,
        #[serde(rename = "rangeFactorPer9Inn")]
        pub range_factor_per9inn: String,
        pub innings: String,
        pub games: usize,
        pub double_plays: usize,
        pub triple_plays: usize,
        pub throwing_errors: Option<usize>,
    }


}

pub mod generic {
    use serde::{Deserialize,Serialize};
    use crate::objects::schemas::generics::Position;

    #[derive(Deserialize, Serialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct GenericStats {
        pub games_played: usize,
        pub games_started: Option<usize>,
        pub runs: Option<usize>,
        pub home_runs: Option<usize>,
        pub strike_outs: Option<usize>,
        pub base_on_balls: Option<usize>,
        pub hits: Option<usize>,
        pub hit_by_pitch: Option<usize>,
        pub avg: Option<String>,
        pub at_bats: Option<usize>,
        pub obp: Option<String>,
        pub era: Option<String>,
        pub innings_pitched: Option<String>,
        pub wins: Option<usize>,
        pub losses: Option<usize>,
        pub saves: Option<usize>,
        pub earned_runs: Option<usize>,
        pub whip: Option<String>,
        pub batters_faced: Option<usize>,
        pub outs: Option<usize>,
        pub games_pitched: Option<usize>,
        pub complete_games: Option<usize>,
        pub shutouts: Option<usize>,
        pub hit_batsmen: Option<usize>,
        pub balks: Option<usize>,
        pub wild_pitches: Option<usize>,
        pub win_percentage: Option<String>,
        pub games_finished: Option<usize>,
        pub strikeout_walk_ratio: Option<String>,
        #[serde(rename = "strikeoutsPer9Inn")]
        pub strikeouts_per9inn: Option<String>,
        #[serde(rename = "walksPer9Inn")]
        pub walks_per9inn: Option<String>,
        #[serde(rename = "hitsPer9Inn")]
        pub hits_per9inn: Option<String>,
        pub runs_scored_per9: Option<String>,
        pub home_runs_per9: Option<String>,
        pub ground_outs: Option<usize>,
        pub air_outs: Option<usize>,
        pub doubles: Option<usize>,
        pub triples: Option<usize>,
        pub intentional_walks: Option<usize>,
        pub slg: Option<String>,
        pub ops: Option<String>,
        pub caught_stealing: Option<usize>,
        pub stolen_bases: Option<usize>,
        pub stolen_base_percentage: Option<String>,
        pub ground_into_double_play: Option<usize>,
        pub number_of_pitches: Option<usize>,
        pub save_opportunities: Option<usize>,
        pub holds: Option<usize>,
        pub blown_saves: Option<usize>,
        pub strikes: Option<usize>,
        pub strike_percentage: Option<String>,
        pub pickoffs: Option<usize>,
        pub total_bases: Option<usize>,
        pub ground_outs_to_airouts: Option<String>,
        pub pitches_per_inning: Option<String>,
        pub inherited_runners: Option<usize>,
        pub inherited_runners_scored: Option<usize>,
        pub catchers_interference: Option<usize>,
        pub sac_bunts: Option<usize>,
        pub sac_flies: Option<usize>,
        pub plate_appearances: Option<usize>,
        pub rbi: Option<usize>,
        pub left_on_base: Option<usize>,
        pub babip: Option<String>,
        pub at_bats_per_home_run: Option<String>,
        pub assists: Option<usize>,
        pub put_outs: Option<usize>,
        pub errors: Option<usize>,
        pub chances: Option<usize>,
        pub fielding: Option<String>,
        pub position: Option<Position>,
        pub range_factor_per_game: Option<String>,
        #[serde(rename = "rangeFactorPer9Inn")]
        pub range_factor_per9inn: Option<String>,
        pub innings: Option<String>,
        pub games: Option<usize>,
        pub double_plays: Option<usize>,
        pub triple_plays: Option<usize>,
        pub throwing_errors: Option<usize>,
    }

}