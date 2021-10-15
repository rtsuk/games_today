use anyhow::{Error, Result};
use games_today::issc::InSeasonCupResults;

#[allow(unused)]
const URL: &'static str = "https://podcast.sportsnet.ca/31-thoughts/the-in-season-stanley-cup/";

#[async_std::main]
async fn main() -> Result<(), Error> {
    let insc = InSeasonCupResults::new().await?;
    dbg!(&insc);
    Ok(())
}
