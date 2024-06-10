use super::{Board, CastleRights};
use crate::{Piece, Player, Square, State, SQUARES};

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
/// builder.player("b".parse().unwrap());
///
/// let board = builder.build();
/// ```
#[derive(Clone, Copy, Debug)]
pub struct BoardBuilder {
    pub(super) pieces: [Option<Piece>; SQUARES],
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
    /// # use chess_engine::{BoardBuilder, Piece, SQUARES};
    /// let mut builder = BoardBuilder::new();
    ///
    /// let mut pieces = [None; SQUARES];
    /// pieces[0] = Some("R".parse().unwrap());
    /// pieces[63] = Some("r".parse().unwrap());
    ///
    /// builder.pieces(pieces);
    /// ```
    pub fn pieces(&mut self, pieces: [Option<Piece>; SQUARES]) -> &mut BoardBuilder {
        self.pieces = pieces;
        self
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
    /// let board = Board::builder()
    ///     .put_piece("R".parse().unwrap(), "a1".parse().unwrap())
    ///     .build();
    /// ```
    pub fn put_piece(&mut self, piece: Piece, square: Square) -> &mut BoardBuilder {
        assert!(square.is_valid());

        self.pieces[square.0 as usize] = Some(piece);
        self
    }

    /// Sets the player to move.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{BoardBuilder, Player};
    /// let mut builder = BoardBuilder::new();
    /// builder.player("b".parse().unwrap());
    /// ```
    pub fn player(&mut self, player: Player) -> &mut BoardBuilder {
        self.state.set_player(player);
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
    /// # use chess_engine::{BoardBuilder, Player, CastleRights};
    /// let mut builder = BoardBuilder::new();
    /// builder.player("b".parse().unwrap());
    /// builder.castling_rights(CastleRights::default());
    /// let board = builder.build();
    pub fn build(self) -> Board {
        let mut board = Board {
            piece_types_bitboards: Default::default(),
            player_bitboards: Default::default(),
            state: self.state,
        };

        for (square, piece) in self.pieces.iter().enumerate() {
            if let Some(piece) = piece {
                board.put_piece(*piece, Square(square as u8));
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
            pieces: [None; SQUARES],
            state: State::default(),
        }
    }
}
