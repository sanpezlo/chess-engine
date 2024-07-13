use chess_engine_movegen::*;

fn main() {
    let board = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        .parse::<BoardBuilder>()
        .unwrap()
        .build();

    println!("{:?}", board);

    for moves in board.generate_moves() {
        println!("{:?}", moves);
    }
}
