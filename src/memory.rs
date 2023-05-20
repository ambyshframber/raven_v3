use crate::utils::*;

mod btreemem;
mod splitmem;

pub trait Memory {
    fn read_u32(&self, addr: u32) -> MemoryResult<u32>;
    fn read_u16(&self, addr: u32) -> MemoryResult<u16>;
    fn read_u8(&self, addr: u32) -> MemoryResult<u8>;

    /// does not have to return the entire length requested, as memory implementations may store data non-contiguously
    fn read_slice(&self, addr: u32, len: u32) -> MemoryResult<&[u8]>;

    fn write_u32(&mut self, addr: u32, v: u32) -> MemoryResult<()>;
    fn write_u16(&mut self, addr: u32, v: u16) -> MemoryResult<()>;
    fn write_u8(&mut self, addr: u32, v: u8) -> MemoryResult<()>;
}

pub type MemoryResult<T> = Result<T, MemoryError>;
use MemoryError::*;
#[derive(Debug, PartialEq)]
pub enum MemoryError {
    Uninit,
    Unaligned,
    OutOfBounds
}

pub type MainMemory = splitmem::SplitMemory;

impl<T: std::ops::DerefMut<Target = [u8]>> Memory for T {
    fn read_u32(&self, addr: u32) -> MemoryResult<u32> {
        read_u32_from_slice(self, addr as usize)
    }
    fn read_u16(&self, addr: u32) -> MemoryResult<u16> {
        read_u16_from_slice(self, addr as usize)
    }
    fn read_u8(&self, addr: u32) -> MemoryResult<u8> {
        self.get(addr as usize).map(|i| *i).ok_or(OutOfBounds)
    }
    fn read_slice(&self, addr: u32, len: u32) -> MemoryResult<&[u8]> {
        let addr = addr as usize;
        if addr >= self.len() {
            return Err(OutOfBounds)
        }
        let end = usize::max(addr + len as usize, self.len());
        Ok(&self[addr..end])
    }

    fn write_u32(&mut self, addr: u32, v: u32) -> MemoryResult<()> {
        write_u32_to_slice(self, addr as usize, v)
    }
    fn write_u16(&mut self, addr: u32, v: u16) -> MemoryResult<()> {
        write_u16_to_slice(self, addr as usize, v)
    }
    fn write_u8(&mut self, addr: u32, v: u8) -> MemoryResult<()> {
        *(self.get_mut(addr as usize).ok_or(OutOfBounds)?) = v;
        Ok(())
    }
}
