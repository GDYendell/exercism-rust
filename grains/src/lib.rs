pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 1_u64 << (s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    // All bits of a u64 set
    u64::MAX
}
