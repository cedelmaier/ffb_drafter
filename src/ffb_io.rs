extern crate serde;
extern crate serde_json;
extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Write;
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
        serde_json::to_string(&self).unwrap()
    }

    fn from_json(name: &str) -> Player {
        let deserialized: Player = serde_json::from_str(&name).unwrap();
        deserialized
    }
}

/// Read the players in a raw format
pub fn read_raw_players(filename: &str, pos: Position, players: &mut HashMap<String, Player>) {
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
        players.insert(name, player.clone());
    }
}

/// Read the entire player database from JSON
pub fn read_players_json(filename: &str, players: &mut HashMap<String, Player>) {
    let path = Path::new(filename);
    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", path.display(), why),
        Ok(f) => f,
    };

    for line in BufReader::new(file).lines() {
        let ln: String = line.unwrap();
        let player: Player = Player::from_json(&ln);
        players.insert(player.name.to_owned(), player.clone());
    }
}

/// Write the entire player database in JSON format using serde
pub fn write_players_json(filename: &str, players: &HashMap<String, Player>) {
    let path = Path::new(filename);
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(f) => f,
    };

    for player in players.values() {
        let ln = player.to_json() + "\n";
        match file.write_all(ln.as_bytes()) {
            Err(why) => println!("couldn't write to {}: {}", path.display(), why),
            Ok(_) => (),
        }
    }
}

