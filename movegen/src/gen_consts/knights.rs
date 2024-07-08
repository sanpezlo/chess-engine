use chess_engine_core::{BitBoard, Square};

/// Returns the mask for the knight attacks for a square.
///
/// # Example
///
/// ```
/// # use chess_engine_movegen::*;
/// # use chess_engine_core::*;
/// let knight_attacks = mask_knight_attacks(Square::E4);
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
pub const fn mask_knight_attacks(square: Square) -> BitBoard {
    let square = BitBoard(1u64 << square as usize);

    let mut attacks = BitBoard::EMPTY;

    attacks = attacks.set_bit(square.up().up().right().0);
    attacks = attacks.set_bit(square.up().up().left().0);
    attacks = attacks.set_bit(square.right().right().up().0);
    attacks = attacks.set_bit(square.right().right().down().0);
    attacks = attacks.set_bit(square.down().down().right().0);
    attacks = attacks.set_bit(square.down().down().left().0);
    attacks = attacks.set_bit(square.left().left().up().0);
    attacks = attacks.set_bit(square.left().left().down().0);

    attacks
}

/// Writes to a file the precomputed knight attacks for all squares.
pub fn write(f: &mut std::fs::File) {
    use std::io::Write;

    write!(
        f,
        "/// precomputed knight attacks for all squares\n"
    )
    .unwrap();

    write!(
        f,
        "pub const KNIGHT_ATTACKS: [BitBoard; {}] = [\n",
        Square::LEN
    )
    .unwrap();

    for square in Square::ALL {
        write!(f, "\tBitBoard({}),\n", mask_knight_attacks(square).0).unwrap();
    }

    write!(f, "];\n").unwrap();
}
