pub fn imm_upper(s1: u32, s2: u32, funct: u32) -> Option<u32> {
    Some(match funct {
        0 => s1 + s2,
        1 => s1 - s2,

        8 => s1 & s2,
        9 => s1 | s2,
        10 => s1 ^ s2,
        11 => (s1 & (2u32.pow(13) - 1)) | s2,

        _ => return None
    })
}
