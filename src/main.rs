// File: main.rs
// File Created: 15 Dec 2018 20:08
// By Maxence Moutoussamy <maxence.moutoussamy1@gmail.com>

mod game;

use game::runner;

use std::io::{self, BufReader, Write};

fn main() -> io::Result<()> {
    println!("=== Tic tac toe ===");

    let mut runner = runner::Runner::new();

    loop {
        println!("Choose an option:");
        println!("  (1) Play a game");
        println!("  (2) Quit");

        print!("your choice: ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let choice = input.trim().parse::<u8>();

        if let Err(_) = choice {
            continue;
        }

        match choice.unwrap() {
            1 => {
                let mut input = BufReader::new(io::stdin());
                runner.start_game(&mut input);
            }
            2 => {
                break;
            }
            _ => {
                continue;
            }
        }
    }

    println!("Bye.");
    Ok(())
}
