use anyhow::{anyhow, Error, Result};
use chrono::{DateTime, Utc};
use games_today::{issc::InSeasonCupResults, Schedule};

#[allow(unused)]
const URL: &'static str = "https://podcast.sportsnet.ca/31-thoughts/the-in-season-stanley-cup/";

#[async_std::main]
async fn main() -> Result<(), Error> {
    let date_time_now: DateTime<Utc> = Utc::now();
    let date = date_time_now.date();
    let uri = format!(
            "https://statsapi.web.nhl.com/api/v1/schedule?startDate=2021-10-12&endDate={}&gameType=R",
            date.format("%F")
        );
        dbg!(&uri);
    let string: String = surf::get(uri)
        .recv_string()
        .await
        .map_err(|e| anyhow!("e: {}", e))?;
    let schedule: Schedule = serde_json::from_str(&string)?;
    let insc = InSeasonCupResults::new(schedule)?;
    dbg!(&insc);
    Ok(())
}
