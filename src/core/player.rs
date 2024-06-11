use crate::{Color, ColorError};
use std::{fmt, ops::Not, str::FromStr};
use thiserror::Error;

/// Players in a chess game.
pub const PLAYERS: usize = 2;

/// An error that can occur when parsing a [`Player`].
#[derive(Error, Debug)]
pub enum PlayerError {
    /// The color is not valid.
    #[error("`{0}`")]
    Invalid(#[from] ColorError),
}

/// A `Player` in a chess game.
///
/// A `Player` is represented by a [`Color`].
///
/// # Examples
///
/// ```
/// # use chess_engine::{Player, Color};
/// let player = Player::default();
/// assert_eq!(player, Player(Color::White));
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Player(pub Color);

/// Negates a `Player`.
///
/// # Examples
///
/// ```
/// # use chess_engine::{Player, Color};
/// let player = Player::default();
/// assert_eq!(!player, Player(Color::Black));
/// ```
impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        Player(!self.0)
    }
}

/// Parses a `Player` from a string.
///
/// # Errors
///
/// Returns a [`PlayerError`] if the string is not a valid player.
///
/// # Examples
///
/// ```
/// # use chess_engine::Player;
/// let player: Player = "w".parse().unwrap();
/// assert_eq!(player, Player::default());
/// ```
impl FromStr for Player {
    type Err = PlayerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let color = Color::from_str(s)?;

        Ok(Player(color))
    }
}

/// Formats a `Player` as a string.
///
/// # Examples
///
/// ```
/// # use chess_engine::Player;
/// let player = Player::default();
/// assert_eq!(player.to_string(), "w");
/// ```
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Player(Color::White) => "w",
            Player(Color::Black) => "b",
        };

        write!(f, "{}", s)
    }
}

/// The default `Player` is `Player(Color::White)`.
///
/// # Examples
///
/// ```
/// # use chess_engine::{Player, Color};
/// assert_eq!(Player::default(), Player(Color::White));
/// ```
impl Default for Player {
    fn default() -> Self {
        Player(Color::White)
    }
}
