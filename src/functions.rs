use chrono::Datelike;
#[allow(unused)]
#[allow(dead_code)]

use pyo3::IntoPy;

use crate::{
    objects::people::{PersonResponse, Person},
    objects::schemas::{
        team::{TeamResponse,Team},
        standings::StandingsResponse,
        schedule::ScheduleResponse
    },
    objects::rosters::RosterResponse,
    utils::BASE,
};

#[allow(unused)]
pub fn get_next_game(team_id: usize) {
    let url: String = format!(
        "https://statsapi.mlb.com/api/v1/teams/{team_id}?hydrate={hydrations}",
        team_id = team_id.to_string(),
        hydrations = "nextSchedule(limit=1),game(tickets,atBatTickets)"
    );

    let response = reqwest::blocking::get(url);
}

#[allow(unused)]
pub fn get_team(team_id: usize) -> Option<Team> {
    let url: String = format!(
        "https://statsapi.mlb.com/api/v1/teams/{team_id}?hydrate={hydrations}",
        team_id = team_id.to_string(),
        hydrations = "venue(location,fieldInfo),nextSchedule(limit=1),game(tickets,atBatTickets)"
    );

    let response = reqwest::blocking::get(url);

    if let Ok(response) = response {
        let team_response: reqwest::Result<TeamResponse> = response.json();

        if let Ok(team_response) = team_response {
            let team = team_response.teams.get(0).unwrap();
            return Some(team.to_owned());
        };
    };

    None
}

#[allow(unused)]
pub fn get_person(person_id: usize) -> Option<Person> {
    let url: String = format!(
        "https://statsapi.mlb.com/api/v1/people/{person_id}",
        person_id = person_id.to_string()
    );

    let response = reqwest::blocking::get(url);

    if let Ok(response) = response {
        let json_resp: reqwest::Result<PersonResponse> = response.json();

        if let Ok(person_resp_obj) = json_resp {
            if let Some(person_obj) = person_resp_obj.people.get(0) {
                return Some(person_obj.to_owned());
            };
        };
    };

    None
}

#[allow(unused)]
pub fn get_roster(team_id: usize) -> Option<RosterResponse> {
    
    let url: String = format!(
        "https://statsapi.mlb.com/api/v1/teams/{team_id}/roster",
        team_id = team_id.to_string()
    );

    let response = reqwest::blocking::get(url);

    if let Ok(response) = response {
        let json_resp: reqwest::Result<RosterResponse> = response.json();
        if let Ok(roster_obj) = json_resp {
            return Some(roster_obj)
        }
    }

    None

}

#[allow(unused)]
pub fn get_schedule(date: Option<String>) -> Option<ScheduleResponse> {
    let hydrations: &str = "game(tickets,atBatTickets)";

    let url: String = match date {
        Some(date) => {
            format!(
                "https://statsapi.mlb.com/api/v1/schedule?sportId=1&date={}&hydrate={}",
                date.to_string(),
                hydrations
            )
        }
        None => {
            format!(
                "https://statsapi.mlb.com/api/v1/schedule?sportId=1&hydrate={}",
                hydrations
            )
        }
    };
    
    let response = reqwest::blocking::get(url);

    if let Ok(response) = response {
        
        let json_resp: reqwest::Result<ScheduleResponse> = response.json();
        if let Ok(json_resp) = json_resp {
            return Some(json_resp)
        } 
    } 

    return None
    

}

#[allow(unused)]
pub fn get_league_standings(season:Option<usize>) {
    // pub fn get_league_standings(season:Option<usize>) -> Option<LeagueStandingsResponse> {
    // hydrate=league,division
    let season: String = match season {
        Some(season) => {
            season.to_string()
        }
        None => {
            chrono::Utc::now().year().to_string()
        }
    };

    let url: String = format!(
        "{base}/api/v1/standings?season={season}&leagueId=103,104&standingsType=byLeague&hydrate=league,division",
        base = BASE,
        season = season
    );

    let response = reqwest::blocking::get(url);

    if let Ok(resp) = response {
        let standings_json: reqwest::Result<StandingsResponse> = resp.json();
        if let Ok(standings_resp) = standings_json {
            println!("{:#?}",standings_resp.records);
        }
    }


}

#[allow(unused)]
pub fn get_division_standings(season:Option<usize>) {
    // pub fn get_league_standings(season:Option<usize>) -> Option<LeagueStandingsResponse> {
    // hydrate=league,division
    let season: String = match season {
        Some(season) => {
            season.to_string()
        }
        None => {
            chrono::Utc::now().year().to_string()
        }
    };

    let url: String = format!(
        "{base}/api/v1/standings?season={season}&leagueId=103,104&standingsType=byDivision&hydrate=league,division",
        base = BASE,
        season = season
    );

    let response = reqwest::blocking::get(url);

    if let Ok(resp) = response {
        let standings_json: reqwest::Result<StandingsResponse> = resp.json();
        if let Ok(standings_resp) = standings_json {
            println!("{:#?}",standings_resp.records);
        }
    }


}

