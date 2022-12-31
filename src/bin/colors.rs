use anyhow::{Error, Result};
use games_today::teams::*;
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::Write};

const ALL_TEAMS: &[usize] = &[
    ANAHEIM_DUCKS_ID,
    ARIZONA_COYOTES_ID,
    BOSTON_BRUINS_ID,
    BUFFALO_SABRES_ID,
    CALGARY_FLAMES_ID,
    CAROLINA_HURRICANES_ID,
    CHICAGO_BLACKHAWKS_ID,
    COLORADO_AVALANCHE_ID,
    COLUMBUS_BLUE_JACKETS_ID,
    DALLAS_STARS_ID,
    DETROIT_RED_WINGS_ID,
    EDMONTON_OILERS_ID,
    FLORIDA_PANTHERS_ID,
    LOS_ANGELES_KINGS_ID,
    MINNESOTA_WILD_ID,
    MONTREAL_CANADIENS_ID,
    NASHVILLE_PREDATORS_ID,
    NEW_JERSEY_DEVILS_ID,
    NEW_YORK_ISLANDERS_ID,
    NEW_YORK_RANGERS_ID,
    OTTAWA_SENATORS_ID,
    PHILADELPHIA_FLYERS_ID,
    PITTSBURGH_PENGUINS_ID,
    SAN_JOSE_SHARKS_ID,
    SEATTLE_KRAKEN_ID,
    ST_LOUIS_BLUES_ID,
    TAMPA_BAY_LIGHTNING_ID,
    TORONTO_MAPLE_LEAFS_ID,
    VANCOUVER_CANUCKS_ID,
    VEGAS_GOLDEN_KNIGHTS_ID,
    WASHINGTON_CAPITALS_ID,
    WINNIPEG_JETS_ID,
];

const COLOR_DATA: &str = include_str!("../../data/team_colors.json");

const HEAD: &str = r#"<!DOCTYPE html>
<html>
    <head>
      <!-- Required meta tags -->
      <meta charset="utf-8">
      <meta name="viewport" content="width=device-width, initial-scale=1">

      <link rel="stylesheet" href="colors.css"/>
      <title>NHL Colors</title>
    </head>
<body>
<ul>"#;

const TAIL: &str = r#"</ul>
</body>
</html>"#;

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
struct Colors {
    hex: Vec<String>,
}

#[derive(Debug, Default, Deserialize, PartialEq, Eq)]
struct TeamColor {
    name: String,
    colors: Colors,
}

type TeamNameMap = HashMap<&'static str, usize>;

fn team_class_name(team: &str, map: &TeamNameMap) -> String {
    format!(
        "team_id_{}",
        map.get(team).expect(&format!("team {}", team))
    )
}

fn two_team_class_name(team: &str, other_team: &str, map: &TeamNameMap) -> String {
    format!(
        "team_ids_{}_{}",
        map.get(team).expect(&format!("team {}", team)),
        map.get(other_team).expect(&format!("team {}", other_team)),
    )
}

fn main() -> Result<(), Error> {
    let colors: Vec<TeamColor> = serde_json::from_str(COLOR_DATA)?;

    let team_names: TeamNameMap = ALL_TEAMS
        .iter()
        .copied()
        .map(|team_id| (team_name(team_id), team_id))
        .collect();

    let mut html_file = File::create("colors.html")?;
    let mut css_file = File::create("colors.css")?;

    writeln!(html_file, "{HEAD}")?;

    for team in &colors {
        let class_name = team_class_name(&team.name, &team_names);
        let team_color = &team.colors.hex[0];
        writeln!(css_file, "span.{class_name} {{")?;
        writeln!(css_file, "  display: inline-block;")?;
        writeln!(css_file, "  color: white;")?;
        writeln!(css_file, "  background-color: #{};", team.colors.hex[0])?;
        writeln!(css_file, "}}\n")?;
        for other_team in &colors {
            if other_team != team {
		        let other_class_name = team_class_name(&other_team.name, &team_names);
                let class_name_two = two_team_class_name(&team.name, &other_team.name, &team_names);
                let other_team_color = &other_team.colors.hex[0];
                writeln!(css_file, "li.{class_name_two} {{")?;
                writeln!(css_file, "  color: white;")?;
                writeln!(css_file, "  background-color: #{};", team.colors.hex[0])?;
                writeln!(css_file, "  background: linear-gradient(90deg, #{} 0%, #{} 30%, black 45%, black 55%, #{} 70%, #{} 100%);",
		team_color, team_color, other_team_color, other_team_color)?;
                writeln!(css_file, "}}\n")?;

                writeln!(
                    html_file,
                    r#"<li><span class="{}">{}</span> @ <span class="{}">{}</span></li>"#,
                    class_name, team.name, other_class_name, other_team.name
                )?;
            }
        }
        writeln!(
            html_file,
            r#"<li class="{}">{}</li>"#,
            class_name, team.name
        )?;
    }

    writeln!(html_file, "{TAIL}")?;

    Ok(())
}
