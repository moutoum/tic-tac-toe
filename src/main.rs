// File: main.rs
// File Created: 15 Dec 2018 20:08
// By Maxence Moutoussamy <maxence.moutoussamy1@gmail.com>

mod game;

use game::runner;

use std::io::{self, BufReader};

fn main() {
    println!("=== Tic tac toe ===");
    runner::Runner::new().start_game(&mut BufReader::new(io::stdin()));
}
