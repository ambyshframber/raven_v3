pub fn imm_upper(s1: u32, s2: u32, funct: u32) -> Option<u32> {
    Some(match funct {
        0 => s1 + s2,
        1 => s1 - s2,

        4 => s1 & s2,
        5 => s1 | s2,
        6 => s1 ^ s2,
        7 => (s1 & (2u32.pow(13) - 1)) | s2,

        _ => return None
    })
}
