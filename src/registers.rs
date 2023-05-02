use crate::utils::*;

pub struct Registers {
    globals: [u32; 8],
    // top local is shared with callee (r8..=15)
    // second from top local is true locals and shared with caller (r16..=23, r24..=31)
    locals: Vec<LocalSet>,
}
impl Registers {
    pub fn new() -> Self {
        let mut ret = Self {
            globals: [0; 8],
            locals: Vec::new()
        };
        ret.call(); ret.call();
        ret
    }

    pub fn call(&mut self) {
        self.locals.push(LocalSet::new())
    }
    pub fn ret(&mut self) {
        self.locals.pop();
    }

    pub fn read(&self, rs: RegisterSelector) -> u32 {
        match rs.0 {
            0 => 0,
            1..=7 => {
                self.globals[rs.0 as usize]
            }
            8..=15 => {
                let set = &self.locals[self.locals.len() - 1];
                set.shared[rs.0 as usize - 8]
            }
            16..=23 => {
                let set = &self.locals[self.locals.len() - 2];
                set.local[rs.0 as usize - 16]
            }
            24..=31 => {
                let set = &self.locals[self.locals.len() - 2];
                set.shared[rs.0 as usize - 16]
            }
            _ => unreachable!()
        }
    }
    pub fn write(&mut self, rs: RegisterSelector, v: u32) {
        let llen = self.locals.len();
        match rs.0 {
            0..=7 => {
                self.globals[rs.0 as usize] = v
            }
            8..=15 => {
                let set = &mut self.locals[llen - 1];
                set.shared[rs.0 as usize - 8] = v
            }
            16..=23 => {
                let set = &mut self.locals[llen - 2];
                set.local[rs.0 as usize - 16] = v
            }
            24..=31 => {
                let set = &mut self.locals[llen - 2];
                set.shared[rs.0 as usize - 16] = v
            }
            _ => unreachable!()
        }
    }
}

struct LocalSet {
    shared: [u32; 8],
    local: [u32; 8]
}
impl LocalSet {
    fn new() -> Self {
        Self { shared: [0; 8], local: [0; 8] }
    }
}

/// enforces an invariant
#[derive(Clone, Copy)]
pub struct RegisterSelector(u8);
impl RegisterSelector {
    pub fn new(r: u8) -> Option<Self> {
        if r < 32 {
            Some(Self(r))
        }
        else { None }
    }
    pub fn inner(&self) -> u8 {
        self.0
    }

    pub const ZERO: Self = Self(0);
    pub const PC: Self = Self(2);

    pub fn rd(i: u32) -> Self {
        Self(extract_5_bits(i, 4) as u8)
    }
    pub fn rs1(i: u32) -> Self {
        Self(extract_5_bits(i, 14) as u8)
    }
    pub fn rs2(i: u32) -> Self {
        Self(extract_5_bits(i, 19) as u8)
    }
    pub fn rs3(i: u32) -> Self {
        Self(extract_5_bits(i, 24) as u8)
    }
}
