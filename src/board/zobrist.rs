use once_cell::sync::Lazy;
use rand::prelude::*;

use crate::{
    CastleRightsType, Color, Piece, Rank, Square, CASTLE_RIGHTS_TYPES, FILES, PIECE_TYPES, PLAYERS,
    SQUARES,
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
    player: u64,
    pieces: [[[u64; PIECE_TYPES]; PLAYERS]; SQUARES],
    en_passant: [[u64; FILES]; PLAYERS],
    castling_rights: [[u64; CASTLE_RIGHTS_TYPES]; PLAYERS],
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

        zobrist.player = rng.gen();

        for square in 0..SQUARES {
            for player in 0..PLAYERS {
                for piece_type in 0..PIECE_TYPES {
                    zobrist.pieces[square][player][piece_type] = rng.gen();
                }
            }
        }

        for player in 0..PLAYERS {
            for file in 0..FILES {
                zobrist.en_passant[player][file] = rng.gen();
            }
        }

        for player in 0..PLAYERS {
            for castle_rights in 0..CASTLE_RIGHTS_TYPES {
                zobrist.castling_rights[player][castle_rights] = rng.gen();
            }
        }

        zobrist
    }

    /// Returns the hash for the player to move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::ZOBRIST;
    /// let player = ZOBRIST.player();
    /// ```
    pub fn player(&self) -> u64 {
        self.player
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
    /// # use chess_engine::{ZOBRIST, Square, Piece, Color, PieceType, Player};
    /// let square: Square = "a2".parse().unwrap();
    /// let piece = Piece::new(PieceType::Pawn, Player(Color::White));
    /// let hash = ZOBRIST.piece(square, piece);
    /// ```
    pub fn piece(&self, square: Square, piece: Piece) -> u64 {
        assert!(square.is_valid());

        self.pieces[square.0 as usize][piece.player().0 as usize][piece.piece_type() as usize]
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
    pub fn castling_rights(
        &self,
        player_color: Color,
        castle_rights_type: CastleRightsType,
    ) -> u64 {
        self.castling_rights[player_color as usize][castle_rights_type as usize]
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
/// assert_eq!(zobrist.player(), 0);
/// ```
impl Default for Zobrist {
    fn default() -> Self {
        Self {
            player: 0,
            pieces: [[[0; PIECE_TYPES]; PLAYERS]; SQUARES],
            en_passant: [[0; FILES]; PLAYERS],
            castling_rights: [[0; CASTLE_RIGHTS_TYPES]; PLAYERS],
        }
    }
}
