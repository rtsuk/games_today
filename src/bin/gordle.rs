use anyhow::{anyhow, Error, Result};
use games_today::{FranchiseData, PlayerRecordData};
use std::collections::BTreeSet;

#[async_std::main]
async fn main() -> Result<(), Error> {
    let mut names = BTreeSet::new();
    let uri = "https://records.nhl.com/site/api/franchise";
    let string: String = surf::get(uri)
        .recv_string()
        .await
        .map_err(|e| anyhow!("e: {}", e))?;
    let franchises: FranchiseData = serde_json::from_str(&string)?;
    for franchise in franchises.data {
        let goalie_uri = format!(
            "https://records.nhl.com/site/api/franchise-goalie-records?cayenneExp=franchiseId={}",
            franchise.most_recent_team_id
        );
        let string: String = surf::get(goalie_uri)
            .recv_string()
            .await
            .map_err(|e| anyhow!("e: {}", e))?;
        let goalies: PlayerRecordData = serde_json::from_str(&string)?;

        let skater_uri = format!(
            "https://records.nhl.com/site/api/franchise-skater-records?cayenneExp=franchiseId={}",
            franchise.most_recent_team_id
        );
        let string: String = surf::get(skater_uri)
            .recv_string()
            .await
            .map_err(|e| anyhow!("e: {}", e))?;
        let skaters: PlayerRecordData = serde_json::from_str(&string)?;

        for player in goalies.data.iter().chain(skaters.data.iter()) {
            if player.last_name.len() == 5 {
                names.insert(player.last_name.clone());
            }
        }
    }
    println!("const NAMES: [&'static str] = {{");
    for name in names {
        println!(r#""{}","#, name);
    }
    println!("}};");
    Ok(())
}

