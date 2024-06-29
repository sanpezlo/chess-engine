use chess_engine::bitboard;

fn main() {
    let bb = bitboard! {
        X X X X X X X X
        X . . . . . . X
        X . . . . . . X
        X . . . . . . X
        X . . . . . . X
        X . . . . . . X
        X . . . . . . X
        X X X X X X X X
    };

    println!("{}", bb);
}
