use std::sync::Mutex;

/// The seed used to generate pseudo-random numbers.
///
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
/// assert_eq!(random_u32(), 1741896308);
/// assert_eq!(random_u32(), 321584506);
/// ```
pub static RAND_SEED: Mutex<u32> = Mutex::new(1804289383);

/// Generate a pseudo-random number using a simple xorshift algorithm.
pub fn random_u32() -> u32 {
    let mut seed = RAND_SEED.lock().unwrap();

    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;

    *seed
}

/// Generate a pseudo-random u64 number.
///
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
/// assert_eq!(random_u64(), 4747071328949516916);
/// assert_eq!(random_u64(), 17614855446833723417);
/// ```
pub fn random_u64() -> u64 {
    let n1 = random_u32() as u64 & 0xFFFF;

    let n2 = random_u32() as u64 & 0xFFFF;

    let n3 = random_u32() as u64 & 0xFFFF;

    let n4 = random_u32() as u64 & 0xFFFF;

    n1 | (n2 << 16) | (n3 << 32) | (n4 << 48)
}

/// Generate a magic number candidate.
///
/// # Examples
///
/// ```
/// # use chess_engine_core::*;
/// assert_eq!(random_magic_number(), 27162335321796624);
/// assert_eq!(random_magic_number(), 2305904590471561224);
/// ```
pub fn random_magic_number() -> u64 {
    random_u64() & random_u64() & random_u64()
}
