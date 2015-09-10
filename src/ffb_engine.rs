use std::collections::HashMap;

use types::*;
use ffb_io::*;

#[derive(Clone, Debug)]
pub struct FFBEngine {
    pub players: HashMap<String, Player>,
}

impl FFBEngine {
    pub fn new() -> FFBEngine {
        let p: HashMap<String, Player> = HashMap::new();
        FFBEngine{ players: p }
    }

    pub fn init(&mut self) {
        read_players_json(r"data/players.json", &mut self.players);
        println!("Engine Initialized");
    }

    pub fn print(&self) {
        for val in self.players.values() {
            println!("{:?}", val);
        }
    }

    pub fn clear(&mut self) {
        self.players.clear();
    }
}
