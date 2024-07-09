use chess_engine_core::{BitBoard, Color, SlidingPiece, Square};

include!(concat!(env!("OUT_DIR"), "/magic_gen.rs"));

/// Returns the pawn attacks for a square and color.
///
/// # Example
///
/// ```
/// # use chess_engine_core::*;
/// # use chess_engine_movegen::*;
/// let white_pawn_attacks = get_pawn_attacks(Color::White, Square::E4);
/// let black_pawn_attacks = get_pawn_attacks(Color::Black, Square::E4);
///
/// assert_eq!(white_pawn_attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . X . X . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
/// });
/// assert_eq!(black_pawn_attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . X . X . .
///     . . . . . . . .
///     . . . . . . . .
/// });
/// ```
pub fn get_pawn_attacks(color: Color, square: Square) -> BitBoard {
    PAWN_ATTACKS[color as usize][square as usize]
}

/// Returns the pawn moves for a square and color.
///
/// # Example
///
/// ```
/// # use chess_engine_core::*;
/// # use chess_engine_movegen::*;
/// let white_pawn_attacks = get_pawn_moves(Color::White, Square::E2);
/// let black_pawn_attacks = get_pawn_moves(Color::Black, Square::E7);
///
/// assert_eq!(white_pawn_attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . X . . .
///     . . . . X . . .
///     . . . . . . . .
///     . . . . . . . .
/// });
/// assert_eq!(black_pawn_attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . . X . . .
///     . . . . X . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
/// });
/// ```
pub fn get_pawn_moves(color: Color, square: Square) -> BitBoard {
    PAWN_MOVES[color as usize][square as usize]
}

/// Returns the king attacks for a square.
///
/// /// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let king_attacks = get_king_attacks(Square::E4);
///
/// assert_eq!(king_attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . X X X . .
///     . . . X . X . .
///     . . . X X X . .
///     . . . . . . . .
///     . . . . . . . .
/// });
/// ```
pub fn get_king_attacks(square: Square) -> BitBoard {
    KING_ATTACKS[square as usize]
}

/// Returns the knight attacks for a square.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let knight_attacks = get_knight_attacks(Square::E4);
///
/// assert_eq!(knight_attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . X . X . .
///     . . X . . . X .
///     . . . . . . . .
///     . . X . . . X .
///     . . . X . X . .
///     . . . . . . . .
/// });
/// ```
pub fn get_knight_attacks(square: Square) -> BitBoard {
    KNIGHT_ATTACKS[square as usize]
}

/// Returns the bishop attacks for a square with blockers.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let attacks = get_bishop_attacks(Square::E4, bitboard!{
///     X . . . . . . .
///     . . . . . . . X
///     . . X . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . X . .
///     . . X . . . X .
///     . . . . . . . .
/// });
///
/// assert_eq!(attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . X
///     . . X . . . X .
///     . . . X . X . .
///     . . . . . . . .
///     . . . X . X . .
///     . . X . . . . .
///     . . . . . . . .
/// });
/// ```
pub fn get_bishop_attacks(square: Square, blockers: BitBoard) -> BitBoard {
    let magic_index = blockers.0 & RELEVANT_BISHOP_BLOCKERS[square as usize].0;
    let magic_index = magic_index
        .wrapping_mul(MAGIC_NUMBERS[SlidingPiece::Bishop as usize][square as usize])
        >> (64 - RELEVANT_BISHOP_BLOCKERS_COUNT[square as usize]);

    BISHOP_ATTACKS[square as usize][magic_index as usize]
}

/// Returns the rook attacks for a square with blockers.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let attacks = get_rook_attacks(Square::E4, bitboard!{
///     . . . . . . . .
///     . . . . X . . .
///     . . . . . . . .
///     . . . . X . . .
///     . X . . . . . X
///     . . . . . . . .
///     . . . . X . . .
///     . . . . X . . .
/// });
///
/// assert_eq!(attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . . .
///     . . . . X . . .
///     . X X X . X X X
///     . . . . X . . .
///     . . . . X . . .
///     . . . . . . . .
/// });
/// ```
pub fn get_rook_attacks(square: Square, blockers: BitBoard) -> BitBoard {
    let magic_index = blockers.0 & RELEVANT_ROOK_BLOCKERS[square as usize].0;
    let magic_index = magic_index
        .wrapping_mul(MAGIC_NUMBERS[SlidingPiece::Rook as usize][square as usize])
        >> (64 - RELEVANT_ROOK_BLOCKERS_COUNT[square as usize]);

    ROOK_ATTACKS[square as usize][magic_index as usize]
}

/// Returns the queen attacks for a square with blockers.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let attacks = get_queen_attacks(Square::E4, bitboard!{
///     . . . . . . . .
///     . X . . X . . .
///     . . . . . . X .
///     . . . X X . . .
///     . X . . . . . X
///     . . . . . . . .
///     . . X . X . . .
///     . . . . X . . X
/// });
///
/// assert_eq!(attacks, bitboard!{
///     . . . . . . . .
///     . . . . . . . .
///     . . . . . . X .
///     . . . X X X . .
///     . X X X . X X X
///     . . . X X X . .
///     . . X . X . X .
///     . . . . . . . X
/// });
/// ```
pub fn get_queen_attacks(square: Square, blockers: BitBoard) -> BitBoard {
    get_bishop_attacks(square, blockers) | get_rook_attacks(square, blockers)
}
