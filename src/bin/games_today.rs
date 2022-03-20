use anyhow::{anyhow, Error, Result};
use chrono::Local;
use games_today::NextGameSchedule;

#[async_std::main]
async fn main() -> Result<(), Error> {
    let uri = "https://statsapi.web.nhl.com/api/v1/schedule?expand=schedule.linescore";
    let string: String = surf::get(uri)
        .recv_string()
        .await
        .map_err(|e| anyhow!("e: {}", e))?;
    let schedule: NextGameSchedule = serde_json::from_str(&string)?;
    let tz = Local::now();
    for date in schedule.dates {
        println!("{}", date.date);
        for game in date.games {
            println!("{}", game.describe(tz.offset().utc_minus_local() as f64),)
        }
    }
    Ok(())
}
