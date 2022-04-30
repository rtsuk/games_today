use anyhow::{anyhow, Error, Result};
use chrono::Local;
use games_today::NextGameSchedule;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(long)]
    date: Option<String>
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    let opt = Opts::from_args();
    let date = opt.date.unwrap_or(String::from("2022-05-02"));
    let uri = format!("https://statsapi.web.nhl.com/api/v1/schedule?expand=schedule.linescore&date={}", date);
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
