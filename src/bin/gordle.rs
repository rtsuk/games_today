use anyhow::{anyhow, Error, Result};
use games_today::{gordle_guesses, FranchiseData, PlayerRecordData, FIVE_LETTER_LAST_NAMES};
use std::collections::{BTreeMap, BTreeSet};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opts {
    #[structopt(long, default_value = "")]
    valid_letters: String,
    #[structopt(long, default_value = "")]
    #[allow(unused)]
    placed_letters: String,
    #[structopt(long, default_value = "")]
    bad_letters: String,
    #[structopt(long)]
    names: bool,
}

impl Opts {
    fn is_blank(&self) -> bool {
        self.valid_letters.len() == 0
            && self.placed_letters.len() == 0
            && self.bad_letters.len() == 0
    }
}

fn name_value(name: &str, counts: &[BTreeMap<char, usize>; 5]) -> usize {
    name.to_lowercase()
        .chars()
        .enumerate()
        .map(|(i, c)| counts[i].get(&c).unwrap_or(&0))
        .sum()
}

fn no_overlap(a: &str, b: &str) -> bool {
    for (a, b) in a.chars().zip(b.chars()) {
        if a == b {
            return false;
        }
    }
    true
}

fn calculate_frequencies() -> Result<(), Error> {
    let mut counts = [
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
        BTreeMap::new(),
    ];

    for name in FIVE_LETTER_LAST_NAMES {
        for (i, c) in name.to_lowercase().chars().enumerate() {
            *counts[i].entry(c).or_insert(0) += 1;
        }
    }

    let mut values_and_names: Vec<(usize, &'static str)> = FIVE_LETTER_LAST_NAMES
        .iter()
        .map(|name| (name_value(name, &counts), *name))
        .collect();

    values_and_names.sort();
    values_and_names.reverse();

    for i in 0..9 {
        let best = &values_and_names[i].1;
        for (_v, n) in &values_and_names {
            if no_overlap(best, n) {
                println!("{}, {}", best, n);
                break;
            }
        }
    }

    Ok(())
}

async fn get_names() -> Result<BTreeSet<String>, Error> {
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
            franchise.id
        );
        let string: String = surf::get(goalie_uri)
            .recv_string()
            .await
            .map_err(|e| anyhow!("e: {}", e))?;
        let goalies: PlayerRecordData = serde_json::from_str(&string)?;

        let skater_uri = format!(
            "https://records.nhl.com/site/api/franchise-skater-records?cayenneExp=franchiseId={}",
            franchise.id
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
    Ok(names)
}

#[async_std::main]
async fn main() -> Result<(), Error> {
    let opt = Opts::from_args();

    if opt.names {
        let names = get_names().await;
        println!("names = {:#?}", names);
    } else if opt.is_blank() {
        calculate_frequencies()?;
    } else {
        let names = gordle_guesses(opt.valid_letters, opt.bad_letters, opt.placed_letters);
        println!("guesses = {:#?}", names);
    }
    Ok(())
}
