pub fn extract_5_bits(i: u32, idx: u32) -> u32 {
    (i & (0b1_1111 << idx)) >> idx
}