pub fn stat_abbrv(stat:&str) -> &str {
    match stat {
        "gamesPlayed" => "G",
        "atBats" => "AB",
        "position" => "pos",
        "Pos" => "pos",
        "plateAppearances" => "PA",
        "hits" => "H",
        "runs" => "R",
        "doubles" => "2B",
        "triples" => "3B",
        "homeRuns" => "HR",
        "rbi" => "RBI",
        "strikeOuts" => "SO",
        "baseOnBalls" => "BB",
        "intentionalWalks" => "IBB",
        "groundOuts" => "GO",
        "airOuts" => "AO",
        "groundOutsToAirouts" => "GO/AO",
        "gidp" => "GIDP",
        "groundIntoDoublePlay" => "GIDP",
        "groundIntoTriplePlay" => "GITP",
        "gidpOpp" => "GIDPO",
        "hitByPitch" => "HBP",
        "avg" => "AVG",
        "obp" => "OBP",
        "slg" => "SLG",
        "ops" => "OPS",
        "babip" => "BABIP",
        "caughtStealing" => "CS",
        "stolenBases" => "SB",
        "stolenBasePercentage" => "SB%",
        "pitchesThrown" => "P",
        "numberOfPitches" => "P",
        "totalBases" => "TB",
        "leftOnBase" => "LOB",
        "sacBunts" => "sB",
        "sacFlies" => "sF",
        "catchersInterference" => "CI",
        "atBatsPerHomeRun" => "AB/HR",
        "extraBaseHits" => "exBH",
        "pitchesPerPlateAppearance" => "P/PA",
        "walksPerPlateAppearance" => "BB/PA",
        "strikeoutsPerPlateAppearance" => "SO/PA",
        "homeRunsPerPlateAppearance" => "HR/PA",
        "walksPerStrikeout" => "BB/SO",
        "iso" => "ISO",
        "reachedOnError" => "ROE",
        "walkOffs" => "WO",
        "flyOuts" => "FO",
        "popOuts" => "PO",
        "lineOuts" => "LO",
        "totalSwings" => "TS",
        "swingAndMisses" => "Whiffs",
        "ballsInPlay" => "BIP",
        "groundHits" => "GH",
        "flyHits" => "FH",
        "popHits" => "PH",
        "lineHits" => "LH",
        "pickoffs" => "PK",
        "hitBatsmen" => "HB",
        "completeGames" => "CG",
        "shutouts" => "ShO",
        "strikes" => "K",
        "balls" => "B",
        "whip" => "WHIP",
        "balks" => "BK",
        "inningsPitched" => "IP",
        "wins" => "W",
        "losses" => "L",
        "saves" => "SV",
        "saveOpportunities" => "SVO",
        "era" => "ERA",
        "holds" => "HLD",
        "blownSaves" => "BS",
        "earnedRuns" => "ER",
        "battersFaced" => "BF",
        "gamesPitched" => "GP",
        "strikePercentage" => "K%",
        "wildPitches" => "WP",
        "passedBall" => "PB",
        "winPercentage" => "W%",
        "pitchesPerInning" => "P/Inn",
        "strikeoutWalkRatio" => "SO:BB",
        "strikesoutsToWalks" => "SO:BB",
        "strikeoutsPer9Inn" => "SO/9",
        "strikeoutsPer9" => "SO/9",
        "walksPer9Inn" => "BB/9",
        "walksPer9" => "BB/9",
        "baseOnBallsPer9" => "BB/9",
        "hitsPer9Inn" => "H/9",
        "hitsPer9" => "H/9",
        "runsScoredPer9" => "R/9",
        "homeRunsPer9" => "HR/9",
        "inheritedRunners" => "IR",
        "inheritedRunnersScored" => "IRS",
        "qualityStarts" => "QS",
        "runSupport" => "RS",
        "gamesStarted" => "GS",
        "outs" => "O",
        "gamesFinished" => "GF",
        "games" => "G",
        "innings" => "INNs",
        "putOuts" => "PO",
        "assists" => "A",
        "errors" => "E",
        "chances" => "Ch",
        "fielding" => "FLD%",
        "rangeFactorPerGame" => "RF/G",
        "rangeFactorPer9Inn" => "RF/9",
        "doublePlays" => "DP",
        "triplePlays" => "TP",
        "throwingErrors" => "thE",
        "catcherERA" => "cERA",
        "PA" => "PA",
        "AB" => "AB",
        "R" => "R",
        "H" => "H",
        "2B" => "2B",
        "3B" => "3B",
        "HR" => "HR",
        "RBI" => "RBI",
        "BB" => "BB",
        "IBB" => "IBB",
        "SO" => "SO",
        "HBP" => "HBP",
        "SH" => "sB",
        "SF" => "sF",
        "ROE" => "ROE",
        "GIDP" => "GIDP",
        "SB" => "SB",
        "CS" => "CS",
        "batting_avg" => "AVG",
        "onbase_perc" => "OBP",
        "slugging_perc" => "SLG",
        "onbase_plus_slugging" => "OBP",
        "LOB" => "LOB",
        "team_game_num" => "Season Gm#",
        "date_game" => "Date",
        "team_homeORaway" => "",
        "opp_ID" => "Opp",
        "game_result" => "Result",
        "seasonId" => "Season",
        "hasWildcard" => "Wild Card",
        "preSeasonStartDate" => "Pre-Season Start",
        "preSeasonEndDate" => "Pre-Season End",
        "regularSeasonStartDate" => "Regular Season Start",
        "regularSeasonEndDate" => "Regular Season End",
        "lastDate1stHalf" => "1st Half End",
        "firstDate2ndHalf" => "2nd Half Start",
        "allStarDate" => "All-Star Game",
        "postSeasonStartDate" => "Post-Season Start",
        "postSeasonEndDate" => "Post-Season End",
        "seasonStartDate" => "Season Start",
        "seasonEndDate" => "Season End",
        "season" => "season",
        "runsScored" => "R",
        "runsAllowed" => "RA",
        "runDifferential" => "RunDiff",
        "leagueRank" => "Rank (Lg)",
        "divisionRank" => "Rank (Div)",
        "divisionGamesBack" => "Div GB",
        "leagueGamesBack" => "Lg GB",
        "sportGamesBack" => "GB",
        "League" => "Lg",
        "Division" => "Div",
        "bequeathedRunners" => "BQ",
        "bequeathedRunnersScored" => "BQS",
        "103" => "AL",
        "104" => "NL",
        "200" => "AL West",
        "201" => "AL East",
        "202" => "AL Central",
        "203" => "NL West",
        "204" => "NL East",
        "205" => "NL Central",
        "winningPercentage" => "W%",
        "fullName" => "Team",
        "lgAbbrv" => "Lg/Div",
        "manager" => "Manager(s)",
        "GP" => "G",
        "career_game_num" => "Career Gm#",
        "player_game_span" => "Play Span",
        "team_ID" => "Team",
        "batting_order_position" => "BOP",
        "pos_game" => "Pos",
        "inherited_runners" => "IR",
        "inherited_score" => "IS",
        "inplay_gb_total" => "GB",
        "inplay_fb_total" => "FB",
        "inplay_ld" => "LD",
        "inplay_pu" => "PU",
        "Inn_def" => "Inns.",
        "E_def" => "E",
        "DP_def" => "DP",
        "strikes_total" => "K",
        "strikes_looking" => "K (Lk)",
        "strikes_swinging" => "K (Sw)",
        "batters_faced" => "BF",
        "days_rest" => "DR",
        "earned_run_avg" => "ERA",
        "pitches" => "Pitch Ct",
        "split_group" => "split_group",
        "tm_mlbam" => "tm_mlbam",
        "tm_name" => "tm_name",
        "tm_bbrefID" => "tm_bbrefID",
        "lg_mlbam" => "lg_mlbam",
        "lg_abbrv" => "lg_abbrv",
        "div_mlbam" => "div_mlbam",
        "v_mlbam" => "v_mlbam",
        "G" => "G",
        "W" => "W",
        "L" => "L",
        "W%" => "W%",
        "RA" => "RA",
        "RunDiff" => "Rdiff",
        "vAL" => "vAL",
        "vNL" => "vNL",
        "vWest" => "vWest",
        "vEast" => "vEast",
        "vCentral" => "vCentral",
        "Home" => "Home",
        "Away" => "Away",
        "vRHP" => "vRHP",
        "vLHP" => "vLHP",
        "Last10" => "Last10",
        "ExInns" => "ExInns",
        "OneRun" => "OneRun",
        "Winners" => "Winners",
        "Day" => "Day",
        "Night" => "Night",
        "Grass" => "Grass",
        "Turf" => "Turf",
        "home" => "Home",
        "away" => "Away",
        "last10" => "Last10",
        "exInns" => "ExInns",
        "oneRun" => "OneRun",
        "winners" => "Win",
        "day" => "Day",
        "night" => "Night",
        "grass" => "Grass",
        "turf" => "Turf",
        "runDiff" => "R.Diff",
        "Season" => "season",
        "player" => "player",
        "Player" => "player",
        "player_mlbam" => "player_mlbam",
        "player_id" => "player_mlbam",
        "Team" => "team",
        "team" => "team",
        "team_mlbam" => "team_mlbam",
        "team_id" => "team_mlbam",
        "league" => "league",
        "lg" => "league",
        "league_mlbam" => "league_mlbam",
        "league_id" => "league_mlbam",
        "game_type" => "game_type",
        "gameType" => "game_type",
        "event" => "event_type",
        "event_type" => "event_type",
        "eventType" => "eventType",
        _ => ""
    }
}