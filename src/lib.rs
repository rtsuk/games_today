use chrono::{DateTime, FixedOffset, Timelike, Utc};
use serde::{Deserialize, Serialize};

mod pages;

pub mod teams {
    use crate::Team;
    use deunicode::deunicode;
    use inflector::Inflector;
    use serde::{Deserialize, Serialize};

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

    pub fn team_name(team_id: usize) -> &'static str {
        match team_id {
            ANAHEIM_DUCKS_ID => "Anaheim Ducks",
            ARIZONA_COYOTES_ID => "Arizona Coyotes",
            BOSTON_BRUINS_ID => "Boston Bruins",
            BUFFALO_SABRES_ID => "Buffalo Sabres",
            CALGARY_FLAMES_ID => "Calgary Flames",
            CAROLINA_HURRICANES_ID => "Carolina Hurricanes",
            CHICAGO_BLACKHAWKS_ID => "Chicago Blackhawks",
            COLORADO_AVALANCHE_ID => "Colorado Avalanche",
            COLUMBUS_BLUE_JACKETS_ID => "Columbus Blue Jackets",
            DALLAS_STARS_ID => "Dallas Stars",
            DETROIT_RED_WINGS_ID => "Detroit Red Wings",
            EDMONTON_OILERS_ID => "Edmonton Oilers",
            FLORIDA_PANTHERS_ID => "Florida Panthers",
            LOS_ANGELES_KINGS_ID => "Los Angeles Kings",
            MINNESOTA_WILD_ID => "Minnesota Wild",
            MONTREAL_CANADIENS_ID => "Montréal Canadiens",
            NASHVILLE_PREDATORS_ID => "Nashville Predators",
            NEW_JERSEY_DEVILS_ID => "New Jersey Devils",
            NEW_YORK_ISLANDERS_ID => "New York Islanders",
            NEW_YORK_RANGERS_ID => "New York Rangers",
            OTTAWA_SENATORS_ID => "Ottawa Senators",
            PHILADELPHIA_FLYERS_ID => "Philadelphia Flyers",
            PITTSBURGH_PENGUINS_ID => "Pittsburgh Penguins",
            SAN_JOSE_SHARKS_ID => "San Jose Sharks",
            SEATTLE_KRAKEN_ID => "Seattle Kraken",
            ST_LOUIS_BLUES_ID => "St. Louis Blues",
            TAMPA_BAY_LIGHTNING_ID => "Tampa Bay Lightning",
            TORONTO_MAPLE_LEAFS_ID => "Toronto Maple Leafs",
            VANCOUVER_CANUCKS_ID => "Vancouver Canucks",
            VEGAS_GOLDEN_KNIGHTS_ID => "Vegas Golden Knights",
            WASHINGTON_CAPITALS_ID => "Washington Capitals",
            WINNIPEG_JETS_ID => "Winnipeg Jets",
            _ => "",
        }
    }

    const TEAMS_TEXT: &str = include_str!("../data/teams.json");

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    struct AllTeams {
        teams: Vec<Team>,
    }

    pub fn get_teams() {
        let all_teams: AllTeams = serde_json::from_str(&TEAMS_TEXT).expect("from_str");
        for team in all_teams.teams {
            println!(
                "{}_ID => \"{}\",",
                deunicode(&team.name).to_screaming_snake_case(),
                team.name
            )
        }
    }
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

    pub fn winner(&self) -> usize {
        if self.teams.away.score > self.teams.home.score {
            self.teams.away.team.id
        } else {
            self.teams.home.team.id
        }
    }

    pub fn check_for_handoff(&self, competitor: usize) -> Option<usize> {
        if self.has_competitor(competitor) {
            let winner = self.winner();
            if winner != competitor {
                Some(winner)
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

pub mod issc {
    use crate::{teams::*, Schedule};
    use anyhow::Error;
    use chrono::{Date, Utc};
    use std::collections::HashMap;

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

    #[derive(Debug)]
    pub struct Handoff {
        date: Date<Utc>,
        from: usize,
        to: usize,
    }

    #[derive(Default, Debug)]
    pub struct InSeasonCupResults {
        pub team_stats: HashMap<usize, usize>,
        pub standings: HashMap<String, usize>,
        pub handoffs: Vec<Handoff>,
    }

    impl InSeasonCupResults {
        pub fn new(schedule: Schedule) -> Result<Self, Error> {
            let mut last_transfer_date: Option<Date<Utc>> = None;
            let mut current_in_season = 14;
            let mut days_with_cup: HashMap<usize, usize> = HashMap::new();
            let mut handoffs = Vec::new();
            for date in schedule.dates {
                for game in date.games {
                    if game.is_finished() && game.is_regular_season() {
                        if game.has_competitor(current_in_season) {
                            if let Some(new_holder) = game.check_for_handoff(current_in_season) {
                                if let Some(last_transfer_date) = last_transfer_date {
                                    let days = game.game_date.date() - last_transfer_date;
                                    let current_days =
                                        days_with_cup.entry(current_in_season).or_insert(0);
                                    *current_days += days.num_days() as usize;
                                    handoffs.push(Handoff {
                                        date: game.game_date.date(),
                                        from: current_in_season,
                                        to: new_holder,
                                    });
                                } else {
                                    handoffs.push(Handoff {
                                        date: game.game_date.date(),
                                        from: 14,
                                        to: new_holder,
                                    });
                                }
                                last_transfer_date = Some(game.game_date.date());
                                current_in_season = new_holder;
                            }
                        }
                    }
                }
            }

            if let Some(last_transfer_date) = last_transfer_date {
                let days = Utc::today() - last_transfer_date;
                let current_days = days_with_cup.entry(current_in_season).or_insert(0);
                *current_days += days.num_days() as usize;
            }

            let players = [
                ("Carolyn", CAROLYN_TEAMS),
                ("David", DAVID_TEAMS),
                ("Jeff", JEFF_TEAMS),
                ("Elliotte", ELIOTTE_TEAMS),
            ];

            let mut standings = HashMap::new();
            for (player, teams) in &players {
                let days: usize = teams
                    .iter()
                    .map(|team_id| days_with_cup.get(team_id).unwrap_or(&0))
                    .sum();
                standings.insert(player.to_string(), days);
            }

            Ok(Self {
                team_stats: days_with_cup,
                standings,
                handoffs,
                ..Default::default()
            })
        }
    }
}

mod web {
    use wasm_bindgen::prelude::*;
    use yew::prelude::*;

    use crate::pages::GamesToday;

    #[allow(unused)]
    #[wasm_bindgen(start)]
    pub fn run_app() {
        wasm_logger::init(wasm_logger::Config::default());
        App::<GamesToday>::new().mount_to_body();
    }
}
