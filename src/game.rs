// File: game.rs
// File Created: 15 Dec 2018 20:31
// By Maxence Moutoussamy <maxence.moutoussamy1@gmail.com>

use game::Error::*;
use game::Player::*;
use game::Tile::*;
use std::fmt;
use std::fmt::Write;

const BOARD_WIDTH: usize = 3;
macro_rules! compute_index_with_coordinates {
    ($x:ident, $y:ident) => {
        $y * BOARD_WIDTH + $x
    };

    ($x:expr, $y:expr) => {
        $y * BOARD_WIDTH + $x
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Used(Player),
}

/* Player */

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    P1,
    P2,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_char(char::from(self))
    }
}

impl From<&Player> for char {
    fn from(p: &Player) -> Self {
        match p {
            P1 => 'X',
            P2 => 'O',
        }
    }
}

/* Winner */

#[derive(Debug, PartialEq)]
pub enum Winner {
    Nobody,
    Player(Player),
}

/* Error */

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidCoordinates,
    TileAlreadyUsed,
}

/* Game */

#[derive(Debug, PartialEq)]
pub struct Game {
    board: [Tile; BOARD_WIDTH * BOARD_WIDTH],
    turn: Player,
    winner: Option<Winner>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [Tile::Empty; BOARD_WIDTH * BOARD_WIDTH],
            turn: P1,
            winner: None,
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

    fn get_winner(&self) -> Option<Winner> {
        for i in 0..BOARD_WIDTH {
            // lines
            let tile = self.board[compute_index_with_coordinates! {0, i}];
            if let Used(player) = tile {
                if tile == self.board[compute_index_with_coordinates! {1, i}]
                && tile == self.board[compute_index_with_coordinates! {2, i}] {
                    return Some(Winner::Player(player));
                }
            }

            // columns
            let tile = self.board[compute_index_with_coordinates! {i, 0}];
            if let Used(player) = tile {
                if tile == self.board[compute_index_with_coordinates! {i, 1}]
                && tile == self.board[compute_index_with_coordinates! {i, 2}] {
                    return Some(Winner::Player(player));
                }
            }
        }

        // first diagonal
        let tile = self.board[compute_index_with_coordinates!{0, 0}];
        if let Used(player) = tile {
            if tile == self.board[compute_index_with_coordinates!{1, 1}]
            && tile == self.board[compute_index_with_coordinates!{2, 2}] {
                return Some(Winner::Player(player));
            }
        }

        // last diagonal
        let tile = self.board[compute_index_with_coordinates!{2, 0}];
        if let Used(player) = tile {
            if tile == self.board[compute_index_with_coordinates!{1, 1}]
            && tile == self.board[compute_index_with_coordinates!{0, 2}] {
                return Some(Winner::Player(player))
            }
        }

        // equality
        for &tile in self.board.iter() {
            if tile == Tile::Empty {
                return None;
            }
        }

        Some(Winner::Nobody)
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
                if x > 0 {
                    line.push_str(" |");
                }
                line.push(' ');
                line.push(match self.board[compute_index_with_coordinates! {x, y}] {
                    Empty => ' ',
                    Used(p) => char::from(&p),
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
                turn: P1,
                winner: None,
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

    macro_rules! get_winner_test {
        ($name:ident : [$($x:expr, $y:expr => $tile:expr;)*] => $res:expr) => {
                #[test]
                fn $name() {
                    let mut _game = Game::new();
                    $(
                        _game.board[compute_index_with_coordinates!{$x, $y}] = $tile;
                    )*
                    assert_eq!(_game.get_winner(), $res);
                }
        };
    }

    get_winner_test!(no_winner_empty_map: [] => None);

    get_winner_test!(no_winner_3_random_player: [
            0, 0 => Used(P1);
            0, 1 => Used(P1);
            1, 0 => Used(P1);
        ] => None
    );

    get_winner_test!(no_winner_filled_line_with_2_players: [
            0, 0 => Used(P1);
            1, 0 => Used(P1);
            2, 0 => Used(P2);
        ] => None
    );

    get_winner_test!(winner_on_first_line: [
            0, 0 => Used(P1); 
            1, 0 => Used(P1); 
            2, 0 => Used(P1);
        ] => Some(Winner::Player(P1))
    );

    get_winner_test!(winner_on_middle_line: [
            0, 1 => Used(P1); 
            1, 1 => Used(P1); 
            2, 1 => Used(P1);
        ] => Some(Winner::Player(P1))
    );

    get_winner_test!(winner_on_last_line: [
            0, 2 => Used(P1); 
            1, 2 => Used(P1); 
            2, 2 => Used(P1);
        ] => Some(Winner::Player(P1))
    );

    get_winner_test!(winner_on_first_column: [
            0, 0 => Used(P2); 
            0, 1 => Used(P2); 
            0, 2 => Used(P2);
        ] => Some(Winner::Player(P2))
    );

    get_winner_test!(winner_on_middle_column: [
            1, 0 => Used(P2); 
            1, 1 => Used(P2); 
            1, 2 => Used(P2);
        ] => Some(Winner::Player(P2))
    );

    get_winner_test!(winner_on_last_column: [
            2, 0 => Used(P2); 
            2, 1 => Used(P2); 
            2, 2 => Used(P2);
        ] => Some(Winner::Player(P2))
    );

    get_winner_test!(winner_on_first_diagonal: [
        0, 0 => Used(P1);
        1, 1 => Used(P1);
        2, 2 => Used(P1);
    ] => Some(Winner::Player(P1)));

    get_winner_test!(winner_on_last_diagonal: [
        2, 0 => Used(P2);
        1, 1 => Used(P2);
        0, 2 => Used(P2);
    ] => Some(Winner::Player(P2)));

    get_winner_test!(equality: [
        0, 0 => Used(P1); 1, 0 => Used(P1); 2, 0 => Used(P2);
        0, 1 => Used(P2); 1, 1 => Used(P2); 2, 1 => Used(P1);
        0, 2 => Used(P1); 1, 2 => Used(P2); 2, 2 => Used(P1);
    ] => Some(Winner::Nobody));

    #[test]
    fn test_game_play() {
        let mut game = Game::new();

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
                winner: None,
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
                winner: None,
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
                winner: None,
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
                winner: None,
            }
        );
    }
}
