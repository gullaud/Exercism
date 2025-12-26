pub fn square(s: u32) -> u64 {
    if s == 0 || s > 64 {panic!("Square must be in between 1..=64")}
    (2_u64).pow(s-1)
}

pub fn total() -> u64 {
    (1..=64).fold(0, |sum, i| sum + square(i))
}
