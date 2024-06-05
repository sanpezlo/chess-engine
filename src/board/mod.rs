mod board_builder;
mod castle_rights;
mod fen;

use crate::{BitBoard, Color, Piece, PieceType, Player, Square, PIECE_TYPES, PLAYERS};
pub use board_builder::*;
pub use castle_rights::*;

#[derive(Clone, Copy, Debug)]
pub struct Board {
    player: Player,
    piece_types_bitboards: [BitBoard; PIECE_TYPES],
    player_bitboards: [BitBoard; PLAYERS],
    castling_rights: CastleRights,
    en_passant_square: Option<Square>,
    halfmove_clock: u8,
    fullmove_counter: u16,
}

impl Board {
    pub fn player(&self) -> Player {
        self.player
    }

    pub fn piece_types_bitboard(&self, piece_type: PieceType) -> BitBoard {
        self.piece_types_bitboards[piece_type as usize]
    }

    pub fn player_bitboard(&self, color: Color) -> BitBoard {
        self.player_bitboards[color as usize]
    }

    pub fn castling_rights(&self) -> CastleRights {
        self.castling_rights
    }

    pub fn en_passant_square(&self) -> Option<Square> {
        self.en_passant_square
    }

    pub fn halfmove_clock(&self) -> u8 {
        self.halfmove_clock
    }

    pub fn fullmove_counter(&self) -> u16 {
        self.fullmove_counter
    }
}

impl Board {
    pub fn builder() -> BoardBuilder {
        BoardBuilder::new()
    }

    pub fn put_piece(&mut self, piece: Piece, square: Square) {
        let piece_type = piece.piece_type() as usize;
        let player = piece.player().0 as usize;

        self.piece_types_bitboards[piece_type] |= square.into();
        self.player_bitboards[player] |= square.into();
    }

    pub fn has_bishop_pair(&self, color: Color) -> bool {
        let color = color as usize;

        let mut bitboard =
            self.piece_types_bitboards[PieceType::Bishop as usize] & self.player_bitboards[color];

        let mut white_square = 0;
        let mut black_square = 0;

        if bitboard.0.count_ones() >= 2 {
            while bitboard.0 != 0 {
                let square = Square(bitboard.0.trailing_zeros() as u8);
                if square.color() == Color::White {
                    white_square += 1;
                } else {
                    black_square += 1;
                }

                bitboard = BitBoard(bitboard.0 ^ 1 << bitboard.0.trailing_zeros() as u64);
            }
        }

        white_square >= 1 && black_square >= 1
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            player: Player(Color::White),
            piece_types_bitboards: [BitBoard::default(); PIECE_TYPES],
            player_bitboards: [BitBoard::default(); PLAYERS],
            castling_rights: CastleRights::default(),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_counter: 1,
        }
    }
}
