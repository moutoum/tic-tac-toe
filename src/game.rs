// File: game.rs
// File Created: 15 Dec 2018 20:31
// By Maxence Moutoussamy <maxence.moutoussamy1@gmail.com>

use game::Error::*;
use game::Player::*;
use game::Tile::*;
use std::fmt;

const BOARD_WIDTH: usize = 3;
macro_rules! compute_index_with_coordinates {
    ($x:ident, $y:ident) => {
        $y * BOARD_WIDTH + $x
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Used(Player),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    P1,
    P2,
}

#[derive(Debug, PartialEq)]
pub struct Game {
    board: [Tile; BOARD_WIDTH * BOARD_WIDTH],
    turn: Player,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidCoordinates,
    TileAlreadyUsed,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [Tile::Empty; BOARD_WIDTH * BOARD_WIDTH],
            turn: P1,
        }
    }

    pub fn play(&mut self, x: usize, y: usize) -> Result<(), Error> {
        let tile = Used(self.turn);
        self.put_tile(x, y, tile)?;
        self.toggle_turn();
        Ok(())
    }

    fn toggle_turn(&mut self) {
        self.turn = match self.turn {
            P1 => P2,
            P2 => P1,
        };
    }

    fn put_tile(&mut self, x: usize, y: usize, tile: Tile) -> Result<(), Error> {
        let index = compute_index_with_coordinates!(x, y);

        if x > BOARD_WIDTH || y > BOARD_WIDTH {
            return Err(InvalidCoordinates);
        } else if let Used(_) = self.board[index] {
            return Err(TileAlreadyUsed);
        }

        self.board[index] = tile;

        Ok(())
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..BOARD_WIDTH {
            let mut line = String::new();

            if y > 0 {
                f.write_str("---+---+---")?;
            }

            for x in 0..BOARD_WIDTH {
                let tile = self.board[y * BOARD_WIDTH + x];

                if x > 0 {
                    line.push_str(" |");
                }

                line.push(' ');

                line.push(match tile {
                    Empty => ' ',
                    Used(P1) => 'X',
                    Used(P2) => 'O',
                });
            }

            f.write_str(&line)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_game_creation() {
        let game = Game::new();
        assert_eq!(
            game,
            Game {
                board: [Empty; BOARD_WIDTH * BOARD_WIDTH],
                turn: P1
            }
        );
    }

    #[test]
    fn test_game_toogle_turn() {
        let mut game = Game::new();
        assert_eq!(game.turn, P1);
        game.toggle_turn();
        assert_eq!(game.turn, P2);
        game.toggle_turn();
        assert_eq!(game.turn, P1);
    }

    #[test]
    fn test_game_put_tile() {
        let mut game = Game::new();
        assert_eq!(game.board, [Empty; BOARD_WIDTH * BOARD_WIDTH]);

        // working case
        let tile = Used(game.turn);
        game.put_tile(0, 0, tile).unwrap();
        assert_eq!(
            game.board,
            [
                Used(P1),
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty,
                Empty
            ]
        );

        // invalid coordinates case
        game.put_tile(4, 0, tile)
            .unwrap_or_else(|err| assert_eq!(err, InvalidCoordinates));

        // tile already used case
        game.put_tile(0, 0, tile)
            .unwrap_or_else(|err| assert_eq!(err, TileAlreadyUsed));
    }

    #[test]
    fn test_game_play() {
        let mut game = Game::new();
        assert_eq!(
            game,
            Game {
                board: [Empty; BOARD_WIDTH * BOARD_WIDTH],
                turn: P1
            }
        );

        game.play(0, 0).unwrap();
        assert_eq!(
            game,
            Game {
                board: [
                    Used(P1),
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty
                ],
                turn: P2,
            }
        );

        game.play(1, 0).unwrap();
        assert_eq!(
            game,
            Game {
                board: [
                    Used(P1),
                    Used(P2),
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty
                ],
                turn: P1,
            }
        );

        game.play(1, 0)
            .unwrap_or_else(|err| assert_eq!(err, TileAlreadyUsed));
        assert_eq!(
            game,
            Game {
                board: [
                    Used(P1),
                    Used(P2),
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty
                ],
                turn: P1,
            }
        );

        game.play(4, 0)
            .unwrap_or_else(|err| assert_eq!(err, InvalidCoordinates));
        assert_eq!(
            game,
            Game {
                board: [
                    Used(P1),
                    Used(P2),
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty,
                    Empty
                ],
                turn: P1,
            }
        );
    }
}
