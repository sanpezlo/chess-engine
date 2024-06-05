use crate::{Color, ColorError};
use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlayerError {
    #[error("the player `{0}` is not valid")]
    Invalid(#[from] ColorError),
}

pub const PLAYERS: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player(pub Color);

impl FromStr for Player {
    type Err = PlayerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = Color::from_str(s)?;

        Ok(Player(color))
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Player(Color::White) => "w",
            Player(Color::Black) => "b",
        };

        write!(f, "{}", s)
    }
}

impl Default for Player {
    fn default() -> Self {
        Player(Color::White)
    }
}
