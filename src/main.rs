use chess_engine::Rank;

fn main() {
    let rank: Rank = "a".parse().unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    });

    println!("{:?}", rank);
}
