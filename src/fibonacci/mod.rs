
pub fn generate_at_position(num: u64) -> u64 {
    if num == 1 { return 0 }
    if num == 2 { return 1 } else { return generate_at_position(num - 1) + generate_at_position(num - 2)}
}
