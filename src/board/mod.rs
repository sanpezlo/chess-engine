//! The `Board` type for chessboard representation.

mod board_builder;
mod castle_rights;
pub mod fen;

use crate::{BitBoard, Color, Piece, PieceType, Player, Square, PIECE_TYPES, PLAYERS};
pub use board_builder::*;
pub use castle_rights::*;

/// Chessboard representation.
///
/// For building a board, use the [`Board::builder()`] method or the
/// [`BoardBuilder`] struct.
///
/// # Examples
///
/// ```
/// # use chess_engine::Board;
/// let board = Board::builder().build();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board {
    player: Player,
    piece_types_bitboards: [BitBoard; PIECE_TYPES],
    player_bitboards: [BitBoard; PLAYERS],
    castling_rights: CastleRights,
    en_passant_square: Option<Square>,
    halfmove_clock: u8,
    fullmove_counter: u16,
}

/// Getters for the `Board` struct.
impl Board {
    /// Returns the [`Player`] to move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Board, Player, Color};
    /// let board = Board::default();
    /// assert_eq!(board.player(), Player(Color::White));
    /// ```
    pub fn player(&self) -> Player {
        self.player
    }

    /// Returns the [`BitBoard`] for a specific [`PieceType`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Board, BitBoard, PieceType};
    /// let board = Board::default();
    /// assert_eq!(
    ///     board.piece_types_bitboard(PieceType::Pawn),
    ///     BitBoard(0x00FF00000000FF00)
    /// );
    /// ```
    pub fn piece_types_bitboard(&self, piece_type: PieceType) -> BitBoard {
        self.piece_types_bitboards[piece_type as usize]
    }

    /// Returns the [`BitBoard`] for a specific [`Player`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Board, BitBoard, Color};
    /// let board = Board::default();
    /// assert_eq!(
    ///     board.player_bitboard(Color::White),
    ///     BitBoard(0x000000000000FFFF)
    /// );
    pub fn player_bitboard(&self, color: Color) -> BitBoard {
        self.player_bitboards[color as usize]
    }

    /// Returns the [`CastleRights`] for the board.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Board, CastleRights};
    /// let board = Board::default();
    /// assert_ne!(board.castling_rights(), CastleRights::default());
    /// ```
    pub fn castling_rights(&self) -> CastleRights {
        self.castling_rights
    }

    /// Returns the En Passant [`Square`] if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Board, Square};
    /// let board = Board::default();
    /// assert_eq!(board.en_passant_square(), None);
    /// ```
    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    /// Returns the halfmove clock.
    ///
    /// The halfmove clock is the number of halfmoves since the last capture or pawn move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::Board;
    /// let board = Board::default();
    /// assert_eq!(board.halfmove_clock(), 0);
    /// ```
    pub fn halfmove_clock(&self) -> u8 {
        self.halfmove_clock
    }

    /// Returns the fullmove counter.
    ///
    /// The fullmove counter is the number of the full move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::Board;
    /// let board = Board::default();
    /// assert_eq!(board.fullmove_counter(), 1);
    /// ```
    pub fn fullmove_counter(&self) -> u16 {
        self.fullmove_counter
    }

    /// Returns the [`BitBoard`] for a specific [`Piece`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Board, BitBoard, Piece, Color, PieceType, Player};
    /// let board = Board::default();
    /// let piece = Piece::new(PieceType::Pawn, Player(Color::White));
    /// assert_eq!(board.piece_bitboard(piece), BitBoard(0x000000000000FF00));
    /// ```
    pub fn piece_bitboard(&self, piece: Piece) -> BitBoard {
        self.piece_types_bitboards[piece.piece_type() as usize]
            & self.player_bitboards[piece.player().0 as usize]
    }
}

impl Board {
    /// Returns a new [`BoardBuilder`] to build a [`Board`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::Board;
    /// let board = Board::builder().build();
    /// ```
    pub fn builder() -> BoardBuilder {
        BoardBuilder::new()
    }

    /// Puts a [`Piece`] on a [`Square`] on the board.
    ///
    /// # Panics
    ///
    /// Panics if the [`Square`] is not a legal square.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Board};
    /// let mut board = Board::default();
    /// board.put_piece("R".parse().unwrap(), "a1".parse().unwrap());
    /// ```
    pub fn put_piece(&mut self, piece: Piece, square: Square) {
        assert!(square.is_valid());

        let piece_type = piece.piece_type() as usize;
        let player = piece.player().0 as usize;

        self.piece_types_bitboards[piece_type] |= square.into();
        self.player_bitboards[player] |= square.into();
    }

    /// Returns `true` if the [`Player`] has the bishop pair.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Board, Color};
    /// let board = Board::default();
    /// assert_eq!(board.has_bishop_pair(Color::White), true);
    /// ```
    pub fn has_bishop_pair(&self, color: Color) -> bool {
        let mut bitboard = self.piece_bitboard(Piece::new(PieceType::Bishop, Player(color)));

        let mut white_square = 0;
        let mut black_square = 0;

        if bitboard.0.count_ones() >= 2 {
            while bitboard.0 != 0 {
                let trailing_zeros = bitboard.0.trailing_zeros();
                let square = Square(trailing_zeros as u8);
                if square.color() == Color::White {
                    white_square += 1;
                } else {
                    black_square += 1;
                }

                bitboard ^= BitBoard(1 << trailing_zeros);
            }
        }

        white_square >= 1 && black_square >= 1
    }

    /// Returns `true` if the board has sufficient material to force checkmate.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Board};
    /// let board = Board::default();
    /// assert_eq!(board.has_sufficient_material(), true);
    /// ```
    pub fn has_sufficient_material(&self) -> bool {
        for color in [Color::White, Color::Black] {
            if self.piece_bitboard(Piece::new(PieceType::Queen, Player(color))) > BitBoard(0) {
                return true;
            }

            if self.piece_bitboard(Piece::new(PieceType::Rook, Player(color))) > BitBoard(0) {
                return true;
            }

            if self.piece_bitboard(Piece::new(PieceType::Pawn, Player(color))) > BitBoard(0) {
                return true;
            }

            if self.has_bishop_pair(color) {
                return true;
            }

            if self.piece_bitboard(Piece::new(PieceType::Bishop, Player(color))) > BitBoard(0)
                && self.piece_bitboard(Piece::new(PieceType::Knight, Player(color))) > BitBoard(0)
            {
                return true;
            }

            if self
                .piece_bitboard(Piece::new(PieceType::Knight, Player(color)))
                .0
                .count_ones()
                >= 3
            {
                return true;
            }
        }

        return false;
    }
}

/// Default implementation for the `Board` struct.
///
/// The default board is the starting position of a chess game.
///
/// # Examples
///
/// ```
/// # use chess_engine::{Board, Player, Color};
/// let board = Board::default();
/// assert_eq!(board.player(), Player(Color::White));
/// ```
impl Default for Board {
    fn default() -> Self {
        Self {
            player: Player(Color::White),
            piece_types_bitboards: [
                BitBoard(0x00FF00000000FF00),
                BitBoard(0x4200000000000042),
                BitBoard(0x2400000000000024),
                BitBoard(0x8100000000000081),
                BitBoard(0x0800000000000008),
                BitBoard(0x1000000000000010),
            ],
            player_bitboards: [BitBoard(0x000000000000FFFF), BitBoard(0xFFFF000000000000)],
            castling_rights: CastleRights([CastleRightsType::Both; 2]),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_counter: 1,
        }
    }
}
