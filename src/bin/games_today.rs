use anyhow::{anyhow, Error, Result};
use chrono::{Local, Timelike};
use games_today::Schedule;

#[async_std::main]
async fn main() -> Result<(), Error> {
    let uri = "https://statsapi.web.nhl.com/api/v1/schedule";
    let string: String = surf::get(uri)
        .recv_string()
        .await
        .map_err(|e| anyhow!("e: {}", e))?;
    let schedule: Schedule = serde_json::from_str(&string)?;
    for date in schedule.dates {
        println!("{}", date.date.format("%x"));
        for game in date.games {
            let t = game.game_date.with_timezone(&Local).time();
            let (pm, h) = t.hour12();
            let pm_str = if pm { "PM" } else { "AM" };
            println!(
                "{} vs {} @ {}:{:02} {}",
                game.teams.home.team.name,
                game.teams.away.team.name,
                h,
                t.minute(),
                pm_str
            )
        }
    }
    Ok(())
}
