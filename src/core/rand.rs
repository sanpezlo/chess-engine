/// Generate a random number using a simple xorshift algorithm.
pub const fn random_u32(seed: u32) -> u32 {
    let mut number = seed;

    number ^= number << 13;
    number ^= number >> 17;
    number ^= number << 5;

    number
}

/// Generate a random u64 number.
pub const fn random_u64(seed: u32) -> (u64, u32) {
    let mut seed = seed;

    seed = random_u32(seed);
    let n1 = seed as u64 & 0xFFFF;

    seed = random_u32(seed);
    let n2 = seed as u64 & 0xFFFF;

    seed = random_u32(seed);
    let n3 = seed as u64 & 0xFFFF;

    seed = random_u32(seed);
    let n4 = seed as u64 & 0xFFFF;

    (n1 | (n2 << 16) | (n3 << 32) | (n4 << 48), seed)
}

/// Generate a magic number candidate.
pub const fn random_magic_number(seed: u32) -> (u64, u32) {
    let (n1, seed) = random_u64(seed);
    let (n2, seed) = random_u64(seed);
    let (n3, seed) = random_u64(seed);

    (n1 & n2 & n3, seed)
}
