use crate::{Color, File, FileError, Rank, RankError};
use std::{fmt, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SquareError {
    #[error("invalid length (expected 2, got {0})")]
    Length(usize),

    #[error("{0}")]
    Rank(#[from] RankError),

    #[error("{0}")]
    File(#[from] FileError),

    #[error("uknown error")]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Square(pub u8);

impl Square {
    pub fn new(file: File, rank: Rank) -> Self {
        Self(rank as u8 * 8 + file as u8)
    }

    pub fn file(self) -> File {
        File::new(self.0 % 8)
    }

    pub fn rank(self) -> Rank {
        Rank::new(self.0 / 8)
    }

    pub fn color(self) -> Color {
        if self.rank() as u8 % 2 == 0 {
            if self.file() as u8 % 2 == 0 {
                Color::Black
            } else {
                Color::White
            }
        } else {
            if self.file() as u8 % 2 == 0 {
                Color::White
            } else {
                Color::Black
            }
        }
    }
}

impl FromStr for Square {
    type Err = SquareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        if s.len() != 2 {
            return Err(SquareError::Length(s.len()));
        }

        let file = chars.next().ok_or(SquareError::Unknown)?;
        let rank = chars.next().ok_or(SquareError::Unknown)?;

        Ok(Square::new(
            file.to_string().parse()?,
            rank.to_string().parse()?,
        ))
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.file(), self.rank())
    }
}
