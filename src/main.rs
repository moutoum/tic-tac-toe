// File: main.rs
// File Created: 15 Dec 2018 20:08
// By Maxence Moutoussamy <maxence.moutoussamy1@gmail.com>

mod game;

use std::io;
use std::num;

enum Error {
    ParseError(num::ParseIntError),
    InputError(io::Error),
    InvalidArgumentsError,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::InputError(e)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(e: num::ParseIntError) -> Self {
        Error::ParseError(e)
    }
}

struct GameRunner {
    game: Option<game::Game>,
}

impl GameRunner {
    fn new() -> GameRunner {
        GameRunner { game: None }
    }

    fn start_game(&mut self) {
        self.game = Some(game::Game::new());
        println!("New game starting");

        loop {
            self.display_info();
            match Self::read_coordinates() {
                Ok((x, y)) => {
                    if let Some(game) = &mut self.game {
                        match game.play(x, y) {
                            Err(game::Error::InvalidCoordinates) => {
                                println!("command error: invalid coordinates")
                            }
                            Err(game::Error::TileAlreadyUsed) => {
                                println!("command error: selected coordinate already used")
                            }
                            Ok(_) => match game.winner {
                                None => (),
                                Some(game::Winner::Player(player)) => {
                                    println!("{}\nPlayer {} won the game.", game, player);
                                    break;
                                }
                                Some(game::Winner::Nobody) => {
                                    println!("{}\nNobody won the game.", game);
                                    break;
                                }
                            },
                        };
                    }
                }
                Err(e) => match e {
                    Error::ParseError(e) => println!("command error: {}", e),
                    Error::InputError(e) => println!("command error: {}", e),
                    Error::InvalidArgumentsError => println!("command error: invalid argument"),
                },
            }
        }
    }

    fn display_info(&self) {
        if let Some(game) = &self.game {
            println!(
                "{board}\nPlayer {turn} to play",
                board = game,
                turn = game.turn
            );
        }
    }

    fn read_coordinates() -> Result<(usize, usize), Error> {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;
        let inputs: Vec<_> = input.split_whitespace().collect();
        match inputs.iter().as_slice() {
            [x, y] => Ok((x.parse()?, y.parse()?)),
            _ => Err(Error::InvalidArgumentsError),
        }
    }
}

fn main() -> io::Result<()> {
    println!("=== Tic tac toe ===");
    GameRunner::new().start_game();
    Ok(())
}
