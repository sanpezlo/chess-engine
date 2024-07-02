use crate::{BitBoard, Board, Color, Piece, PieceType, Player, MAX_HALFMOVE_CLOCK, PLAYERS};

impl Board {
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
        let bitboard = self.piece_bitboard(Piece::new(PieceType::Bishop, Player(color)));

        let mut white_square = 0;
        let mut black_square = 0;

        for square in bitboard {
            if square.color() == Color::White {
                white_square += 1;
            } else {
                black_square += 1;
            }
        }

        white_square >= 1 && black_square >= 1
    }

    /// Returns `true` if the game is a draw by insufficient material.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Board};
    /// let board = Board::default();
    /// assert_eq!(board.draw_by_insufficient_material(), false);
    /// ```
    pub fn draw_by_insufficient_material(&self) -> bool {
        for player_color in 0..PLAYERS {
            if self.piece_bitboard(Piece::new(
                PieceType::Queen,
                Player(Color::new(player_color)),
            )) > BitBoard(0)
            {
                return false;
            }

            if self.piece_bitboard(Piece::new(
                PieceType::Rook,
                Player(Color::new(player_color)),
            )) > BitBoard(0)
            {
                return false;
            }

            if self.piece_bitboard(Piece::new(
                PieceType::Pawn,
                Player(Color::new(player_color)),
            )) > BitBoard(0)
            {
                return false;
            }

            if self.has_bishop_pair(Color::new(player_color)) {
                return false;
            }

            if self.piece_bitboard(Piece::new(
                PieceType::Bishop,
                Player(Color::new(player_color)),
            )) > BitBoard(0)
                && self.piece_bitboard(Piece::new(
                    PieceType::Knight,
                    Player(Color::new(player_color)),
                )) > BitBoard(0)
            {
                return false;
            }

            if self
                .piece_bitboard(Piece::new(
                    PieceType::Knight,
                    Player(Color::new(player_color)),
                ))
                .0
                .count_ones()
                >= 3
            {
                return false;
            }
        }

        let w_bishops = self.piece_bitboard(Piece::new(PieceType::Bishop, Player(Color::White)));

        let w_knights = self.piece_bitboard(Piece::new(PieceType::Knight, Player(Color::White)));

        let b_bishops = self.piece_bitboard(Piece::new(PieceType::Bishop, Player(Color::Black)));

        let b_knights = self.piece_bitboard(Piece::new(PieceType::Knight, Player(Color::Black)));

        if w_bishops.0.count_ones() != 0 && b_bishops.0.count_ones() != 0 {
            let mut w = false;
            let mut b = false;
            for square in w_bishops {
                if square.color() == Color::White {
                    w = true
                } else {
                    b = true
                }
            }

            for square in b_bishops {
                if square.color() == Color::White {
                    w = true
                } else {
                    b = true
                }
            }

            if w && b {
                return false;
            }
        }

        if w_knights.0.count_ones() != 0 && b_knights.0.count_ones() != 0 {
            return false;
        }

        return true;
    }

    /// Returns `true` if the game is a draw by the fifty moves rule.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Board};
    /// let board = Board::default();
    /// assert_eq!(board.draw_by_fifty_moves(), false);
    /// ```
    pub fn draw_by_fifty_moves(&self) -> bool {
        self.halfmove_clock() >= MAX_HALFMOVE_CLOCK
    }

    /// Returns `true` if the game is a draw by threefold repetition.
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::{Board};
    /// let board = Board::default();
    /// assert_eq!(board.draw_by_repetition(), false);
    /// ```
    pub fn draw_by_repetition(&self) -> bool {
        let mut repetitions = 0;

        let hash = self.hash();

        for state in self.history.iter().rev() {
            if state.halfmove_clock() == 0 {
                break;
            }

            if state.hash() == hash {
                repetitions += 1;
            }
        }

        repetitions >= 2
    }
}
