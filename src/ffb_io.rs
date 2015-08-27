extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

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
    pub name: String
}

impl Player {

    fn new(name: &str) -> Player {
        Player { name: name.to_string() }
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
pub fn read_raw_players(filename: String, pos: Position) {
    let path = Path::new(&filename);
    let file = match File::open(&path) {
        Err(why) => panic!("failed to open {}: {}", path.display(), why),
        Ok(f) => f,
    };

    for line in BufReader::new(file).lines() {
        let ln: String = line.unwrap();
        // Split the line on all of the whitespace to get the pieces
        let words: Vec<String> = ln.split(|w: char| !w.is_alphanumeric())
                                   .map(|w| w.to_lowercase())
                                   .filter(|w| !w.is_empty())
                                   .collect();
    }
}

pub fn ffb_io_test() {
    let point = Point { x: 1, y: 2};
    let serialized = serde_json::to_string(&point).unwrap();

    println!("{}", serialized);

    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    println!("{:?}", deserialized);
}
