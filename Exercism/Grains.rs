pub fn square(s: u32) -> u64 {
    if s >= 1 && s <= 64{
        2u64.pow(s - 1)
    } else {
        panic!("Square must be between 1 and 64");
    }
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}