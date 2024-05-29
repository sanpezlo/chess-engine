use std::{fmt, str::FromStr};

use thiserror::Error;

use crate::Player;

pub const PIECE_TYPES: usize = 6;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn new(piece: u8) -> Self {
        match piece {
            0 => PieceType::Pawn,
            1 => PieceType::Knight,
            2 => PieceType::Bishop,
            3 => PieceType::Rook,
            4 => PieceType::Queen,
            5 => PieceType::King,
            _ => unreachable!(),
        }
    }
}

#[derive(Error, Debug)]
pub enum PieceError {
    #[error("the piece `{0}` is not valid")]
    Invalid(String),
}

pub struct Piece {
    piece_type: PieceType,
    player: Player,
}

impl Piece {
    pub fn new(piece_type: PieceType, player: Player) -> Self {
        Piece { piece_type, player }
    }
}

impl FromStr for Piece {
    type Err = PieceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let (piece_type, player) = match chars.next() {
            Some('P') => (PieceType::Pawn, Player::White),
            Some('N') => (PieceType::Knight, Player::White),
            Some('B') => (PieceType::Bishop, Player::White),
            Some('R') => (PieceType::Rook, Player::White),
            Some('Q') => (PieceType::Queen, Player::White),
            Some('K') => (PieceType::King, Player::White),
            Some('p') => (PieceType::Pawn, Player::Black),
            Some('n') => (PieceType::Knight, Player::Black),
            Some('b') => (PieceType::Bishop, Player::Black),
            Some('r') => (PieceType::Rook, Player::Black),
            Some('q') => (PieceType::Queen, Player::Black),
            Some('k') => (PieceType::King, Player::Black),
            _ => return Err(PieceError::Invalid(s.to_string())),
        };

        Ok(Piece::new(piece_type, player))
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let piece_type = match self.piece_type {
            PieceType::Pawn => "p",
            PieceType::Knight => "n",
            PieceType::Bishop => "b",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k",
        };

        if self.player == Player::White {
            write!(f, "{}", piece_type.to_uppercase())
        } else {
            write!(f, "{}", piece_type)
        }
    }
}
