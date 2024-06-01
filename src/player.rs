use std::{fmt, str::FromStr};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("the player `{0}` is not valid")]
    Invalid(String),
}

pub const PLAYERS: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}

impl Player {
    pub fn new(player: u8) -> Self {
        match player {
            0 => Player::White,
            1 => Player::Black,
            _ => unreachable!(),
        }
    }
}

impl FromStr for Player {
    type Err = PlayerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "w" => Ok(Player::White),
            "b" => Ok(Player::Black),
            _ => Err(PlayerError::Invalid(s.to_string())),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Player::White => "w",
            Player::Black => "b",
        };

        write!(f, "{}", s)
    }
}
