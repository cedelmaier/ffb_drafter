extern crate serde;
extern crate serde_json;
extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

use regex::Regex;

static NFLTEAMS_REGEX: &'static str = "(BAL|BUF|CIN|CLE|DEN|HOU|IND|JAC|KC|MIA|NE|NYJ|OAK|PIT|SD|TEN|ARI|ATL|CAR|CHI|DAL|DET|GB|MIN|NO|NYG|PHI|STL|SF|SEA|TB|WAS)";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Position {
    QB,
    RB,
    WR,
    TE,
    K,
    DEF,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub name: String,
    pub pos: Position,
    pub points: f32,
}

impl Player {

    fn new(name: &str, pos: Position, pts: f32) -> Player {
        Player { name: name.to_string(),
                 pos: pos,
                 points: pts }
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self.name).unwrap()
    }

    fn from_json(name: &str) -> Player {
        let deserialized: Player = serde_json::from_str(&name).unwrap();
        deserialized
    }
}

/// Read the players in a raw format
pub fn read_raw_players(filename: &str, pos: Position) {
    let path = Path::new(filename);
    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", path.display(), why),
        Ok(f) => f,
    };

    let re = Regex::new(NFLTEAMS_REGEX).unwrap();

    for line in BufReader::new(file).lines() {
        let ln: String = line.unwrap();

        let sentence: Vec<&str> = re.split(&ln).collect();
        let name: String = sentence[0].to_string().trim().to_string();
        let words: Vec<String> = sentence[1].split(|w: char| w.is_whitespace())
                                   .map(|w| w.to_lowercase())
                                   .filter(|w| !w.is_empty())
                                   .collect();
        let points: f32 = words[9].parse().unwrap();
        let player = Player::new(&name, pos.clone(), points);
        println!("{:?}", player);
    }
}

