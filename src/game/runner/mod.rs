// File: mod.rs
// File Created: 23 Dec 2018 14:16
// By Maxence Moutoussamy <maxence.moutoussamy1@gmail.com>

mod error;

use super::game;
use self::error::Error;

use std::io;

pub struct Runner {
    game: Option<game::Game>,
}

impl Runner {
    pub fn new() -> Runner {
        Runner { game: None }
    }

    pub fn start_game<T>(&mut self, bufread: &mut T)
    where
        T: io::BufRead,
    {
        self.game = Some(game::Game::new());
        println!("New game starting");

        loop {
            self.display_info();
            match Self::read_coordinates(bufread) {
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

    fn read_coordinates<T>(buf: &mut T) -> Result<(usize, usize), Error>
    where
        T: io::BufRead,
    {
        let mut input = String::new();

        buf.read_line(&mut input)?;
        let inputs: Vec<_> = input.split_whitespace().collect();
        match inputs.iter().as_slice() {
            [x, y] => Ok((x.parse()?, y.parse()?)),
            _ => Err(Error::InvalidArgumentsError),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_runner_creation() {
        let runner = Runner::new();
        assert_eq!(runner.game, None);
    }

    macro_rules! read_coordinates_test {
        ($($name:ident : $input:expr => $expected:expr),*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(
                        Runner::read_coordinates(&mut io::BufReader::new($input.as_bytes())),
                        $expected
                    )
                }
            )*
        };
    }

    read_coordinates_test!(
        valid_input: "1 1" => Ok((1, 1)),
        empty_input: "" => Err(Error::InvalidArgumentsError),
        too_much_arguments: "1 2 3" => Err(Error::InvalidArgumentsError)

    );

}
