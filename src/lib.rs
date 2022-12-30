use chrono::{DateTime, FixedOffset, Timelike, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};

pub mod pages;

pub mod teams {
    use crate::Team;
    use deunicode::deunicode;
    use inflector::Inflector;
    use serde::{Deserialize, Serialize};

    pub const ANAHEIM_DUCKS_ID: usize = 24;
    pub const ARIZONA_COYOTES_ID: usize = 53;
    pub const BOSTON_BRUINS_ID: usize = 6;
    pub const BUFFALO_SABRES_ID: usize = 7;
    pub const CALGARY_FLAMES_ID: usize = 20;
    pub const CAROLINA_HURRICANES_ID: usize = 12;
    pub const CHICAGO_BLACKHAWKS_ID: usize = 16;
    pub const COLORADO_AVALANCHE_ID: usize = 21;
    pub const COLUMBUS_BLUE_JACKETS_ID: usize = 29;
    pub const DALLAS_STARS_ID: usize = 25;
    pub const DETROIT_RED_WINGS_ID: usize = 17;
    pub const EDMONTON_OILERS_ID: usize = 22;
    pub const FLORIDA_PANTHERS_ID: usize = 13;
    pub const LOS_ANGELES_KINGS_ID: usize = 26;
    pub const MINNESOTA_WILD_ID: usize = 30;
    pub const MONTREAL_CANADIENS_ID: usize = 8;
    pub const NASHVILLE_PREDATORS_ID: usize = 18;
    pub const NEW_JERSEY_DEVILS_ID: usize = 1;
    pub const NEW_YORK_ISLANDERS_ID: usize = 2;
    pub const NEW_YORK_RANGERS_ID: usize = 3;
    pub const OTTAWA_SENATORS_ID: usize = 9;
    pub const PHILADELPHIA_FLYERS_ID: usize = 4;
    pub const PITTSBURGH_PENGUINS_ID: usize = 5;
    pub const SAN_JOSE_SHARKS_ID: usize = 28;
    pub const SEATTLE_KRAKEN_ID: usize = 55;
    pub const ST_LOUIS_BLUES_ID: usize = 19;
    pub const TAMPA_BAY_LIGHTNING_ID: usize = 14;
    pub const TORONTO_MAPLE_LEAFS_ID: usize = 10;
    pub const VANCOUVER_CANUCKS_ID: usize = 23;
    pub const VEGAS_GOLDEN_KNIGHTS_ID: usize = 54;
    pub const WASHINGTON_CAPITALS_ID: usize = 15;
    pub const WINNIPEG_JETS_ID: usize = 52;

