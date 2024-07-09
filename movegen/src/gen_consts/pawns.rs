use chess_engine_core::{BitBoard, Color, Rank, Square};

/// Returns the mask for the pawn attacks for a square.
///
/// # Example
///
/// ```
/// # use chess_engine_core::*;
/// # use chess_engine_movegen::*;
/// let white_pawn_attacks = mask_pawn_attacks(Color::White, Square::E4);
/// let black_pawn_attacks = mask_pawn_attacks(Color::Black, Square::E4);
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
pub const fn mask_pawn_attacks(color: Color, square: Square) -> BitBoard {
    let square = BitBoard(1u64 << square as usize);

    let mut attacks = BitBoard::EMPTY;

    match color {
        Color::White => {
            attacks = attacks.set_bit(square.up().right().0);
            attacks = attacks.set_bit(square.up().left().0);
        }
        Color::Black => {
            attacks = attacks.set_bit(square.down().right().0);
            attacks = attacks.set_bit(square.down().left().0);
        }
    }

    attacks
}

/// Returns the mask for the pawn moves for a square.
///
/// # Example
///
/// ```
/// # use chess_engine_core::*;
/// # use chess_engine_movegen::*;
/// let white_pawn_attacks = mask_pawn_attacks(Color::White, Square::E4);
/// let black_pawn_attacks = mask_pawn_attacks(Color::Black, Square::E4);
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
pub const fn mask_pawn_moves(color: Color, square: Square) -> BitBoard {
    let rank = square.rank();
    let square = BitBoard(1u64 << square as usize);

    let mut attacks = BitBoard::EMPTY;

    match color {
        Color::White => {
            attacks = attacks.set_bit(square.up().0);

            if rank as usize == Rank::Two as usize {
                attacks = attacks.set_bit(square.up().up().0);
            }
        }
        Color::Black => {
            attacks = attacks.set_bit(square.down().0);

            if rank as usize == Rank::Seven as usize {
                attacks = attacks.set_bit(square.down().down().0);
            }
        }
    }

    attacks
}

/// Writes to a file
///
/// - the precomputed pawn attacks for all squares and colors.
/// - the precomputed pawn moves for all squares and colors.
pub fn write(f: &mut std::fs::File) {
    use std::io::Write;

    // pawn attacks

    write!(f, "/// Precomputed pawn attacks for all squares\n").unwrap();

    write!(
        f,
        "pub const PAWN_ATTACKS: [[BitBoard; {}]; {}] = [\n",
        Square::LEN,
        Color::LEN
    )
    .unwrap();

    for color in Color::ALL {
        write!(f, "\t[\n").unwrap();

        for square in Square::ALL {
            write!(f, "\t\tBitBoard({}),\n", mask_pawn_attacks(color, square).0).unwrap();
        }

        write!(f, "\t],\n").unwrap();
    }
    write!(f, "];\n").unwrap();

    // pawn moves

    write!(f, "/// Precomputed pawn moves for all squares\n").unwrap();

    write!(
        f,
        "pub const PAWN_MOVES: [[BitBoard; {}]; {}] = [\n",
        Square::LEN,
        Color::LEN
    )
    .unwrap();

    for color in Color::ALL {
        write!(f, "\t[\n").unwrap();

        for square in Square::ALL {
            write!(f, "\t\tBitBoard({}),\n", mask_pawn_moves(color, square).0).unwrap();
        }

        write!(f, "\t],\n").unwrap();
    }
    write!(f, "];\n").unwrap();
}
