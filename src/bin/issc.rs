use anyhow::{anyhow, Error, Result};
use chrono::Local;
use games_today::{issc::InSeasonCupResults, Schedule};

#[allow(unused)]
const URL: &'static str = "https://podcast.sportsnet.ca/31-thoughts/the-in-season-stanley-cup/";

#[async_std::main]
async fn main() -> Result<(), Error> {
    let uri = "https://statsapi.web.nhl.com/api/v1/schedule?season=20212022&gameType=R";
    let string: String = surf::get(uri)
        .recv_string()
        .await
        .map_err(|e| anyhow!("e: {}", e))?;
    let schedule: Schedule = serde_json::from_str(&string)?;
    let tz = Local::now();
    let offset = tz.offset().utc_minus_local() as f64;
    let insc = InSeasonCupResults::new(schedule, offset)?;
    println!("standings = {:#?}", insc.sorted_standings());
    println!("team_map = {:#?}", insc.team_map);
    for game in &insc.next_games {
        println!(
            "next = {} [{:?}]",
            game.describe_upcoming(offset),
            insc.team_map.get(&game.opposition(insc.current_in_season))
        );
    }
    Ok(())
}
