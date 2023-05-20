use super::registers::RegisterSelector;
use crate::utils::*;

pub mod arithmetic;
pub mod immupper;
pub mod mem;

pub struct Instruction {
    pub opcode: Opcode,
    pub funct: u32,
    is_imm: bool,

    pub rs1: RegisterSelector,
    pub rs2: RegisterSelector,
    pub rs3: RegisterSelector,
    pub rd: RegisterSelector,

    primary_immediate: u32,

    pub p: u32,
}
impl Instruction {
    pub fn select_source_2(&self, regv: u32) -> u32 {
        if self.is_imm {
            self.primary_immediate
        }
        else { regv }
    }

    /// infallible - every bit pattern is a valid instruction
    /// 
    /// it might not be a valid operation, but it's a valid instruction
    pub fn from_iword(i: u32) -> Instruction {
        let (opcode, is_imm) = Opcode::parse(i);
        let primary_immediate = opcode.select_immediate(i);
        let p = extract_p(i);

        type RS = RegisterSelector;
        let rd = if let St = opcode { RS::ZERO } else { RS::rd(i) };
        let rs1 = if let ImmUpper = opcode { rd } else { RS::rs1(i) };
        let rs2 = RS::rs2(i);
        let rs3 = RS::rs3(i);

        let funct = opcode.extract_funct(i, is_imm);

        Instruction {
            opcode, funct, is_imm,
            rs1, rs2, rs3, rd,
            primary_immediate, p
        }
    }
}
#[derive(Copy, Clone)]
pub struct InsData {
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
}
impl InsData {
    pub fn new(s1: u32, s2: u32, s3: u32) -> Self {
        Self { s1, s2, s3 }
    }
}

use Opcode::*;
#[derive(PartialEq, Debug)]
pub enum Opcode {
    Func,
    Arith, ArithSkip,
    Ld, St,
    Io,
    ImmUpper,
    Comp
}
impl Opcode {
    fn parse(i: u32) -> (Opcode, bool) {
        let mut is_imm = i & 1 != 0;
        let opcode = match (i >> 1) & 0b111 {
            0 => Comp,
            1 => Ld,
            2 => Func,
            3 => St,
            4 => Arith,
            5 => Io,
            6 => ArithSkip,
            7 => {
                is_imm = true;
                ImmUpper
            }
            _ => unreachable!()
        };
        (opcode, is_imm)
    }

    fn select_immediate(&self, i: u32) -> u32 {
        match self {
            ImmUpper => extract_u(i),
            St => extract_s(i),
            Func => extract_f(i),
            Io => extract_i(i),
            _ => extract_a(i),
        }
    }
    fn extract_funct(&self, i: u32, is_imm: bool) -> u32 {
        let funct5a = extract_5_bits(i, 9);
        let funct3 = i & !(2u32.pow(29) - 1) << 5;
        match self {
            Arith => {
                if !is_imm {
                    let f5b = extract_5_bits(i, 24) << 8;
                    funct5a | funct3 | f5b
                }
                else {
                    funct5a
                }
            }
            Io => funct5a | funct3,
            ImmUpper => funct5a & 0b1111,
            Func => funct5a & 1,
            _ => funct5a
        }
    }
}

fn extract_u(i: u32) -> u32 {
    i & !(2u32.pow(13) - 1)
}
fn extract_a(i: u32) -> u32 {
    ((i as i32) >> 19) as u32
}
fn extract_s(i: u32) -> u32 {
    let high_3 = i & !(2u32.pow(29) - 1);
    let high_3_sign_extended = (high_3 as i32) >> 19;

    let mid_5 = extract_5_bits(i, 19) << 5;
    let low_5 = extract_5_bits(i, 4);

    (high_3_sign_extended as u32) | mid_5 | low_5
}
fn extract_f(i: u32) -> u32 {
    (((i as i32) >> 13) as u32) & !1
}
fn extract_i(i: u32) -> u32 {
    extract_5_bits(i, 19)
}
fn extract_p(i: u32) -> u32 {
    (i & (0b1111 << 10)) >> 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn immediates() {
        let i: u32 =                0b1010_0101__1010_0101__1010_0101__1010_0101;

        assert_eq!(extract_u(i),    0b1010_0101__1010_0101__1010_0000__0000_0000);
        assert_eq!(extract_a(i),    0b1111_1111__1111_1111__1111_0100__1011_0100);
        assert_eq!(extract_s(i),    0b1111_1111__1111_1111__1111_0110__1001_1010);
        assert_eq!(extract_f(i),    0b1111_1111__1111_1101__0010_1101__0010_1100);
        assert_eq!(extract_i(i),    0b1_0100);
        assert_eq!(extract_p(i),    0b10_01);
    }

    #[test]
    fn parse_1() {
        let iw: u32 = 0b000_00001_00010_00011_00100_00101_1000;
        let i = Instruction::from_iword(iw);

        assert_eq!(i.funct, 0b00001_000_00100);
        assert_eq!(i.rd.inner(), 0b00101);
        assert_eq!(i.rs1.inner(), 0b00011);
        assert_eq!(i.rs2.inner(), 0b00010);
        assert_eq!(i.rs3.inner(), 0b00001);
        assert_eq!(i.primary_immediate, extract_a(iw));
        assert_eq!(i.opcode, Arith);
        assert!(!i.is_imm)
    }
}
