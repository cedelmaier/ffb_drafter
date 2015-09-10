use std::io::prelude::*;
use std::io::{stdin, BufReader};

use types::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn main_loop() {
    let stdin = stdin();

    for line in stdin.lock().lines() {
        let line = line.unwrap_or("".into());
        let mut params = line.split_whitespace();

        if let Some(first_word) = params.next() {
            match first_word {
                "version"   => version(),
                "init"      => init(),
                "read_raw"  => read_raw(&mut params),
                "read_db"   => read_db_json(&mut params),
                "write_db"  => write_db_json(&mut params),
                _           => println!("Unknown command: {}", first_word),
            }
        }


    }
}

pub fn version() {
   println!("ffb_drafter v{}", VERSION);
   println!("Christopher Edelmaier");
}

pub fn init() {

}

pub fn read_raw(params: &mut Params) {

}

pub fn read_db_json(params: &mut Params) {

}

pub fn write_db_json(params: &mut Params) {

}
