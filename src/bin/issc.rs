use anyhow::{anyhow, Error, Result};
use chrono::{Date, Utc};
use deunicode::deunicode;
use games_today::{teams::*, Schedule, Team};
use inflector::Inflector;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const TEAMS_TEXT: &str = include_str!("../../data/teams.json");

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct AllTeams {
    teams: Vec<Team>,
}

#[allow(unused)]
const CAROLYN_TEAMS: [usize; 8] = [
    VEGAS_GOLDEN_KNIGHTS_ID,
    NEW_YORK_RANGERS_ID,
    PHILADELPHIA_FLYERS_ID,
    SEATTLE_KRAKEN_ID,
    CHICAGO_BLACKHAWKS_ID,
    PITTSBURGH_PENGUINS_ID,
    COLUMBUS_BLUE_JACKETS_ID,
    ANAHEIM_DUCKS_ID,
];

#[allow(unused)]
const DAVID_TEAMS: [usize; 8] = [
    COLORADO_AVALANCHE_ID,
    VANCOUVER_CANUCKS_ID,
    FLORIDA_PANTHERS_ID,
    WASHINGTON_CAPITALS_ID,
    TORONTO_MAPLE_LEAFS_ID,
    OTTAWA_SENATORS_ID,
    MONTREAL_CANADIENS_ID,
    BUFFALO_SABRES_ID,
];

#[allow(unused)]
const JEFF_TEAMS: [usize; 8] = [
    TAMPA_BAY_LIGHTNING_ID,
    CAROLINA_HURRICANES_ID,
    DETROIT_RED_WINGS_ID,
    EDMONTON_OILERS_ID,
    MINNESOTA_WILD_ID,
    ST_LOUIS_BLUES_ID,
    LOS_ANGELES_KINGS_ID,
    ARIZONA_COYOTES_ID,
];

#[allow(unused)]
const ELIOTTE_TEAMS: [usize; 8] = [
    NEW_YORK_ISLANDERS_ID,
    WINNIPEG_JETS_ID,
    DALLAS_STARS_ID,
    BOSTON_BRUINS_ID,
    CALGARY_FLAMES_ID,
    NEW_JERSEY_DEVILS_ID,
    NASHVILLE_PREDATORS_ID,
    SAN_JOSE_SHARKS_ID,
];

#[allow(unused)]
const URL: &'static str = "https://podcast.sportsnet.ca/31-thoughts/the-in-season-stanley-cup/";

#[async_std::main]
async fn main() -> Result<(), Error> {
    if false {
        let all_teams: AllTeams = serde_json::from_str(&TEAMS_TEXT).expect("from_str");
        for team in all_teams.teams {
            println!(
                "pub const {}_ID : usize = {};",
                deunicode(&team.name).to_screaming_snake_case(),
                team.id
            )
        }
    }
    let uri = "https://statsapi.web.nhl.com/api/v1/schedule?startDate=2021-10-12&endDate=2021-10-13&gameType=R";
    //    let uri = "https://statsapi.web.nhl.com/api/v1/schedule?season=20172018&gameType=R";
    let string: String = surf::get(uri)
        .recv_string()
        .await
        .map_err(|e| anyhow!("e: {}", e))?;
    let schedule: Schedule = serde_json::from_str(&string)?;
    let mut last_transfer_date: Option<Date<Utc>> = None;
    let mut current_in_season = 14;
    let mut current_in_season_name = "".to_string();
    let mut days_with_cup: HashMap<String, usize> = HashMap::new();
    for date in schedule.dates {
        println!("{}", date.date.format("%x"));
        for game in date.games {
            if game.is_finished() && game.is_regular_season() {
                if game.has_competitor(current_in_season) {
                    if let Some((new_holder, new_holder_name)) =
                        game.check_for_handoff(current_in_season)
                    {
                        println!("{} takes the cup", new_holder_name);
                        if let Some(last_transfer_date) = last_transfer_date {
                            let days = game.game_date.date() - last_transfer_date;
                            let current_days =
                                days_with_cup.entry(new_holder_name.to_owned()).or_insert(0);
                            *current_days += days.num_days() as usize;
                        }
                        last_transfer_date = Some(game.game_date.date());
                        current_in_season = new_holder;
                        current_in_season_name = new_holder_name;
                    }
                }
            }
        }
    }

    if let Some(last_transfer_date) = last_transfer_date {
        let days = Utc::today() - last_transfer_date;
        let current_days = days_with_cup.entry(current_in_season_name).or_insert(0);
        *current_days += days.num_days() as usize;
    }

    dbg!(days_with_cup);
    Ok(())
}
