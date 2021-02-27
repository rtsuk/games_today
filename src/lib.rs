use chrono::{DateTime, FixedOffset, Timelike, Utc};
use serde::{Deserialize, Serialize};

mod pages;

const SHARKS_ID: usize = 28;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TeamAtGame {
    pub score: usize,
    pub team: Team,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Teams {
    pub home: TeamAtGame,
    pub away: TeamAtGame,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    abstract_game_state: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub game_date: DateTime<Utc>,
    pub teams: Teams,
    pub status: Status,
}

impl Game {
    pub fn describe(&self, offset: f64) -> String {
        if self.is_finished() {
            format!(
                "{} vs {}",
                self.teams.home.team.name, self.teams.away.team.name,
            )
        } else {
            let tz = FixedOffset::west(offset as i32);
            let t = self.game_date.with_timezone(&tz).time();
            let (pm, h) = t.hour12();
            let pm_str = if pm { "PM" } else { "AM" };
            format!(
                "{} vs {} @ {}:{:02} {}",
                self.teams.home.team.name,
                self.teams.away.team.name,
                h,
                t.minute(),
                pm_str
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
