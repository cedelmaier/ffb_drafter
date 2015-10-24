use std::io::prelude::*;
use std::io::stdin;

use types::*;
use ffb_engine::FFBEngine;
use ffb_io::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// Command line interface main loop
pub fn main_loop() {
    let stdin = stdin();
    let mut engine: FFBEngine = FFBEngine::new();

    for line in stdin.lock().lines() {
        let line = line.unwrap_or("".into());
        let mut params = line.split_whitespace();

        if let Some(first_word) = params.next() {
            match first_word {
                "version"   => version(),
                "init"      => engine.init(),
                "read_raw"  => read_raw(&mut params, &mut engine),
                "read_db"   => read_db_json(&mut params, &mut engine),
                "write_db"  => write_db_json(&mut params, &mut engine),
                "print"     => engine.print(),
                "clear"     => engine.clear(),
                "create_team"   => create_team(&mut params, &mut engine),
                _           => println!("Unknown command: {}", first_word),
            }
        }
    }
}

/// Print version information from cargo metadata
pub fn version() {
    println!("ffb_drafter v{}", VERSION);
    println!("Christopher Edelmaier");
}

/// Read players from a raw format file
pub fn read_raw(params: &mut Params, engine: &mut FFBEngine) {
    // Expect read_raw <Position> <filename>
    let pos_str = params.next().unwrap();
    let pos: Position = match pos_str {
        "QB"    => Position::QB,
        "RB"    => Position::RB,
        "WR"    => Position::WR,
        "TE"    => Position::TE,
        "K"     => Position::K,
        "DEF"   => Position::DEF,
        _       => { println!("Invalid Position: {}", pos_str);
                     return; },
    };
    let filename = params.next().unwrap();
    read_raw_players(filename, pos, &mut engine.players);
}

/// Read a JSON Database
pub fn read_db_json(params: &mut Params, engine: &mut FFBEngine) {
    let filename = params.next().unwrap();
    read_players_json(filename, &mut engine.players);
}

/// Write the entire JSON Database
pub fn write_db_json(params: &mut Params, engine: &mut FFBEngine) {
    let filename = params.next().unwrap();
    write_players_json(filename, &mut engine.players);
}

/// Create a fantasy team
pub fn create_team(params: &mut Params, engine: &mut FFBEngine) {
    let teamname = params.next().unwrap();
    engine.create_team(teamname);
}

