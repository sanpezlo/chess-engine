use chess_engine::{Board, Color, ZOBRIST};

fn main() {
    let mut builder = Board::builder();

    builder.put_piece("N".parse().unwrap(), "a1".parse().unwrap());

    builder.put_piece("N".parse().unwrap(), "c2".parse().unwrap());

    builder.put_piece("N".parse().unwrap(), "a2".parse().unwrap());

    let board = builder.build();

    println!("{}", board.player_bitboard(Color::White));

    println!("{}", board.draw_by_insufficient_material());

    println!("{:?}", *ZOBRIST);
}
