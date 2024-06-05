use super::{Board, CastleRights};
use crate::{Piece, Player, Square};

#[derive(Clone, Copy, Debug)]
pub struct BoardBuilder {
    pub(super) pieces: [Option<Piece>; 64],
    pub(super) player: Player,
    pub(super) castling_rights: CastleRights,
    pub(super) en_passant_square: Option<Square>,
    pub(super) halfmove_clock: u8,
    pub(super) fullmove_counter: u16,
}

impl BoardBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pieces(&mut self, pieces: [Option<Piece>; 64]) -> &mut BoardBuilder {
        self.pieces = pieces;
        self
    }

    pub fn put_piece(&mut self, piece: Piece, square: Square) -> &mut BoardBuilder {
        self.pieces[square.0 as usize] = Some(piece);
        self
    }

    pub fn player(&mut self, player: Player) -> &mut BoardBuilder {
        self.player = player;
        self
    }

    pub fn castling_rights(&mut self, castling_rights: CastleRights) -> &mut BoardBuilder {
        self.castling_rights = castling_rights;
        self
    }

    pub fn en_passant_square(&mut self, en_passant_square: Option<Square>) -> &mut BoardBuilder {
        self.en_passant_square = en_passant_square;
        self
    }

    pub fn halfmove_clock(&mut self, halfmove_clock: u8) -> &mut BoardBuilder {
        self.halfmove_clock = halfmove_clock;
        self
    }

    pub fn fullmove_counter(&mut self, fullmove_counter: u16) -> &mut BoardBuilder {
        assert!(fullmove_counter > 0);

        self.fullmove_counter = fullmove_counter;
        self
    }

    pub fn build(self) -> Board {
        let mut board = Board {
            player: self.player,
            piece_types_bitboards: Default::default(),
            player_bitboards: Default::default(),
            castling_rights: self.castling_rights,
            en_passant_square: self.en_passant_square,
            halfmove_clock: self.halfmove_clock,
            fullmove_counter: self.fullmove_counter,
        };

        for (square, piece) in self.pieces.iter().enumerate() {
            if let Some(piece) = piece {
                board.put_piece(*piece, Square(square as u8));
            }
        }

        board
    }
}

impl Default for BoardBuilder {
    fn default() -> Self {
        BoardBuilder {
            pieces: [None; 64],
            player: Player::default(),
            castling_rights: CastleRights::default(),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_counter: 1,
        }
    }
}
