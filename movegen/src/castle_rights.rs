use std::{fmt, str::FromStr};

use chess_engine_core::{CastleRightsType, CastleRightsTypeError, Color};

/// A `CastleRights` in chess.
///
/// # Examples
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let castle_rights = CastleRights([CastleRightsType::Both; Color::LEN]);
/// assert_ne!(castle_rights, CastleRights::default());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CastleRights(pub [CastleRightsType; Color::LEN]);

/// Parses a `CastleRights` from a string.
///
/// # Errors
///
/// Returns a [`CastleRightsTypeError`] if the string is not a valid castle rights.
///
/// # Examples
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let castle_rights: CastleRights = "KQkq".parse().unwrap();
/// assert_eq!(castle_rights, CastleRights([CastleRightsType::Both; Color::LEN]));
/// ```
impl FromStr for CastleRights {
    type Err = CastleRightsTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut castle_rights = CastleRights::default().0;

        if s.len() > 4 || s.len() == 0 {
            return Err(CastleRightsTypeError(s.to_string()));
        }

        if s == "-" {
            return Ok(CastleRights::default());
        }

        for c in s.chars() {
            let lowercase = c.to_lowercase().next().unwrap();

            let castle_rights_type = CastleRightsType::from_str(&lowercase.to_string())?;

            if lowercase == c {
                castle_rights[Color::Black as usize].try_bitor_assign(castle_rights_type)?;
            } else {
                castle_rights[Color::White as usize].try_bitor_assign(castle_rights_type)?;
            }
        }

        Ok(CastleRights(castle_rights))
    }
}

/// Formats a `CastleRights` as a string.
///
/// # Examples
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let castle_rights = CastleRights([CastleRightsType::None; Color::LEN]);
/// assert_eq!(castle_rights.to_string(), "-");
/// ```
impl fmt::Display for CastleRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        let white = self.0[Color::White as usize] as usize;
        let black = self.0[Color::Black as usize] as usize;

        s.push_str(&CastleRightsType::new(white).to_string().to_uppercase());
        s.push_str(&CastleRightsType::new(black).to_string());

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
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let castle_rights = CastleRights::default();
/// assert_eq!(castle_rights, CastleRights([CastleRightsType::None; 2]));
impl Default for CastleRights {
    fn default() -> Self {
        CastleRights([CastleRightsType::None; Color::LEN])
    }
}
