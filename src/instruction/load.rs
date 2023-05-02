use crate::memory::{Memory, MemoryError};

pub fn load<M: Memory>(s1: u32, s2: u32, funct: u32, mem: &M) -> Result<u32, LoadError> {
    let addr = s1 + s2;
    
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

pub enum LoadError {
    Mem(MemoryError),
    Funct
}
impl From<MemoryError> for LoadError {
    fn from(value: MemoryError) -> Self {
        Self::Mem(value)
    }
}
