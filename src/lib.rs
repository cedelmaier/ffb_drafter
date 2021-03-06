#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;
extern crate regex;

pub mod ffb_engine;
pub mod ffb_io;
pub mod cli;
pub mod types;

