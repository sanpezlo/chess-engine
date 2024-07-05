use chess_engine_core::{BitBoard, CastleRightsType, Color, Piece, PieceType, Square};

use crate::{BoardBuilder, CastleRights, State, ZOBRIST};

/// Chessboard representation.
///
/// For building a board, use the [`Board::builder()`] method or the
/// [`BoardBuilder`] struct.
///
/// # Examples
///
/// ```
/// # use chess_engine_movegen::*;
/// let board = Board::builder().build();
/// ```
#[derive(Clone, Debug)]
pub struct Board {
    pub(crate) piece_types_bitboards: [BitBoard; PieceType::LEN],
    pub(crate) color_bitboards: [BitBoard; Color::LEN],
    pub(crate) state: State,
    pub(crate) history: Vec<State>,
}

/// Getters for the `Board` struct.
impl Board {
    /// Average number of moves in a game.
    pub const AVERAGE_MOVES: usize = 79;

    /// Returns the [`BitBoard`] for a specific [`PieceType`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// let board = Board::default();
    /// assert_eq!(
    ///     board.piece_types_bitboard(PieceType::Pawn),
    ///     BitBoard(0x00FF00000000FF00)
    /// );
    /// ```
    pub fn piece_types_bitboard(&self, piece_type: PieceType) -> BitBoard {
        self.piece_types_bitboards[piece_type as usize]
    }

    /// Returns the [`BitBoard`] for a specific [`Color`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// let board = Board::default();
    /// assert_eq!(
    ///     board.color_bitboard(Color::White),
    ///     BitBoard(0x000000000000FFFF)
    /// );
    pub fn color_bitboard(&self, color: Color) -> BitBoard {
        self.color_bitboards[color as usize]
    }

    /// Returns the [`BitBoard`] for a specific [`Piece`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// let board = Board::default();
    /// let piece = Piece::new(PieceType::Pawn, Color::White);
    /// assert_eq!(board.piece_bitboard(piece), BitBoard(0x000000000000FF00));
    /// ```
    pub fn piece_bitboard(&self, piece: Piece) -> BitBoard {
        self.piece_types_bitboards[piece.piece_type() as usize]
            & self.color_bitboards[piece.color() as usize]
    }

    /// Returns the [`Color`] to move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// let board = Board::default();
    /// assert_eq!(board.color(), Color::White);
    /// ```
    pub fn color(&self) -> Color {
        self.state.color()
    }

    /// Returns the [`CastleRights`] for the board.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// let board = Board::default();
    /// assert_ne!(board.castling_rights(), CastleRights::default());
    /// ```
    pub fn castling_rights(&self) -> CastleRights {
        self.state.castling_rights()
    }

    /// Returns the En Passant [`Square`] if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// let board = Board::default();
    /// assert_eq!(board.en_passant_square(), None);
    /// ```
    pub fn en_passant_square(&self) -> Option<Square> {
        self.state.en_passant_square()
    }

    /// Returns the halfmove clock.
    ///
    /// The halfmove clock is the number of halfmoves since the last capture or pawn move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// let board = Board::default();
    /// assert_eq!(board.halfmove_clock(), 0);
    /// ```
    pub fn halfmove_clock(&self) -> u8 {
        self.state.halfmove_clock()
    }

    /// Returns the fullmove counter.
    ///
    /// The fullmove counter is the number of the full move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// let board = Board::default();
    /// assert_eq!(board.fullmove_counter(), 1);
    /// ```
    pub fn fullmove_counter(&self) -> u16 {
        self.state.fullmove_counter()
    }

    /// Returns the history of the board.
    /// The history is a vector of [`State`] structs.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// let board = Board::default();
    /// assert_eq!(board.history().len(), 0);
    /// ```
    pub fn history(&self) -> &Vec<State> {
        &self.history
    }
}

impl Board {
    /// Returns a new [`BoardBuilder`] to build a [`Board`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// let board = Board::builder().build();
    /// ```
    pub fn builder() -> BoardBuilder {
        BoardBuilder::new()
    }

    /// Puts a [`Piece`] on a [`Square`] on the board.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// let mut board = Board::default();
    /// board.put_piece("R".parse().unwrap(), "a1".parse().unwrap());
    /// ```
    pub fn put_piece(&mut self, piece: Piece, square: Square) {
        let piece_type = piece.piece_type() as usize;
        let color = piece.color() as usize;

        self.piece_types_bitboards[piece_type] =
            self.piece_types_bitboards[piece_type].set_square(square);
        self.color_bitboards[color] = self.color_bitboards[color].set_square(square);
    }

    /// Returns the hash of the board.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// let board = Board::default();
    /// let hash = board.hash();
    /// ```
    pub fn hash(&self) -> u64 {
        let mut hash = self.state.partial_hash();

        for piece_type in 0..PieceType::LEN {
            let piece_type = PieceType::new(piece_type);
            for color in 0..Color::LEN {
                let piece = Piece::new(piece_type, Color::new(color));

                for square in self.piece_bitboard(piece) {
                    hash ^= ZOBRIST.piece(square, piece);
                }
            }
        }

        hash
    }
}

/// Default implementation for the `Board` struct.
///
/// The default board is the starting position of a chess game.
///
/// # Examples
///
/// ```
/// # use chess_engine_movegen::*;
/// let board = Board::default();
/// assert_eq!(board.color(), Color::White);
/// ```
impl Default for Board {
    fn default() -> Self {
        Self {
            piece_types_bitboards: [
                BitBoard(0x00FF00000000FF00),
                BitBoard(0x4200000000000042),
                BitBoard(0x2400000000000024),
                BitBoard(0x8100000000000081),
                BitBoard(0x0800000000000008),
                BitBoard(0x1000000000000010),
            ],
            color_bitboards: [BitBoard(0x000000000000FFFF), BitBoard(0xFFFF000000000000)],
            state: State::new(
                Color::White,
                CastleRights([CastleRightsType::Both; 2]),
                None,
                0,
                1,
            ),
            history: Vec::with_capacity(Self::AVERAGE_MOVES),
        }
    }
}
