use anyhow::{anyhow, Error, Result};
use chrono::{Date, Local, Utc};
use games_today::Schedule;
use std::collections::HashMap;

#[async_std::main]
async fn main() -> Result<(), Error> {
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
