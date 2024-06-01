use std::{fmt, ops::AddAssign, str::FromStr};

use thiserror::Error;

use crate::Player;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CastleRightsType {
    None,
    KingSide,
    QueenSide,
    Both,
}

impl CastleRightsType {
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

#[derive(Error, Debug)]
pub enum CastleRightsError {
    #[error("the castle rights `{0}` is not valid")]
    Invalid(String),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CastleRights(pub [CastleRightsType; 2]);

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

        castle_rights[Player::White as usize] = CastleRightsType::new(white);
        castle_rights[Player::Black as usize] = CastleRightsType::new(black);

        Ok(CastleRights(castle_rights))
    }
}

impl fmt::Display for CastleRights {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        let white = self.0[Player::White as usize] as u8;

        if white & CastleRightsType::KingSide as u8 != 0 {
            s.push('K');
        }

        if white & CastleRightsType::QueenSide as u8 != 0 {
            s.push('Q');
        }

        let black = self.0[Player::Black as usize] as u8;

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

impl Default for CastleRights {
    fn default() -> Self {
        CastleRights([CastleRightsType::None; 2])
    }
}
