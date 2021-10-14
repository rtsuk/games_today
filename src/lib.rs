use chrono::{DateTime, FixedOffset, Timelike, Utc};
use serde::{Deserialize, Serialize};

mod pages;

pub mod teams {
    pub const ANAHEIM_DUCKS_ID: usize = 24;
    pub const ARIZONA_COYOTES_ID: usize = 53;
    pub const BOSTON_BRUINS_ID: usize = 6;
    pub const BUFFALO_SABRES_ID: usize = 7;
    pub const CALGARY_FLAMES_ID: usize = 20;
    pub const CAROLINA_HURRICANES_ID: usize = 12;
    pub const CHICAGO_BLACKHAWKS_ID: usize = 16;
    pub const COLORADO_AVALANCHE_ID: usize = 21;
    pub const COLUMBUS_BLUE_JACKETS_ID: usize = 29;
    pub const DALLAS_STARS_ID: usize = 25;
    pub const DETROIT_RED_WINGS_ID: usize = 17;
    pub const EDMONTON_OILERS_ID: usize = 22;
    pub const FLORIDA_PANTHERS_ID: usize = 13;
    pub const LOS_ANGELES_KINGS_ID: usize = 26;
    pub const MINNESOTA_WILD_ID: usize = 30;
    pub const MONTREAL_CANADIENS_ID: usize = 8;
    pub const NASHVILLE_PREDATORS_ID: usize = 18;
    pub const NEW_JERSEY_DEVILS_ID: usize = 1;
    pub const NEW_YORK_ISLANDERS_ID: usize = 2;
    pub const NEW_YORK_RANGERS_ID: usize = 3;
    pub const OTTAWA_SENATORS_ID: usize = 9;
    pub const PHILADELPHIA_FLYERS_ID: usize = 4;
    pub const PITTSBURGH_PENGUINS_ID: usize = 5;
    pub const SAN_JOSE_SHARKS_ID: usize = 28;
    pub const SEATTLE_KRAKEN_ID: usize = 55;
    pub const ST_LOUIS_BLUES_ID: usize = 19;
    pub const TAMPA_BAY_LIGHTNING_ID: usize = 14;
    pub const TORONTO_MAPLE_LEAFS_ID: usize = 10;
    pub const VANCOUVER_CANUCKS_ID: usize = 23;
    pub const VEGAS_GOLDEN_KNIGHTS_ID: usize = 54;
    pub const WASHINGTON_CAPITALS_ID: usize = 15;
    pub const WINNIPEG_JETS_ID: usize = 52;
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamAtGame {
    pub score: usize,
    pub team: Team,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Teams {
    pub home: TeamAtGame,
    pub away: TeamAtGame,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    detailed_state: String,
    abstract_game_state: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub game_date: DateTime<Utc>,
    pub game_type: String,
    pub teams: Teams,
    pub status: Status,
}

impl Game {
    pub fn describe(&self, offset: f64) -> String {
        if self.is_finished() {
            format!(
                "{} @ {}",
                self.teams.away.team.name, self.teams.home.team.name,
            )
        } else {
            let tz = FixedOffset::west(offset as i32);
            let t = self.game_date.with_timezone(&tz).time();
            let (pm, h) = t.hour12();
            let pm_str = if pm { "PM" } else { "AM" };
            format!(
                "{: >2}:{:02} {} {} @ {}",
                h,
                t.minute(),
                pm_str,
                self.teams.away.team.name,
                self.teams.home.team.name,
            )
        }
    }

    pub fn class(&self) -> String {
        if self.teams.home.team.id == teams::SAN_JOSE_SHARKS_ID
            || self.teams.away.team.id == teams::SAN_JOSE_SHARKS_ID
        {
            "sharks".to_string()
        } else {
            "".to_string()
        }
    }

    pub fn is_finished(&self) -> bool {
        self.status.abstract_game_state == "Final"
    }

    pub fn is_regular_season(&self) -> bool {
        self.game_type == "R"
    }

    pub fn is_postponed(&self) -> bool {
        self.status.detailed_state == "Postponed"
    }

    pub fn has_competitor(&self, competitor: usize) -> bool {
        self.teams.away.team.id == competitor || self.teams.home.team.id == competitor
    }

    pub fn winner(&self) -> (usize, String) {
        if self.teams.away.score > self.teams.home.score {
            (
                self.teams.away.team.id,
                self.teams.away.team.name.to_string(),
            )
        } else {
            (
                self.teams.home.team.id,
                self.teams.home.team.name.to_string(),
            )
        }
    }

    pub fn check_for_handoff(&self, competitor: usize) -> Option<(usize, String)> {
        if self.has_competitor(competitor) {
            let (winner, name) = self.winner();
            if winner != competitor {
                Some((winner, name))
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    pub date: chrono::NaiveDate,
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub total_games: usize,
    pub dates: Vec<Date>,
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            total_games: 0,
            dates: vec![],
        }
    }
}

mod web {
    use wasm_bindgen::prelude::*;
    use yew::prelude::*;

    use crate::pages::GamesToday;


    #[wasm_bindgen(start)]
    pub fn run_app() {
        wasm_logger::init(wasm_logger::Config::default());
        App::<GamesToday>::new().mount_to_body();
    }
}
