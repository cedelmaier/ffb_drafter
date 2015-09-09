extern crate ffb_drafter;

use ffb_drafter::*;

use std::collections::HashMap;

fn main() {
    let mut players: HashMap<String, ffb_io::Player> = HashMap::new();

    ffb_io::read_raw_players(r"data/qb.dat", ffb_io::Position::QB, &mut players);

    // Convert players to json
    ffb_io::write_players_json(r"data/players.json", &players);

    // Make sure we can read them back
    players.clear();
    ffb_io::read_players_json(r"data/players.json", &mut players);

    for val in players.values() {
        println!("{:?}", val);
    }
}
