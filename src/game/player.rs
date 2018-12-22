// File: player.rs
// File Created: 22 Dec 2018 17:41
// By Maxence Moutoussamy <maxence.moutoussamy1@gmail.com>

use std::fmt::{Display, Formatter, Result, Write};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    P1,
    P2,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter) -> Result {
        f.write_char(char::from(self))
    }
}

impl From<&Player> for char {
    fn from(p: &Player) -> Self {
        match p {
            Player::P1 => 'X',
            Player::P2 => 'O',
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from_convertion() {
        assert_eq!('X', char::from(&Player::P1));
        assert_eq!('O', char::from(&Player::P2));
    }

    #[test]
    fn test_display() {
        assert_eq!("X", format!("{}", Player::P1));
        assert_eq!("O", format!("{}", Player::P2));
    }
}
