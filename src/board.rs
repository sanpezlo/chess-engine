use crate::{BitBoard, CastleRights, Piece, Player, Square, PIECE_TYPES, PLAYERS};

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

    pub fn piece_types_bitboards(&self) -> &[BitBoard; PIECE_TYPES] {
        &self.piece_types_bitboards
    }

    pub fn player_bitboards(&self) -> &[BitBoard; PLAYERS] {
        &self.player_bitboards
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

    pub fn set_player(&mut self, player: Player) {
        self.player = player;
    }

    pub fn put_piece(&mut self, square: Square, piece: Piece) {
        let piece_type = piece.piece_type() as usize;
        let player = piece.player() as usize;

        self.piece_types_bitboards[piece_type] |= square.into();
        self.player_bitboards[player] |= square.into();
    }

    pub fn set_castling_rights(&mut self, castling_rights: CastleRights) {
        self.castling_rights = castling_rights;
    }

    pub fn set_en_passant_square(&mut self, en_passant_square: Option<Square>) {
        self.en_passant_square = en_passant_square;
    }

    pub fn set_halfmove_clock(&mut self, halfmove_clock: u8) {
        self.halfmove_clock = halfmove_clock;
    }

    pub fn set_fullmove_counter(&mut self, fullmove_counter: u16) {
        self.fullmove_counter = fullmove_counter;
    }
}

impl Default for Board {
    fn default() -> Self {
        Self {
            player: Player::White,
            piece_types_bitboards: [BitBoard::default(); PIECE_TYPES],
            player_bitboards: [BitBoard::default(); PLAYERS],
            castling_rights: CastleRights::default(),
            en_passant_square: None,
            halfmove_clock: 0,
            fullmove_counter: 1,
        }
    }
}
