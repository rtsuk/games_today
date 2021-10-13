use chrono::{DateTime, FixedOffset, Timelike, Utc};
use serde::{Deserialize, Serialize};

mod pages;

const SHARKS_ID: usize = 28;

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
        if self.teams.home.team.id == SHARKS_ID || self.teams.away.team.id == SHARKS_ID {
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
