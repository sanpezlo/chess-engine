use chess_engine_core::{
    BitBoard, CastleRightsType, Color, Move, Piece, PieceMoves, PieceType, Rank, Square,
};

use crate::{
    get_bishop_attacks, get_king_attacks, get_knight_attacks, get_pawn_attacks, get_pawn_moves,
    get_queen_attacks, get_rook_attacks, Board,
};

impl Board {
    /// Returns true if the square is attacked by the color.
    pub fn is_square_attcked(&self, square: Square, color: Color) -> bool {
        if get_pawn_attacks(!color, square)
            & self.piece_bitboard(Piece::new(PieceType::Pawn, color))
            != BitBoard::EMPTY
        {
            return true;
        }

        if get_knight_attacks(square) & self.piece_bitboard(Piece::new(PieceType::Knight, color))
            != BitBoard::EMPTY
        {
            return true;
        }

        if get_bishop_attacks(square, self.both_bitboard())
            & self.piece_bitboard(Piece::new(PieceType::Bishop, color))
            != BitBoard::EMPTY
        {
            return true;
        }

        if get_rook_attacks(square, self.both_bitboard())
            & self.piece_bitboard(Piece::new(PieceType::Rook, color))
            != BitBoard::EMPTY
        {
            return true;
        }

        if get_queen_attacks(square, self.both_bitboard())
            & self.piece_bitboard(Piece::new(PieceType::Queen, color))
            != BitBoard::EMPTY
        {
            return true;
        }

        if get_king_attacks(square) & self.piece_bitboard(Piece::new(PieceType::King, color))
            != BitBoard::EMPTY
        {
            return true;
        }

        false
    }

    /// Returns the bitboard of all squares attacked by the color.
    pub fn attacked_bitboard(&self, color: Color) -> BitBoard {
        let mut attacked = BitBoard::EMPTY;

        for square in Square::ALL {
            if self.is_square_attcked(square, color) {
                attacked = attacked.set_square(square);
            }
        }

        attacked
    }

    fn generate_piece_moves(&self, piece_type: PieceType, square: Square) -> Option<PieceMoves> {
        if !self
            .piece_bitboard(Piece::new(piece_type, self.color()))
            .is_get_square(square)
        {
            return None;
        }

        let to = match piece_type {
            PieceType::Pawn => {
                let color = self.color();
                let mut blockers = self.both_bitboard();

                // pawn moves

                match color {
                    Color::White => {
                        blockers |= BitBoard((Rank::Three.bitboard() & blockers).0 << 8);
                    }
                    Color::Black => {
                        blockers |= BitBoard((Rank::Six.bitboard() & blockers).0 >> 8);
                    }
                }

                let pawn_moves = get_pawn_moves(color, square).unset_bit(blockers.0);

                // pawn attacks

                let pawn_attacks = get_pawn_attacks(color, square) & self.opponent_bitboard();

                let mut en_passant_capture = BitBoard::EMPTY;

                if let Some(en_passant) = self.en_passant_square() {
                    if get_pawn_attacks(color, square).is_get_square(en_passant) {
                        en_passant_capture = en_passant.bitboard();
                    }
                }

                pawn_moves
                    .set_bit(pawn_attacks.0)
                    .set_bit(en_passant_capture.0)
            }
            PieceType::Knight => {
                // knight moves
                get_knight_attacks(square) & !self.ally_bitboard()
            }
            PieceType::Bishop => {
                // bishop moves
                get_bishop_attacks(square, self.both_bitboard()) & !self.ally_bitboard()
            }
            PieceType::Rook => {
                // rook moves
                get_rook_attacks(square, self.both_bitboard()) & !self.ally_bitboard()
            }
            PieceType::Queen => {
                // queen moves
                get_queen_attacks(square, self.both_bitboard()) & !self.ally_bitboard()
            }
            PieceType::King => {
                let color = self.color();

                // king moves

                let king_moves = get_king_attacks(square)
                    & !self.ally_bitboard()
                    & !self.attacked_bitboard(!color);

                // castle moves

                let mut castle_rights = BitBoard::EMPTY;

                let castle_rights_type = self.castling_rights().0[color as usize];

                if castle_rights_type as usize & CastleRightsType::KingSide as usize != 0 {
                    let squares = match color {
                        Color::White => [Square::F1, Square::G1],
                        Color::Black => [Square::F8, Square::G8],
                    };

                    if !self.both_bitboard().is_get_square(squares[0])
                        && !self.both_bitboard().is_get_square(squares[1])
                    {
                        let squares = match color {
                            Color::White => [Square::E1, Square::F1, Square::G1],
                            Color::Black => [Square::E8, Square::F8, Square::G8],
                        };

                        if !self.is_square_attcked(squares[0], !color)
                            && !self.is_square_attcked(squares[1], !color)
                            && !self.is_square_attcked(squares[2], !color)
                        {
                            castle_rights |= squares[2];
                        }
                    }
                }

                if castle_rights_type as usize & CastleRightsType::QueenSide as usize != 0 {
                    let squares = match color {
                        Color::White => [Square::D1, Square::C1, Square::B1],
                        Color::Black => [Square::D8, Square::C8, Square::B8],
                    };

                    if !self.both_bitboard().is_get_square(squares[0])
                        && !self.both_bitboard().is_get_square(squares[1])
                        && !self.both_bitboard().is_get_square(squares[2])
                    {
                        let squares = match color {
                            Color::White => [Square::E1, Square::D1, Square::C1],
                            Color::Black => [Square::E8, Square::D8, Square::C8],
                        };

                        if !self.is_square_attcked(squares[0], !color)
                            && !self.is_square_attcked(squares[1], !color)
                            && !self.is_square_attcked(squares[2], !color)
                        {
                            castle_rights |= squares[2];
                        }
                    }
                }

                king_moves.set_bit(castle_rights.0)
            }
        };

        Some(PieceMoves::new(
            Piece::new(piece_type, self.color()),
            square,
            to,
        ))
    }

    /// Generates all possible moves for the current position.
    pub fn generate_moves(&self) -> Vec<Move> {
        let mut pieces = Vec::new();

        for piece_type in PieceType::ALL {
            for square in self.piece_bitboard(Piece::new(piece_type, self.color())) {
                if let Some(piece_moves) = self.generate_piece_moves(piece_type, square) {
                    pieces.extend(piece_moves.moves())
                }
            }
        }

        pieces
    }
}
