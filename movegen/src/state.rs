use chess_engine_core::{Color, Rank, Square};

use crate::{CastleRights, ZOBRIST};

/// Represents the state of the chessboard.
///
/// # Examples
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let state = State::default();
/// assert_eq!(state.color(), Color::White);
/// ```
#[derive(Clone, Copy, Debug)]
pub struct State {
    color: Color,
    castling_rights: CastleRights,
    en_passant_square: Option<Square>,
    halfmove_clock: u8,
    fullmove_counter: u16,
    hash: u64,
}

/// Getters and setters for the `State` struct.
impl State {
    /// Maximum number of halfmoves before a draw.
    pub const MAX_HALFMOVE_CLOCK: u8 = 100;

    /// Returns the [`Color`] to move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let state = State::default();
    /// assert_eq!(state.color(), Color::White);
    /// ```
    pub fn color(&self) -> Color {
        self.color
    }

    /// Returns the [`CastleRights`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let state = State::default();
    /// assert_eq!(state.castling_rights(), CastleRights::default());
    /// ```
    pub fn castling_rights(&self) -> CastleRights {
        self.castling_rights
    }

    /// Returns the en passant [`Square`] if it exists.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let state = State::default();
    /// assert_eq!(state.en_passant_square(), None);
    /// ```
    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    /// Returns the halfmove clock.
    ///
    /// The halfmove clock is the number of halfmoves since the last capture or
    /// pawn move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let state = State::default();
    /// assert_eq!(state.halfmove_clock(), 0);
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
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let state = State::default();
    /// assert_eq!(state.fullmove_counter(), 1);
    /// ```
    pub fn fullmove_counter(&self) -> u16 {
        self.fullmove_counter
    }

    /// Returns the hash of the state.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let state = State::default();
    /// let hash = state.hash();
    /// assert_eq!(hash, 0);
    /// ```
    pub fn hash(&self) -> u64 {
        self.hash
    }

    /// Sets the color to move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let mut state = State::default();
    /// state.set_color(Color::Black);
    /// assert_eq!(state.color(), Color::Black);
    /// ```
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    /// Sets the castling rights.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let mut state = State::default();
    /// state.set_castling_rights(CastleRights([CastleRightsType::Both; 2]));
    /// assert_eq!(state.castling_rights(), CastleRights([CastleRightsType::Both; 2]));
    /// ```
    pub fn set_castling_rights(&mut self, castling_rights: CastleRights) {
        self.castling_rights = castling_rights;
    }

    /// Sets the en passant square.
    ///
    /// # Panics
    ///
    /// Panics if the en passant square is not on the third or sixth rank
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let mut state = State::default();
    /// state.set_en_passant_square(Some("e3".parse().unwrap()));
    /// assert_eq!(state.en_passant_square(), Some(Square::E3));
    /// ```
    pub fn set_en_passant_square(&mut self, en_passant_square: Option<Square>) {
        assert!(
            en_passant_square.is_none()
                || en_passant_square.unwrap().rank() == Rank::Three
                || en_passant_square.unwrap().rank() == Rank::Six
        );

        self.en_passant_square = en_passant_square;
    }

    /// Sets the halfmove clock.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let mut state = State::default();
    /// state.set_halfmove_clock(50);
    /// assert_eq!(state.halfmove_clock(), 50);
    /// ```
    pub fn set_halfmove_clock(&mut self, halfmove_clock: u8) {
        self.halfmove_clock = halfmove_clock;
    }

    /// Sets the fullmove counter.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let mut state = State::default();
    /// state.set_fullmove_counter(50);
    /// assert_eq!(state.fullmove_counter(), 50);
    /// ```
    pub fn set_fullmove_counter(&mut self, fullmove_counter: u16) {
        self.fullmove_counter = fullmove_counter;
    }

    /// Sets the hash of the state.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let mut state = State::default();
    /// state.set_hash(50);
    /// assert_eq!(state.hash(), 50);
    /// ```
    pub fn set_hash(&mut self, hash: u64) {
        self.hash = hash;
    }
}

impl State {
    /// Creates a new `State` struct.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let state = State::new(
    ///     Color::White,
    ///     CastleRights::default(),
    ///     None,
    ///     0,
    ///     1
    ///  );
    pub fn new(
        color: Color,
        castling_rights: CastleRights,
        en_passant_square: Option<Square>,
        halfmove_clock: u8,
        fullmove_counter: u16,
    ) -> Self {
        Self {
            color,
            castling_rights,
            en_passant_square,
            halfmove_clock,
            fullmove_counter,
            hash: 0,
        }
    }

    /// Returns the partial hash of the state.
    ///
    /// This is a partial hash because it does not include the piece positions.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine_movegen::*;
    /// # use chess_engine_core::*;
    /// let state = State::default();
    /// let hash = state.partial_hash();
    /// ```
    pub fn partial_hash(&self) -> u64 {
        let mut hash: u64 = 0;

        if self.color == Color::White {
            hash ^= ZOBRIST.color()
        }

        for color in 0..Color::LEN {
            hash ^= ZOBRIST.castling_rights(Color::new(color), self.castling_rights.0[color]);
        }

        if let Some(en_passant_square) = self.en_passant_square {
            hash ^= ZOBRIST.en_passant(en_passant_square);
        }

        hash
    }
}

/// Default implementation for the `State` struct.
///
/// Creates a new `State` with the default values.
///
/// # Examples
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let state = State::default();
/// ```
impl Default for State {
    fn default() -> Self {
        Self {
            color: Color::default(),
            castling_rights: CastleRights::default(),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_counter: 1,
            hash: 0,
        }
    }
}
