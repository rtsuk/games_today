use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod pages;

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
pub struct Game {
    pub game_date: DateTime<Utc>,
    pub teams: Teams,
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
