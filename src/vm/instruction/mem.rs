use crate::memory::{Memory, MemoryError};

pub fn load<M: Memory>(s1: u32, s2: u32, funct: u32, mem: &M) -> Result<u32, LoadError> {
    let addr = s1.wrapping_add_signed(s2 as i32);
    
    let res = match funct {
        0 => mem.read_u32(addr)?, // lw
        1 => mem.read_u16(addr)? as u32, // lh.u
        2 => mem.read_u16(addr)? as i16 as i32 as u32, // lh.i
        3 => mem.read_u8(addr)? as u32, // lb.u
        4 => mem.read_u8(addr)? as i8 as i32 as u32, // lb.i
        _ => return Err(LoadError::Funct)
    };

    Ok(res)
}

#[derive(Debug, PartialEq)]
pub enum LoadError {
    Mem(MemoryError),
    Funct
}
impl From<MemoryError> for LoadError {
    fn from(value: MemoryError) -> Self {
        Self::Mem(value)
    }
}

pub fn store<M: Memory>(s1: u32, s2: u32, s3: u32, funct: u32, mem: &mut M) -> Result<(), StoreError> {
    let addr = s1.wrapping_add_signed(s2 as i32);
    
    Ok(match funct {
        0 => mem.write_u32(addr, s3)?,
        1 => mem.write_u16(addr, s3 as u16)?,
        2 => mem.write_u8(addr, s3 as u8)?,

        _ => return Err(StoreError::Funct)
    })
}

#[derive(Debug, PartialEq)]
pub enum StoreError {
    Mem(MemoryError),
    Funct
}
impl From<MemoryError> for StoreError {
    fn from(value: MemoryError) -> Self {
        Self::Mem(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::MainMemory;

    #[test]
    fn mem_instructions() {
        let mut mem = MainMemory::new(vec![]).unwrap();

        store(100, 4, 1234, 0, &mut mem).unwrap();
        assert_eq!(load(104, 0, 0, &mut mem), Ok(1234));
        assert_eq!(load(108, -4i32 as u32, 0, &mut mem), Ok(1234));
        assert_eq!(load(104, 0, 3, &mut mem), Ok(1234 & 0xff));
        assert_eq!(load(104, 0, 4, &mut mem), Ok(-46i32 as u32));
    }
}
