use crate::{Color, Player};
use std::{fmt, str::FromStr};
use thiserror::Error;

pub const PIECE_TYPES: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    piece_type: PieceType,
    player: Player,
}

impl Piece {
    pub fn new(piece_type: PieceType, player: Player) -> Self {
        Piece { piece_type, player }
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn player(&self) -> Player {
        self.player
    }
}

impl FromStr for Piece {
    type Err = PieceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();

        let (piece_type, player) = match chars.next() {
            Some('P') => (PieceType::Pawn, Player(Color::White)),
            Some('N') => (PieceType::Knight, Player(Color::White)),
            Some('B') => (PieceType::Bishop, Player(Color::White)),
            Some('R') => (PieceType::Rook, Player(Color::White)),
            Some('Q') => (PieceType::Queen, Player(Color::White)),
            Some('K') => (PieceType::King, Player(Color::White)),
            Some('p') => (PieceType::Pawn, Player(Color::Black)),
            Some('n') => (PieceType::Knight, Player(Color::Black)),
            Some('b') => (PieceType::Bishop, Player(Color::Black)),
            Some('r') => (PieceType::Rook, Player(Color::Black)),
            Some('q') => (PieceType::Queen, Player(Color::Black)),
            Some('k') => (PieceType::King, Player(Color::Black)),
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

        if self.player == Player(Color::White) {
            write!(f, "{}", piece_type.to_uppercase())
        } else {
            write!(f, "{}", piece_type)
        }
    }
}
