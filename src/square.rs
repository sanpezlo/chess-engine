use core::fmt;
use std::str::FromStr;

use thiserror::Error;

use crate::{File, FileError, Rank, RankError};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Square(pub u8);

#[derive(Error, Debug)]
pub enum SquareError {
    #[error("invalid length (expected 2, got {0})")]
    Length(usize),

    #[error("uknown error")]
    Unknown,

    #[error("{0}")]
    Rank(#[from] RankError),

    #[error("{0}")]
    File(#[from] FileError),
}

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
