use super::{Board, CastleRights};
use crate::{Color, Piece, Square, State};

/// A builder for creating a [`Board`].
///
/// # Examples
///
/// ```
/// # use chess_engine::{BoardBuilder};
/// let mut builder = BoardBuilder::new();
///
/// builder.put_piece("R".parse().unwrap(), "a1".parse().unwrap());
/// builder.put_piece("r".parse().unwrap(), "h8".parse().unwrap());
/// builder.color("b".parse().unwrap());
///
/// let board = builder.build();
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoardBuilder {
    pub(super) pieces: [Option<Piece>; Square::LEN],
    pub(super) state: State,
}

impl BoardBuilder {
    /// Creates a new `BoardBuilder`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::BoardBuilder;
    /// let builder = BoardBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the pieces on the board.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{BoardBuilder, Piece, Square};
    /// let mut builder = BoardBuilder::new();
    ///
    /// let mut pieces = [None; Square::LEN];
    /// pieces[0] = Some("R".parse().unwrap());
    /// pieces[63] = Some("r".parse().unwrap());
    ///
    /// builder.pieces(pieces);
    /// ```
    pub fn pieces(&mut self, pieces: [Option<Piece>; Square::LEN]) -> &mut BoardBuilder {
        self.pieces = pieces;
        self
    }

    /// Puts a [`Piece`] on a [`Square`] on the board.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Board};
    /// let board = Board::builder()
    ///     .put_piece("R".parse().unwrap(), "a1".parse().unwrap())
    ///     .build();
    /// ```
    pub fn put_piece(&mut self, piece: Piece, square: Square) -> &mut BoardBuilder {
        self.pieces[square as usize] = Some(piece);
        self
    }

    /// Sets the color to move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{BoardBuilder, Color};
    /// let mut builder = BoardBuilder::new();
    /// builder.color("b".parse().unwrap());
    /// ```
    pub fn color(&mut self, color: Color) -> &mut BoardBuilder {
        self.state.set_color(color);
        self
    }

    /// Sets the castling rights.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{BoardBuilder, CastleRights};
    /// let mut builder = BoardBuilder::new();
    /// builder.castling_rights(CastleRights::default());
    /// ```
    pub fn castling_rights(&mut self, castling_rights: CastleRights) -> &mut BoardBuilder {
        self.state.set_castling_rights(castling_rights);
        self
    }

    /// Sets the en passant square.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{BoardBuilder, Square};
    /// let mut builder = BoardBuilder::new();
    /// builder.en_passant_square(Some("e3".parse().unwrap()));
    /// ```
    pub fn en_passant_square(&mut self, en_passant_square: Option<Square>) -> &mut BoardBuilder {
        self.state.set_en_passant_square(en_passant_square);
        self
    }

    /// Sets the halfmove clock.
    ///
    /// The halfmove clock is the number of halfmoves since the last capture or
    /// pawn move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{BoardBuilder};
    /// let mut builder = BoardBuilder::new();
    /// builder.halfmove_clock(50);
    /// ```
    pub fn halfmove_clock(&mut self, halfmove_clock: u8) -> &mut BoardBuilder {
        self.state.set_halfmove_clock(halfmove_clock);
        self
    }

    /// Sets the fullmove counter.
    ///
    /// The fullmove counter is the number of the full move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{BoardBuilder};
    /// let mut builder = BoardBuilder::new();
    /// builder.fullmove_counter(1);
    /// ```
    pub fn fullmove_counter(&mut self, fullmove_counter: u16) -> &mut BoardBuilder {
        assert!(fullmove_counter > 0);

        self.state.set_fullmove_counter(fullmove_counter);
        self
    }

    /// Builds the [`Board`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{BoardBuilder, Color, CastleRights};
    /// let mut builder = BoardBuilder::new();
    /// builder.color("b".parse().unwrap());
    /// builder.castling_rights(CastleRights::default());
    /// let board = builder.build();
    pub fn build(self) -> Board {
        let mut board = Board {
            piece_types_bitboards: Default::default(),
            color_bitboards: Default::default(),
            state: self.state,
            history: Vec::with_capacity(Board::AVERAGE_MOVES),
        };

        for (square, piece) in self.pieces.iter().enumerate() {
            if let Some(piece) = piece {
                board.put_piece(*piece, Square::new(square));
            }
        }

        board
    }
}

/// Default implementation for `BoardBuilder`, empty board with white to move
/// and no castling rights.
///
/// # Examples
///
/// ```
/// # use chess_engine::BoardBuilder;
/// let builder = BoardBuilder::default();
/// ```
impl Default for BoardBuilder {
    fn default() -> Self {
        BoardBuilder {
            pieces: [None; Square::LEN],
            state: State::default(),
        }
    }
}
