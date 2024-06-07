use crate::Color;
use std::{fmt, str::FromStr};
use thiserror::Error;

/// A `CastleRightsType` in chess.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CastleRightsType {
    /// No castle rights.
    None,
    /// King side castle rights.
    KingSide,
    /// Queen side castle rights.
    QueenSide,
    /// Both king and queen side castle rights.
    Both,
}

impl CastleRightsType {
    /// Creates a new `CastleRightsType` from a `u8`.
    ///
    /// # Panics
    ///
    /// Panics if the castle rights type is not 0-3.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::CastleRightsType;
    /// let castle_rights_type = CastleRightsType::new(0);
    /// assert_eq!(castle_rights_type, CastleRightsType::None);
    /// ```
    pub fn new(castle_rights_type: u8) -> Self {
        match castle_rights_type {
            0 => CastleRightsType::None,
            1 => CastleRightsType::KingSide,
            2 => CastleRightsType::QueenSide,
            3 => CastleRightsType::Both,
            _ => unreachable!(),
        }
    }
}

/// An error that can occur when parsing [`CastleRights`].
#[derive(Error, Debug)]
pub enum CastleRightsError {
    /// The castle rights are not valid.
    #[error("invalid castle rights (expected K, Q, k, or q, got {0})")]
    Invalid(String),
}

/// A `CastleRights` in chess.
///
/// # Examples
///
/// ```
/// # use chess_engine::{CastleRights, CastleRightsType};
/// let castle_rights = CastleRights([CastleRightsType::Both; 2]);
/// assert_ne!(castle_rights, CastleRights::default());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CastleRights(pub [CastleRightsType; 2]);

/// Parses a `CastleRights` from a string.
///
/// # Errors
///
/// Returns a [`CastleRightsError`] if the string is not a valid castle rights.
///
/// # Examples
///
/// ```
/// # use chess_engine::{CastleRights, CastleRightsType};
/// let castle_rights: CastleRights = "KQkq".parse().unwrap();
/// assert_eq!(castle_rights, CastleRights([CastleRightsType::Both; 2]));
/// ```
impl FromStr for CastleRights {
    type Err = CastleRightsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut castle_rights = CastleRights::default().0;

        let (mut white, mut black) = (0, 0);

        for castle_rights_type in s.chars() {
            match castle_rights_type {
                '-' => {
                    if s.len() != 1 {
                        return Err(CastleRightsError::Invalid(s.to_string()));
                    }
                }
                'K' => {
                    white += CastleRightsType::KingSide as u8;
                }
                'Q' => {
                    white += CastleRightsType::QueenSide as u8;
                }
                'k' => {
                    black += CastleRightsType::KingSide as u8;
                }
                'q' => {
                    black += CastleRightsType::QueenSide as u8;
                }

                _ => return Err(CastleRightsError::Invalid(castle_rights_type.to_string())),
            }
        }

        if white > 3 || black > 3 {
            return Err(CastleRightsError::Invalid(s.to_string()));
        }

        castle_rights[Color::White as usize] = CastleRightsType::new(white);
        castle_rights[Color::Black as usize] = CastleRightsType::new(black);

        Ok(CastleRights(castle_rights))
    }
}

/// Formats a `CastleRights` as a string.
///
/// # Examples
///
/// ```
/// # use chess_engine::{CastleRights, CastleRightsType};
/// let castle_rights = CastleRights([CastleRightsType::None; 2]);
/// assert_eq!(castle_rights.to_string(), "-");
/// ```
impl fmt::Display for CastleRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        let white = self.0[Color::White as usize] as u8;

        if white & CastleRightsType::KingSide as u8 != 0 {
            s.push('K');
        }

        if white & CastleRightsType::QueenSide as u8 != 0 {
            s.push('Q');
        }

        let black = self.0[Color::Black as usize] as u8;

        if black & CastleRightsType::KingSide as u8 != 0 {
            s.push('k');
        }

        if black & CastleRightsType::QueenSide as u8 != 0 {
            s.push('q');
        }

        if s.is_empty() {
            s.push('-');
        }

        write!(f, "{}", s)
    }
}

/// The default `CastleRights`.
///
/// # Examples
///
/// ```
/// # use chess_engine::{CastleRights, CastleRightsType};
/// let castle_rights = CastleRights::default();
/// assert_eq!(castle_rights, CastleRights([CastleRightsType::None; 2]));
impl Default for CastleRights {
    fn default() -> Self {
        CastleRights([CastleRightsType::None; 2])
    }
}
