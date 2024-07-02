use once_cell::sync::Lazy;
use rand::prelude::*;

use crate::{
    CastleRightsType, Color, File, Piece, Rank, Square, CASTLE_RIGHTS_TYPES, PIECE_TYPES, SQUARES,
};

/// A lazy static [`Zobrist`] instance.
pub static ZOBRIST: Lazy<Zobrist> = Lazy::new(|| Zobrist::new());

/// A Zobrist hash for chess.
///
/// # Examples
///
/// ```
/// # use chess_engine::ZOBRIST;
/// println!("{:?}", *ZOBRIST);
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Zobrist {
    color: u64,
    pieces: [[[u64; PIECE_TYPES]; Color::LEN]; SQUARES],
    en_passant: [[u64; File::LEN]; Color::LEN],
    castling_rights: [[u64; CASTLE_RIGHTS_TYPES]; Color::LEN],
}

impl Zobrist {
    /// Creates a new `Zobrist` with random values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::Zobrist;
    /// let zobrist = Zobrist::new();
    /// ```
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut zobrist = Zobrist::default();

        zobrist.color = rng.gen();

        for square in 0..SQUARES {
            for color in 0..Color::LEN {
                for piece_type in 0..PIECE_TYPES {
                    zobrist.pieces[square][color][piece_type] = rng.gen();
                }
            }
        }

        for color in 0..Color::LEN {
            for file in 0..File::LEN {
                zobrist.en_passant[color][file] = rng.gen();
            }
        }

        for color in 0..Color::LEN {
            for castle_rights in 0..CASTLE_RIGHTS_TYPES {
                zobrist.castling_rights[color][castle_rights] = rng.gen();
            }
        }

        zobrist
    }

    /// Returns the hash for the color to move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::ZOBRIST;
    /// let color = ZOBRIST.color();
    /// ```
    pub fn color(&self) -> u64 {
        self.color
    }

    /// Returns the hash for a [`Piece`] on a [`Square`].
    ///
    /// # Panics
    ///
    /// Panics if the [`Square`] is not a legal square.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{ZOBRIST, Square, Piece, Color, PieceType};
    /// let square: Square = "a2".parse().unwrap();
    /// let piece = Piece::new(PieceType::Pawn, Color::White);
    /// let hash = ZOBRIST.piece(square, piece);
    /// ```
    pub fn piece(&self, square: Square, piece: Piece) -> u64 {
        assert!(square.is_valid());

        self.pieces[square.0 as usize][piece.color() as usize][piece.piece_type() as usize]
    }

    /// Returns the hash for en passant on a [`Square`].
    ///
    /// # Panics
    ///
    /// Panics if the [`Square`] is not a legal square.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{ZOBRIST, Square};
    /// let square: Square = "a3".parse().unwrap();
    /// let hash = ZOBRIST.en_passant(square);
    /// ```
    pub fn en_passant(&self, en_passant_square: Square) -> u64 {
        assert!(en_passant_square.rank() == Rank::Three || en_passant_square.rank() == Rank::Six);

        self.en_passant[(en_passant_square.rank() == Rank::Three) as usize]
            [en_passant_square.rank() as usize]
    }

    /// Returns the hash for castling rights for a [`Color`] and [`CastleRightsType`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{ZOBRIST, Color, CastleRightsType};
    /// let hash = ZOBRIST.castling_rights(Color::White, CastleRightsType::Both);
    /// ```
    pub fn castling_rights(&self, color: Color, castle_rights_type: CastleRightsType) -> u64 {
        self.castling_rights[color as usize][castle_rights_type as usize]
    }
}

/// Default implementation for `Zobrist`.
///
/// Creates a new `Zobrist` with 0 values.
///
/// # Examples
///
/// ```
/// # use chess_engine::Zobrist;
/// let zobrist = Zobrist::default();
/// assert_eq!(zobrist.color(), 0);
/// ```
impl Default for Zobrist {
    fn default() -> Self {
        Self {
            color: 0,
            pieces: [[[0; PIECE_TYPES]; Color::LEN]; SQUARES],
            en_passant: [[0; File::LEN]; Color::LEN],
            castling_rights: [[0; CASTLE_RIGHTS_TYPES]; Color::LEN],
        }
    }
}
