pub fn arithmetic(s1: u32, s2: u32, funct: u32) -> Option<u32> {
    Some(match funct {
        0 => s1.wrapping_add(s2), // add
        2 => s2.wrapping_sub(s1), // sub: s2 - s1. for reg - imm, add a negative immediate
        4 => s1 & s2, // and
        5 => s1 | s2, // or
        6 => s1 ^ s2, // xor
        7 => !s1, // not

        8 => s1.widening_mul(s2).0, // mull.u
        9 => s1.widening_mul(s2).1, // mulh.u
        10 => (s1 as i32).wrapping_mul(s2 as i32) as u32, // mul.i
        11 => todo!("mulh.i is harder than it sounds ok"), // mulh.i

        12 => s1.checked_div(s2).unwrap_or(-1i32 as u32),
        13 => s1.checked_rem(s2).unwrap_or(s1),
        14 => (s1 as i32).checked_div(s2 as i32).unwrap_or(-1i32) as u32,
        15 => (s1 as i32).checked_rem(s2 as i32).unwrap_or(s1 as i32) as u32,

        16 => s1 << s2,
        17 => s1 >> s2,
        18 => ((s1 as i32) << s2) as u32,
        19 => ((s1 as i32) >> s2) as u32,
        20 => s1.rotate_left(s2),
        21 => s1.rotate_right(s2),

        22..=31 => {
            (match funct {
                22 => s1 == s2,
                23 => s1 != s2,
        
                24 => s1 > s2,
                25 => s1 >= s2,
                26 => (s1 as i32) > (s2 as i32),
                27 => (s1 as i32) >= (s2 as i32),
        
                28 => s1 < s2,
                29 => s1 <= s2,
                30 => (s1 as i32) < (s2 as i32),
                31 => (s1 as i32) <= (s2 as i32),

                _ => unreachable!()
            }) as u32
        }

        _ => return None
    })
}
