use thiserror::Error;
use registers::RegisterSelector as RS;
use instruction::{Instruction, Opcode, InsData};
use crate::utils::*;
use crate::io;
use crate::memory;


pub mod instruction;
mod registers;

struct VM {
    registers: registers::Registers,
}
impl VM {
    fn new() -> Self {
        todo!()
    }

    /// returns true on exit command
    fn cycle<M: memory::Memory>(&mut self, io: &mut io::IoHandler, memory: &mut M) -> Result<bool, VMError> {
        let pc = self.registers.read(RS::PC);
        let iw = memory.read_u32(pc)?;
        let i = Instruction::from_iword(iw);

        let s1 = self.registers.read(i.rs1);
        let s2 = i.select_source_2(self.registers.read(i.rs2));
        let s3 = self.registers.read(i.rs3);

        let idata = InsData::new(s1, s2, s3);

        let mut next_pc = pc; // actually set to one less than the next address to execute because it gets incremented at the end of the cycle
        let mut exec_result = 0; // all instructions return a value

        use Opcode::*;
        if let Io = i.opcode {
            todo!()
        }
        else {
            let res = Self::exec_instruction(i.opcode, idata, i.funct, pc, memory)?;
            match res {
                Exec::Normal(v) => {
                    exec_result = v;
                }
                Exec::Skip(v) => {
                    exec_result = v;
                    next_pc += 1;
                }
                Exec::Call(ret, pc) => { // jump destinations ARE incremented
                    // hence call 0 is save and return pc is restore
                    self.registers.call();
                    exec_result = ret; // return value is written AFTER window shift
                    next_pc = pc;
                }
                Exec::Return(pc) => {
                    self.registers.ret(); // return value is read BEFORE register shift
                    next_pc = pc;
                }
            }
        }
        self.registers.write(RS::PC, next_pc + 1);
        if i.rd == RS::PC {
            exec_result += 1 // increment!
        }
        self.registers.write(i.rd, exec_result); // write result after incrementing pc, to allow jumping with arithmetic instructions
        
        Ok(false)
    }

    fn exec_instruction<M: memory::Memory>(opcode: Opcode, d: InsData, funct: u32, pc: u32, memory: &mut M) -> Result<Exec, VMError> {
        use Opcode::*;
        Ok(match opcode {
            Arith => instruction::arithmetic::arithmetic(d.s1, d.s2, funct).map(Exec::Normal).ok_or(VMError::Arith)?,
            ArithSkip => instruction::arithmetic::arithmetic(d.s1, d.s2, funct)
                .map(|v| if v != 0 { Exec::Skip(v) } else { Exec::Normal(v) })
                .ok_or(VMError::Arith)?,
            ImmUpper => instruction::immupper::imm_upper(d.s1, d.s2, funct).map(Exec::Normal).ok_or(VMError::ImmUpper)?,
            Ld => instruction::mem::load(d.s1, d.s2, funct, memory).map(Exec::Normal)?,
            St => {
                instruction::mem::store(d.s1, d.s2, d.s3, funct, memory)?;
                Exec::Normal(0)
            },
            Func => { // increments happen inside the main exec loop
                if funct != 0 { // return
                    Exec::Return(d.s2)
                }
                else { // call
                    let old_pc = pc; // gets incremented on return
                    let pc = pc.wrapping_add_signed(d.s2 as i32);

                    Exec::Call(old_pc, pc)
                }
            }
            Comp => return Err(VMError::Compressed),
            _ => unreachable!("io operations are handled outside this function")
        })
    }

    fn io(&mut self, s1: u32, s2: u32, s3: u32, funct: u32) -> Result<u32, VMError> {
        Ok(0)
    }
}

#[derive(Debug, PartialEq)]
pub enum Exec {
    Normal(u32),
    Skip(u32),
    /// (return, new_pc)
    Call(u32, u32),
    Return(u32),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sanity() {
        
    }

    #[test]
    fn test_exec_isolated() {
        let mut mem = vec![0, 1, 2, 3, 4, 5, 6, 7];

        let idata = InsData::new(0, 4, 0x80);
        assert_eq!(VM::exec_instruction(Opcode::Arith, idata, 0, 0, &mut mem), Ok(Exec::Normal(4)));
        assert_eq!(VM::exec_instruction(Opcode::ArithSkip, idata, 0, 0, &mut mem), Ok(Exec::Skip(4)));

        assert_eq!(VM::exec_instruction(Opcode::Ld, idata, 0, 0, &mut mem), Ok(Exec::Normal(0x0706_0504)));
        assert_eq!(VM::exec_instruction(Opcode::St, idata, 2, 0, &mut mem), Ok(Exec::Normal(0)));
        assert_eq!(mem[4], 0x80);
        assert_eq!(VM::exec_instruction(Opcode::Ld, idata, 0, 0, &mut mem), Ok(Exec::Normal(0x0706_0580)));

        assert_eq!(VM::exec_instruction(Opcode::Func, idata, 0, 0, &mut mem), Ok(Exec::Call(0, 4)));
    }
}

#[derive(Debug, Error, PartialEq)]
enum VMError {
    #[error("memory error: {0:?}")]
    Mem(memory::MemoryError),
    #[error("invalid arithmetic funct")]
    Arith,
    #[error("invalid imm_upper funct")]
    ImmUpper,
    #[error("invalid load funct")]
    Ld,
    #[error("invalid store funct")]
    St,
    #[error("invalid io funct")]
    IoFunct,
    #[error("failed io operation")]
    Io,
    #[error("compressed instructions dont exist yet")]
    Compressed,
}
impl From<memory::MemoryError> for VMError {
    fn from(value: memory::MemoryError) -> Self {
        Self::Mem(value)
    }
}
impl From<instruction::mem::LoadError> for VMError {
    fn from(value: instruction::mem::LoadError) -> Self {
        use instruction::mem::LoadError;
        match value {
            LoadError::Funct => Self::Ld,
            LoadError::Mem(m) => Self::from(m)
        }
    }
}
impl From<instruction::mem::StoreError> for VMError {
    fn from(value: instruction::mem::StoreError) -> Self {
        use instruction::mem::StoreError;
        match value {
            StoreError::Funct => Self::St,
            StoreError::Mem(m) => Self::from(m)
        }
    }
}
impl From<io::IoError> for VMError {
    fn from(value: io::IoError) -> Self {
        todo!()
    }
}
