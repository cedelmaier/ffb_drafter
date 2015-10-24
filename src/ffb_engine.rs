use std::collections::HashMap;

use types::*;
use ffb_io::*;

#[derive(Clone, Debug)]
pub struct FFBEngine {
    pub players: HashMap<String, Player>,
    pub teams: HashMap<String, FantasyTeam>,
}

impl FFBEngine {
    pub fn new() -> FFBEngine {
        let p: HashMap<String, Player> = HashMap::new();
        let t: HashMap<String, FantasyTeam> = HashMap::new();
        FFBEngine{ players: p,
                   teams: t}
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

    pub fn create_team(&mut self, teamname: &str) {
        let t = FantasyTeam::new(teamname);
        self.teams.insert(teamname.to_string(), t);
    }
}
