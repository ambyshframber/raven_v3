#![feature(bigint_helper_methods)]

use thiserror::Error;
use registers::RegisterSelector as RS;
use instruction::{Instruction, Opcode};

mod instruction;
mod utils;
mod file;
mod registers;
mod memory;

fn main() {
    println!("Hello, world!");
}

struct VM<M> {
    memory: M,
    registers: registers::Registers,
    files: file::FileTable,
}
impl<M: memory::Memory> VM<M> {
    /// returns true on exit command
    fn cycle(&mut self) -> Result<bool, VMError> {
        let mut pc = self.registers.read(RS::PC);
        let iw = self.memory.read_u32(pc)?;
        let i = Instruction::from_iword(iw);

        let s1 = self.registers.read(i.rs1);
        let s2 = i.select_source_2(self.registers.read(i.rs2));
        let s3 = self.registers.read(i.rs3);

        use Opcode::*;
        let res = match i.opcode {
            Arith => instruction::arithmetic::arithmetic(s1, s2, i.funct).ok_or(VMError::Arith)?,
            ArithSkip => {
                let res = instruction::arithmetic::arithmetic(s1, s2, i.funct).ok_or(VMError::Arith)?;
                if res != 0 {
                    pc += 1 // skip an instruction
                }
                res
            }
            ImmUpper => instruction::immupper::imm_upper(s1, s2, i.funct).ok_or(VMError::ImmUpper)?,
            Ld => instruction::load::load(s1, s2, i.funct, &self.memory)?,
            St => {
                instruction::store::store(s1, s2, s3, i.funct, &mut self.memory)?;
                0
            }
            _ => todo!()
        };
        self.registers.write(i.rd, res);

        self.registers.write(RS::PC, pc + 1);

        Ok(false)
    }
}

#[derive(Debug, Error)]
enum VMError {
    #[error("memory error: {0:?}")]
    Mem(memory::MemoryError),
    #[error("invalid arithmetic funct")]
    Arith,
    #[error("invalid imm_upper funct")]
    ImmUpper,
    #[error("invalid load funct")]
    Ld,
}
impl From<memory::MemoryError> for VMError {
    fn from(value: memory::MemoryError) -> Self {
        Self::Mem(value)
    }
}
impl From<instruction::load::LoadError> for VMError {
    fn from(value: instruction::load::LoadError) -> Self {
        use instruction::load::LoadError;
        match value {
            LoadError::Funct => Self::Ld,
            LoadError::Mem(m) => Self::from(m)
        }
    }
}
impl From<instruction::store::StoreError> for VMError {
    fn from(value: instruction::store::StoreError) -> Self {
        use instruction::store::StoreError;
        match value {
            StoreError::Funct => Self::Ld,
            StoreError::Mem(m) => Self::from(m)
        }
    }
}
