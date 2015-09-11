// Main types needed for everything

extern crate serde;
extern crate serde_json;

use std::str::SplitWhitespace;

pub type Params<'a> = SplitWhitespace<'a>;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Position {
    QB, 
    RB, 
    WR, 
    TE, 
    K,  
    DEF,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Player {
    pub name: String,
    pub team: String,
    pub pos: Position,
    pub points: f32,
}

impl Player {
    pub fn new(name: &str, team: &str, pos: Position, pts: f32) -> Player {
        Player { name: name.to_string(),
                 team: team.to_string(),
                 pos: pos,
                 points: pts }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_json(name: &str) -> Player {
        let deserialized: Player = serde_json::from_str(&name).unwrap();
        deserialized
    }
}

