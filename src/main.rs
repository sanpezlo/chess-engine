use chess_engine::BitBoard;

fn main() {
    println!("{}", BitBoard::new(1u64));
    println!("{}", BitBoard::new(1u64 << 7));
    println!("{}", BitBoard::new(1u64 << 63));
}