    pub fn team_name(team_id: usize) -> &'static str {
        match team_id {
            ANAHEIM_DUCKS_ID => "Anaheim Ducks",
            ARIZONA_COYOTES_ID => "Arizona Coyotes",
            BOSTON_BRUINS_ID => "Boston Bruins",
            BUFFALO_SABRES_ID => "Buffalo Sabres",
            CALGARY_FLAMES_ID => "Calgary Flames",
            CAROLINA_HURRICANES_ID => "Carolina Hurricanes",
            CHICAGO_BLACKHAWKS_ID => "Chicago Blackhawks",
            COLORADO_AVALANCHE_ID => "Colorado Avalanche",
            COLUMBUS_BLUE_JACKETS_ID => "Columbus Blue Jackets",
            DALLAS_STARS_ID => "Dallas Stars",
            DETROIT_RED_WINGS_ID => "Detroit Red Wings",
            EDMONTON_OILERS_ID => "Edmonton Oilers",
            FLORIDA_PANTHERS_ID => "Florida Panthers",
            LOS_ANGELES_KINGS_ID => "Los Angeles Kings",
            MINNESOTA_WILD_ID => "Minnesota Wild",
            MONTREAL_CANADIENS_ID => "Montréal Canadiens",
            NASHVILLE_PREDATORS_ID => "Nashville Predators",
            NEW_JERSEY_DEVILS_ID => "New Jersey Devils",
            NEW_YORK_ISLANDERS_ID => "New York Islanders",
            NEW_YORK_RANGERS_ID => "New York Rangers",
            OTTAWA_SENATORS_ID => "Ottawa Senators",
            PHILADELPHIA_FLYERS_ID => "Philadelphia Flyers",
            PITTSBURGH_PENGUINS_ID => "Pittsburgh Penguins",
            SAN_JOSE_SHARKS_ID => "San Jose Sharks",
            SEATTLE_KRAKEN_ID => "Seattle Kraken",
            ST_LOUIS_BLUES_ID => "St. Louis Blues",
            TAMPA_BAY_LIGHTNING_ID => "Tampa Bay Lightning",
            TORONTO_MAPLE_LEAFS_ID => "Toronto Maple Leafs",
            VANCOUVER_CANUCKS_ID => "Vancouver Canucks",
            VEGAS_GOLDEN_KNIGHTS_ID => "Vegas Golden Knights",
            WASHINGTON_CAPITALS_ID => "Washington Capitals",
            WINNIPEG_JETS_ID => "Winnipeg Jets",
            _ => "",
        }
    }

    const TEAMS_TEXT: &str = include_str!("../data/teams.json");

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    struct AllTeams {
        teams: Vec<Team>,
    }

    pub fn get_teams() {
        let all_teams: AllTeams = serde_json::from_str(&TEAMS_TEXT).expect("from_str");
        for team in all_teams.teams {
            println!(
                "{}_ID => \"{}\",",
                deunicode(&team.name).to_screaming_snake_case(),
                team.name
            )
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: usize,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TeamAtGame {
    pub score: usize,
    pub team: Team,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Teams {
    pub home: TeamAtGame,
    pub away: TeamAtGame,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    detailed_state: String,
    abstract_game_state: String,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Linescore {
    current_period: usize,
    #[serde(default)]
    current_period_ordinal: String,
    #[serde(default)]
    current_period_time_remaining: String,
    #[serde(default)]
    pub intermission_info: IntermissionInfo,
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IntermissionInfo {
    intermission_time_remaining: usize,
    intermission_time_elapsed: usize,
    in_intermission: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContentLink {
    pub link: String,
}

fn parse_preview_string(preview: &str) -> Option<String> {
    let re = Regex::new(r"[^;]+;\s*([^<]+)").ok()?;
    let captures = re.captures(preview)?;
    Some(captures.get(1)?.as_str().to_string())
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub game_pk: usize,
    pub game_date: DateTime<Utc>,
    pub game_type: String,
    pub content: ContentLink,
    pub teams: Teams,
    pub status: Status,
    #[serde(default)]
    pub linescore: Linescore,
}

impl Game {
    pub fn describe(&self, offset: f64) -> String {
        if self.is_finished() {
            format!(
                "{} @ {}",
                self.teams.away.team.name, self.teams.home.team.name,
            )
        } else if self.is_live() {
            if self.linescore.intermission_info.in_intermission {
                format!(
                    "{} @ {} {} {}:{:02} INT",
                    self.teams.away.team.name,
                    self.teams.home.team.name,
                    self.linescore.current_period_ordinal,
                    self.linescore.intermission_info.intermission_time_remaining / 60,
                    self.linescore.intermission_info.intermission_time_remaining % 60
                )
            } else {
                format!(
                    "{} @ {} {} {}",
                    self.teams.away.team.name,
                    self.teams.home.team.name,
                    self.linescore.current_period_ordinal,
                    self.linescore.current_period_time_remaining
                )
            }
        } else {
            let tz = FixedOffset::west_opt(offset as i32).unwrap();
            let t = self.game_date.with_timezone(&tz).time();
            let (pm, h) = t.hour12();
            let pm_str = if pm { "PM" } else { "AM" };
            format!(
                "{: >2}:{:02} {} {} @ {}",
                h,
                t.minute(),
                pm_str,
                self.teams.away.team.name,
                self.teams.home.team.name,
            )
        }
    }

    pub fn describe_with_preview(&self, offset: f64, previews: &HashMap<usize, String>) -> String {
        let preview = previews.get(&self.game_pk).cloned().unwrap_or_default();
        if preview.len() > 0 {
            format!("{} ({})", self.describe(offset), preview)
        } else {
            self.describe(offset)
        }
    }

    pub fn describe_upcoming(&self, offset: f64) -> String {
        let tz = FixedOffset::west_opt(offset as i32).unwrap();
        let d = self.game_date.with_timezone(&tz);
        let t = d.time();
        let (pm, h) = t.hour12();
        let pm_str = if pm { "PM" } else { "AM" };
        format!(
            "{} {: >2}:{:02} {} {} @ {}",
            d.format("%v"),
            h,
            t.minute(),
            pm_str,
            self.teams.away.team.name,
            self.teams.home.team.name,
        )
    }

    pub fn describe_upcoming_teams(&self) -> String {
        format!(
            "{} @ {}",
            self.teams.away.team.name, self.teams.home.team.name,
        )
    }

    pub fn class(&self) -> String {
        if self.teams.home.team.id == teams::SAN_JOSE_SHARKS_ID
            && self.teams.away.team.id == teams::EDMONTON_OILERS_ID
        {
            "oilers-sharks".to_string()
        } else if self.teams.home.team.id == teams::SAN_JOSE_SHARKS_ID
            || self.teams.away.team.id == teams::SAN_JOSE_SHARKS_ID
        {
            "sharks".to_string()
        } else if self.teams.home.team.id == teams::EDMONTON_OILERS_ID
            || self.teams.away.team.id == teams::EDMONTON_OILERS_ID
        {
            "oilers".to_string()
        } else {
            "".to_string()
        }
    }

    pub fn is_finished(&self) -> bool {
        self.status.abstract_game_state == "Final"
    }

    pub fn is_regular_season(&self) -> bool {
        self.game_type == "R"
    }

    pub fn is_postponed(&self) -> bool {
        self.status.detailed_state == "Postponed"
    }

    pub fn is_preview(&self) -> bool {
        self.status.abstract_game_state == "Preview" && self.status.detailed_state != "Postponed"
    }

    pub fn is_live(&self) -> bool {
        self.status.abstract_game_state == "Live"
    }

    pub fn has_competitor(&self, competitor: usize) -> bool {
        self.teams.away.team.id == competitor || self.teams.home.team.id == competitor
    }

    pub fn opposition(&self, competitor: usize) -> usize {
        assert!(self.has_competitor(competitor));
        if self.teams.away.team.id == competitor {
            self.teams.home.team.id
        } else {
            self.teams.away.team.id
        }
    }

    pub fn winner(&self) -> usize {
        if self.teams.away.score > self.teams.home.score {
            self.teams.away.team.id
        } else {
            self.teams.home.team.id
        }
    }

    pub fn check_for_handoff(&self, competitor: usize) -> Option<usize> {
        if self.has_competitor(competitor) {
            let winner = self.winner();
            if winner != competitor {
                Some(winner)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PreviewItem {
    preview: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    title: String,
    items: Vec<PreviewItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Editorial {
    preview: Preview,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    link: String,
    editorial: Editorial,
}

impl Content {
    pub fn preview_string(&self) -> Option<String> {
        parse_preview_string(&self.editorial.preview.items.get(0)?.preview)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    pub date: chrono::NaiveDate,
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub total_games: usize,
    pub dates: Vec<Date>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GameDate {
    pub date: String,
    pub games: Vec<Game>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct NextGameSchedule {
    pub total_items: usize,
    pub dates: Vec<GameDate>,
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            total_games: 0,
            dates: vec![],
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Franchise {
    pub full_name: String,
    pub id: usize,
    pub most_recent_team_id: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FranchiseData {
    pub data: Vec<Franchise>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRecord {
    pub last_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRecordData {
    pub data: Vec<PlayerRecord>,
}

pub const FIVE_LETTER_LAST_NAMES: &[&str] = &[
    "Aalto", "Aberg", "Acomb", "Acton", "Adams", "Agnew", "Ahcan", "Ahern", "Ahlin", "Ahola",
    "Allan", "Allen", "Alley", "Allum", "Alves", "Armia", "Asham", "Ashby", "Aubin", "Aubry",
    "Audet", "Auger", "Aulie", "Aulin", "Aurie", "Avery", "Awrey", "Ayres", "Babin", "Baird",
    "Baker", "Balej", "Balon", "Banks", "Barbe", "Barch", "Baron", "Barry", "Bates", "Bathe",
    "Bauer", "Bayda", "Beech", "Beers", "Begin", "Bekar", "Belak", "Belle", "Belov", "Benda",
    "Berra", "Berry", "Berti", "Betik", "Betts", "Bicek", "Biega", "Bierk", "Biggs", "Biron",
    "Bjork", "Black", "Blade", "Blair", "Blais", "Blake", "Blidh", "Block", "Bloom", "Bodak",
    "Boddy", "Bodie", "Boehm", "Boldy", "Bonar", "Bonin", "Bonni", "Boone", "Booth", "Borer",
    "Bossy", "Bouck", "Bouma", "Bowen", "Bower", "Bowey", "Boyce", "Boyer", "Boyes", "Boyko",
    "Boyle", "Bozak", "Bozek", "Bozon", "Brady", "Bratt", "Braun", "Breen", "Brent", "Brine",
    "Brink", "Britz", "Broda", "Broll", "Brome", "Brown", "Bruce", "Brule", "Bubla", "Bucyk",
    "Budaj", "Bulis", "Burch", "Burke", "Burns", "Burry", "Buzek", "Byers", "Bykov", "Byram",
    "Byron", "Cahan", "Carey", "Carle", "Carlo", "Caron", "Carse", "Casey", "Cates", "Chara",
    "Chase", "Check", "Cibak", "Ciger", "Cisar", "Clark", "Cline", "Clowe", "Clune", "Cohen",
    "Cooke", "Corsi", "Corso", "Corvo", "Cotch", "Coutu", "Cowan", "Cowen", "Cowie", "Coyle",
    "Crabb", "Craig", "Crisp", "Cross", "Crowe", "Curry", "Cutta", "Dafoe", "Daley", "Dalpe",
    "Danis", "Dansk", "Darby", "David", "Davie", "Davis", "Dawes", "DeLeo", "Debol", "Denis",
    "Dewar", "Dietz", "Divis", "Djoos", "Doell", "Doran", "Dorey", "Doull", "Doyon", "Drake",
    "Dries", "Druce", "Drury", "Duehr", "Dumba", "Dunne", "Dupre", "Duris", "Durno", "Durzi",
    "Dwyer", "Eager", "Eakin", "Eaton", "Eaves", "Edler", "Egers", "Ehman", "Ekman", "Elias",
    "Elich", "Eliot", "Eller", "Ellis", "Elomo", "Elson", "Emery", "Ennis", "Errey", "Esche",
    "Evans", "Fahey", "Faksa", "Fasth", "Faulk", "Fauss", "Faust", "Fayne", "Fedun", "Fedyk",
    "Felix", "Fiala", "Field", "Fiore", "Fiset", "Flett", "Flinn", "Flood", "Floyd", "Flynn",
    "Focht", "Folco", "Foley", "Folin", "Foote", "Forey", "Fotiu", "Foudy", "Freer", "Fritz",
    "Frost", "Fusco", "Gaetz", "Gagne", "Garon", "Gaume", "Gavey", "Gavin", "Geale", "Geran",
    "Gerbe", "Gibbs", "Giles", "Gladu", "Glass", "Gloor", "Glynn", "Godin", "Gomez", "Goren",
    "Gould", "Goyer", "Grant", "Greco", "Green", "Greer", "Gregg", "Greig", "Grier", "Gross",
    "Gruen", "Gruhl", "Gryba", "Gudas", "Guhle", "Guite", "Guren", "Gusev", "Hagel", "Hague",
    "Hajdu", "Hajek", "Halak", "Haley", "Halko", "Halmo", "Hamel", "Handy", "Hanna", "Hardy",
    "Harju", "Harms", "Hasek", "Hauer", "Haula", "Hayek", "Hayes", "Healy", "Heath", "Hecht",
    "Hedin", "Heidt", "Heins", "Hejda", "Henry", "Heron", "Hertl", "Hicke", "Hicks", "Himes",
    "Hinse", "Hintz", "Hnidy", "Hodge", "Hoene", "Hogue", "Holan", "Holik", "Holos", "Holst",
    "Holtz", "Honka", "Horak", "Hordy", "Horne", "Hossa", "Houck", "Houda", "Houde", "Hough",
    "Houle", "Howse", "Hoyda", "Hrkac", "Huard", "Huber", "Hucul", "Huddy", "Hudon", "Hulse",
    "Huras", "Hurme", "Hurst", "Huska", "Hyman", "Hynes", "Irmen", "Irvin", "Irwin", "Issel",
    "James", "Janik", "Jaros", "Jarry", "Jarvi", "Jenik", "Jerwa", "Jirik", "Johns", "Jones",
    "Joyal", "Joyce", "Jurco", "Juzda", "Kabel", "Kadri", "Kaese", "Kahun", "Kakko", "Kalus",
    "Kampf", "Kanko", "Kapla", "Karpa", "Katic", "Keane", "Keans", "Keats", "Keefe", "Kehoe",
    "Keith", "Kelly", "Kempe", "Kenny", "Kerch", "Kindl", "Kisio", "Klatt", "Klein", "Klemm",
    "Klima", "Kloos", "Knott", "Kocur", "Koivu", "Konan", "Konik", "Kopak", "Korab", "Kowal",
    "Kozak", "Kozun", "Kraft", "Krahn", "Krake", "Krebs", "Kreps", "Kromm", "Krook", "Krupp",
    "Kruse", "Kukan", "Kulak", "Kulda", "Kunin", "Kuntz", "Kunyk", "Kurka", "Kurri", "Kurtz",
    "Kuzyk", "Kwong", "Kyrou", "LaDue", "Labbe", "Labre", "Laich", "Laine", "Laing", "Laird",
    "Lalor", "Lamby", "Latal", "Latos", "Latta", "Lauen", "Lauer", "Lazar", "Leach", "Leahy",
    "Lebda", "Leddy", "Ledin", "Leduc", "Leger", "Legge", "Lehto", "Leier", "Leino", "Leivo",
    "Lemay", "Lesuk", "Lever", "Levie", "Lewis", "Libby", "Liles", "Lilja", "Lipon", "Lisin",
    "Loach", "Locas", "Locke", "Logan", "Lojek", "Loney", "Lowry", "Loyns", "Lucas", "Lucic",
    "Luksa", "Lumme", "Lunde", "Lundy", "Luoma", "Luoto", "Lupul", "Lynch", "Lyons", "Lysak",
    "Macey", "Magee", "Maggs", "Major", "Makar", "Malec", "Maley", "Malik", "Mamin", "Manno",
    "March", "Marha", "Mario", "Marks", "Marsh", "Maruk", "Mason", "Matte", "Mayer", "Mazur",
    "McKay", "McKee", "McKim", "McNab", "McRae", "Meech", "Meeke", "Megan", "Meger", "Megna",
    "Meier", "Melin", "Meyer", "Mezei", "Miehm", "Miele", "Migay", "Mikol", "Milks", "Mills",
    "Miner", "Minor", "Modin", "Modry", "Moger", "Moher", "Mohns", "Molin", "Moore", "Moran",
    "Morin", "Moser", "Motin", "Motte", "Moxey", "Mozik", "Munro", "Musil", "Myers", "Myles",
    "Nanne", "Necas", "Neely", "Nevin", "Niemi", "Nieto", "Nigro", "Nilan", "Noble", "Nolan",
    "Nolet", "Noris", "Nosek", "Novak", "Nowak", "Nurse", "Nyrop", "O'Ree", "Oates", "Oberg",
    "Obsut", "Oduya", "Olesz", "Oliwa", "Olsen", "Olson", "Olver", "Omark", "Orban", "Orlov",
    "Orpik", "Osala", "Oshie", "Palat", "Panik", "Pardy", "Parks", "Parro", "Parse", "Pasek",
    "Pasin", "Patey", "Payer", "Payne", "Peake", "Pedan", "Peeke", "Pelyk", "Percy", "Perry",
    "Pesce", "Pesut", "Petan", "Petit", "Petry", "Phair", "Pilar", "Pilon", "Pilut", "Pinho",
    "Pinto", "Pionk", "Piros", "Pirri", "Pirus", "Pitre", "Pivko", "Platt", "Pleau", "Plett",
    "Plumb", "Pocza", "Poeta", "Poile", "Point", "Polak", "Polis", "Powis", "Pratt", "Price",
    "Propp", "Prout", "Prpic", "Prust", "Pryor", "Pudas", "Puppa", "Pusie", "Pyatt", "Pysyk",
    "Quick", "Quine", "Quinn", "Quint", "Radil", "Raffl", "Rallo", "Ralph", "Ranta", "Raska",
    "Ready", "Reeds", "Reedy", "Reese", "Regan", "Regin", "Reich", "Reitz", "Repik", "Resch",
    "Ricci", "Riley", "Rinne", "Rioux", "Rivet", "Roach", "Robak", "Roche", "Rodin", "Roest",
    "Rolfe", "Ronan", "Ronty", "Rosen", "Rossi", "Roupe", "Rouse", "Royer", "Runge", "Russo",
    "Rutta", "Ruutu", "Ryder", "Sabol", "Sacco", "Sakic", "Salei", "Samis", "Sands", "Sarno",
    "Satan", "Sauer", "Sauve", "Sbisa", "Scott", "Sedin", "Segal", "Sejba", "Sejna", "Sekac",
    "Selby", "Semak", "Semin", "Seney", "Seppa", "Shack", "Shand", "Shank", "Sharp", "Sherf",
    "Shero", "Shill", "Shmyr", "Shore", "Short", "Shugg", "Shutt", "Simek", "Simon", "Siren",
    "Sislo", "Sivek", "Skjei", "Slegr", "Sloan", "Smaby", "Smail", "Smart", "Smith", "Smrek",
    "Smrke", "Smyth", "Sneep", "Snell", "Somik", "Sopel", "Soucy", "Speck", "Speer", "Srsen",
    "Staal", "Starr", "Steel", "Steen", "Stern", "Stock", "Stoll", "Stone", "Storm", "Storr",
    "Sturm", "Suchy", "Suess", "Sulak", "Suomi", "Surma", "Sustr", "Suter", "Suzor", "Swain",
    "Sydor", "Sykes", "Szura", "Taffe", "Takko", "Tamer", "Tanev", "Tanti", "Tatar", "Terry",
    "Teves", "Thang", "Thoms", "Thyer", "Tichy", "Tidey", "Tiley", "Titov", "Toews", "Trapp",
    "Traub", "Tripp", "Trnka", "Tropp", "Tudin", "Tudor", "Tufte", "Turco", "Turek", "Tuten",
    "Twist", "Tynan", "Ulmer", "Unger", "Urbom", "Vaive", "Vanek", "Varis", "Vaske", "Vasko",
    "Verot", "Vesce", "Vesey", "Virta", "Vokes", "Volek", "Vopat", "Voros", "Vrana", "Waite",
    "Walsh", "Wares", "Watts", "Weber", "Wedin", "Weeks", "Weise", "Weiss", "Welch", "Wells",
    "Welsh", "White", "Whyte", "Wiebe", "Wiley", "Wilks", "Wiste", "Woods", "Wylie", "Wyman",
    "Yates", "Yelle", "Young", "Zacha", "Zaine", "Zajac", "Zanon", "Zezel", "Zizka", "Zombo",
    "Zubov", "Zykov",
];

pub fn gordle_guesses(
    valid_letters: String,
    bad_letters: String,
    placed_letters: String,
) -> Vec<String> {
    let mut names = Vec::new();
    let name_sets: Vec<HashSet<char>> = FIVE_LETTER_LAST_NAMES
        .iter()
        .map(|name| HashSet::from_iter(name.to_lowercase().chars()))
        .collect();

    let valid_set = HashSet::from_iter(valid_letters.to_lowercase().chars().filter_map(|c| {
        if c != '.' {
            Some(c)
        } else {
            None
        }
    }));
    let valid_vec: Vec<_> = valid_letters
        .to_lowercase()
        .chars()
        .map(|char| if char != '.' { Some(char) } else { None })
        .collect();
    let bad_set = HashSet::from_iter(bad_letters.to_lowercase().chars());
    let placed: Vec<_> = placed_letters
        .to_lowercase()
        .chars()
        .map(|char| if char != '.' { Some(char) } else { None })
        .collect();
    'outer: for (name_set, name) in name_sets.iter().zip(FIVE_LETTER_LAST_NAMES.iter()) {
        for (placed, test) in placed.iter().zip(name.to_lowercase().chars()) {
            if placed.is_some() {
                if Some(test) != *placed {
                    continue 'outer;
                }
            }
        }
        let bad_intersection: HashSet<_> = bad_set.intersection(name_set).collect();
        if bad_intersection.len() == 0 {
            let intersection: HashSet<_> = valid_set.intersection(name_set).collect();
            if intersection.len() == valid_set.len() {
                for (valid, test) in valid_vec.iter().zip(name.to_lowercase().chars()) {
                    if valid.is_some() {
                        if Some(test) == *valid {
                            continue 'outer;
                        }
                    }
                }
                names.push(name.to_string());
            }
        }
    }
    names
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_preview_str() {
        const PREVIEW_STRING: &str = "<h4><b>RED WINGS (7-5-4) at SHARKS (6-9-3)</b></h4><h5><b>10:30 p.m. ET; NBCSCA, BSDET, ESPN+, SN NOW</b><br />&nbsp;</h5>";

        let parsed = parse_preview_string(PREVIEW_STRING).unwrap();

        assert_eq!("NBCSCA, BSDET, ESPN+, SN NOW", &parsed);
    }
}
