use anyhow::{anyhow, Error, Result};
use games_today::{issc::InSeasonCupResults, Schedule};

#[allow(unused)]
const URL: &'static str = "https://podcast.sportsnet.ca/31-thoughts/the-in-season-stanley-cup/";

#[async_std::main]
async fn main() -> Result<(), Error> {
    let uri = "https://statsapi.web.nhl.com/api/v1/schedule?startDate=2021-10-12&endDate=2021-10-15&gameType=R";
    let string: String = surf::get(uri)
        .recv_string()
        .await
        .map_err(|e| anyhow!("e: {}", e))?;
    let schedule: Schedule = serde_json::from_str(&string)?;
    let insc = InSeasonCupResults::new(schedule)?;
    dbg!(&insc);
    Ok(())
}
