use super::*;
use btreemem::BTreeMemory;
use crate::utils::*;

pub struct SplitMemory {
    object: Vec<u8>,
    data: BTreeMemory
}
impl SplitMemory {
    pub fn new(object: Vec<u8>) -> MemoryResult<Self> {
        if object.len() % 4 != 0 {
            Err(Unaligned)
        }
        else {
            Ok(Self {
                object,
                data: BTreeMemory::new()
            })
        }
    }
}
impl Memory for SplitMemory {
    fn read_u32(&self, addr: u32) -> MemoryResult<u32> {
        if addr >= self.object.len() as u32 {
            self.data.read_u32(addr)
        }
        else {
            self.object.read_u32(addr)
        }
    }
    fn read_u16(&self, addr: u32) -> MemoryResult<u16> {
        if addr >= self.object.len() as u32 {
            self.data.read_u16(addr)
        }
        else {
            self.object.read_u16(addr)
        }
    }
    fn read_u8(&self, addr: u32) -> MemoryResult<u8> {
        if addr >= self.object.len() as u32 {
            self.data.read_u8(addr)
        }
        else {
            Ok(self.object[addr as usize])
        }
    }
    fn read_slice(&self, addr: u32, len: u32) -> MemoryResult<&[u8]> {
        if addr < self.object.len() as u32 {
            self.object.read_slice(addr, len)
        }
        else {
            self.data.read_slice(addr, len)
        }
    }

    fn write_u32(&mut self, addr: u32, v: u32) -> MemoryResult<()> {
        if addr < self.object.len() as u32 {
            self.object.write_u32(addr, v)
        }
        else {
            self.data.write_u32(addr, v)
        }
    }
    fn write_u16(&mut self, addr: u32, v: u16) -> MemoryResult<()> {
        if addr < self.object.len() as u32 {
            self.object.write_u16(addr, v)
        }
        else {
            self.data.write_u16(addr, v)
        }
    }
    fn write_u8(&mut self, addr: u32, v: u8) -> MemoryResult<()> {
        if addr < self.object.len() as u32 {
            self.object.write_u8(addr, v)
        }
        else {
            self.data.write_u8(addr, v)
        }
    }
}
