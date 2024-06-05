use chess_engine::{Board, Color};

fn main() {
    let mut builder = Board::builder();

    builder.put_piece("B".parse().unwrap(), "a1".parse().unwrap());

    builder.put_piece("B".parse().unwrap(), "c2".parse().unwrap());

    let board = builder.build();

    println!("{}", board.player_bitboard(Color::White));

    println!("{}", board.has_bishop_pair(Color::White));
}
