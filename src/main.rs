// File: main.rs
// File Created: 15 Dec 2018 20:08
// By Maxence Moutoussamy <maxence.moutoussamy1@gmail.com>

mod game;

use game::Game;

fn main() {
    println!("Tic tac toe");
    let mut game = Game::new();

    match game.play(0, 0) {
        Ok(_) => {}
        Err(_) => {}
    }

    match game.play(1, 0) {
        Ok(_) => {}
        Err(_) => {}
    }

    game.display();
}
