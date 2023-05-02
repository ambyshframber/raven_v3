use crate::memory::{Memory, MemoryError};

pub fn store<M: Memory>(s1: u32, s2: u32, s3: u32, funct: u32, mem: &mut M) -> Result<(), StoreError> {
    let addr = s1 + s2;
    Ok(match funct {
        0 => mem.write_u32(addr, s3)?,
        1 => mem.write_u16(addr, s3 as u16)?,
        2 => mem.write_u8(addr, s3 as u8)?,

        _ => return Err(StoreError::Funct)
    })
}

pub enum StoreError {
    Mem(MemoryError),
    Funct
}
impl From<MemoryError> for StoreError {
    fn from(value: MemoryError) -> Self {
        Self::Mem(value)
    }
}
