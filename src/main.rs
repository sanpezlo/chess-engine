use chess_engine::bitboard;

fn main() {
    let bitboard = bitboard! {
        X . . . . . . .
        X . . . . . . .
        X . . . . . . .
        X . . . . . . .
        X . . . . . . .
        X . . . . . . .
        X . . . . . . .
        X . . . . . . .
    };

    println!("{:?}", bitboard | 2);
}
