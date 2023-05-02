pub trait Memory {
    fn read_u32(&self, addr: u32) -> MemoryResult<u32>;
    fn read_u16(&self, addr: u32) -> MemoryResult<u16>;
    fn read_u8(&self, addr: u32) -> MemoryResult<u8>;

    fn write_u32(&mut self, addr: u32, v: u32) -> MemoryResult<()>;
    fn write_u16(&mut self, addr: u32, v: u16) -> MemoryResult<()>;
    fn write_u8(&mut self, addr: u32, v: u8) -> MemoryResult<()>;
}

pub type MemoryResult<T> = Result<T, MemoryError>;
use MemoryError::*;
#[derive(Debug)]
pub enum MemoryError {
    Uninit,
    Unaligned
}

use std::collections::BTreeMap;

pub struct MainMemory {
    blocks: BTreeMap<u32, [u8; 4096]>,
}
impl MainMemory {
    pub fn new() -> Self {
        Self {
            blocks: BTreeMap::new()
        }
    }
    fn split_addr(a: u32) -> (u32, usize) {
        let block = a >> 12;
        let word = (a as usize) & (4096 - 1);
        (block, word)
    }

    fn modify_block<F: FnOnce(&mut [u8; 4096])>(&mut self, addr: u32, f: F) {
        if let Some(b) = self.blocks.get_mut(&addr) {
            f(b)
        }
        else {
            let mut b = [0; 4096];
            f(&mut b);
            self.blocks.insert(addr, b);
        }
    }
}

impl Memory for MainMemory {
    fn read_u32(&self, addr: u32) -> MemoryResult<u32> {
        if addr & 0b11 != 0 {
            return Err(Unaligned)
        }
        let (block_a, byte_a) = Self::split_addr(addr);
        let block = self.blocks.get(&block_a).ok_or(Uninit)?;
        unsafe { // byte_a is in 0..4092 so can't overflow
            let wptr = block.as_ptr().offset(byte_a as isize) as *const u32;
            Ok(u32::from_le(*wptr))
        }
    }
    fn read_u16(&self, addr: u32) -> MemoryResult<u16> {
        if addr & 0b1 != 0 {
            return Err(Unaligned)
        }
        let (block_a, byte_a) = Self::split_addr(addr);
        let block = self.blocks.get(&block_a).ok_or(Uninit)?;
        unsafe { // byte_a is in 0..4094 so can't overflow
            let wptr = block.as_ptr().offset(byte_a as isize) as *const u16;
            Ok(u16::from_le(*wptr))
        }
    }
    fn read_u8(&self, addr: u32) -> MemoryResult<u8> {
        let (block_a, byte_a) = Self::split_addr(addr);
        let block = self.blocks.get(&block_a).ok_or(Uninit)?;
        Ok(block[byte_a])
    }

    fn write_u32(&mut self, addr: u32, v: u32) -> MemoryResult<()> {
        if addr & 0b11 != 0 {
            return Err(Unaligned)
        }
        let (block_a, byte_a) = Self::split_addr(addr);
        self.modify_block(block_a, |block| {
            unsafe {
                let wptr = block.as_mut_ptr().offset(byte_a as isize) as *mut u32;
                *wptr = v.to_le()
            }
        });
        Ok(())
    }
    fn write_u16(&mut self, addr: u32, v: u16) -> MemoryResult<()> {
        if addr & 0b1 != 0 {
            return Err(Unaligned)
        }
        let (block_a, byte_a) = Self::split_addr(addr);
        self.modify_block(block_a, |block| {
            unsafe {
                let wptr = block.as_mut_ptr().offset(byte_a as isize) as *mut u16;
                *wptr = v.to_le()
            }
        });
        Ok(())
    }
    fn write_u8(&mut self, addr: u32, v: u8) -> MemoryResult<()> {
        let (block_a, byte_a) = Self::split_addr(addr);
        self.modify_block(block_a, |block| block[byte_a] = v);
        Ok(())
    }
}
