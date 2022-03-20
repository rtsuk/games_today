use anyhow::{Error, Result};
use games_today::{gordle_guesses, FIVE_LETTER_LAST_NAMES};
use std::collections::BTreeMap;
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

#[async_std::main]
async fn main() -> Result<(), Error> {
    let opt = Opts::from_args();

    if opt.is_blank() {
        calculate_frequencies()?;
    } else {
        let names = gordle_guesses(opt.valid_letters, opt.bad_letters, opt.placed_letters);
        println!("guesses = {:#?}", names);
    }
    Ok(())
}
