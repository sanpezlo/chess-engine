use std::{fmt, str::FromStr};
use thiserror::Error;

/// An error that can occur when parsing a [`Rank`].
#[derive(Error, Debug)]
pub enum RankError {
    /// The rank is not valid.
    #[error("invalid rank (expected a-h, got {0})")]
    Invalid(String),
}

/// A `Rank` on a chessboard.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rank {
    /// The Rank 1.
    One,
    /// The Rank 2.
    Two,
    /// The Rank 3.
    Three,
    /// The Rank 4.
    ///
    Four,
    /// The Rank 5.
    Five,
    /// The Rank 6.
    Six,
    /// The Rank 7.
    Seven,
    /// The Rank 8.
    Eight,
}

impl Rank {
    /// Creates a new `Rank` from a `u8`.
    ///
    /// # Panics
    ///
    /// Panics if the rank is not 0-7.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::Rank;
    /// let rank = Rank::new(0);
    /// assert_eq!(rank, Rank::One);
    /// ```
    pub fn new(rank: u8) -> Self {
        match rank {
            0 => Rank::One,
            1 => Rank::Two,
            2 => Rank::Three,
            3 => Rank::Four,
            4 => Rank::Five,
            5 => Rank::Six,
            6 => Rank::Seven,
            7 => Rank::Eight,
            _ => unreachable!(),
        }
    }
}

/// Parses a `Rank` from a string.
///
/// # Errors
///
/// Returns a [`RankError`] if the string is not a valid rank.
///
/// # Examples
///
/// ```
/// # use chess_engine::Rank;
/// let rank: Rank = "1".parse().unwrap();
/// assert_eq!(rank, Rank::One);
/// ```
impl FromStr for Rank {
    type Err = RankError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Rank::One),
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            _ => Err(RankError::Invalid(s.to_string())),
        }
    }
}

/// Formats the `Rank` as a string.
///
/// # Examples
///
/// ```
/// # use chess_engine::Rank;
/// let rank = Rank::One;
/// assert_eq!(rank.to_string(), "1");
/// ```
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Rank::One => "1",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
        };

        write!(f, "{}", s)
    }
}
