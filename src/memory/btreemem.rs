use std::collections::BTreeMap;

use super::*;
use crate::utils::*;

pub struct BTreeMemory {
    blocks: BTreeMap<u32, [u8; Self::BLOCK_SIZE]>,
}
impl BTreeMemory {
    const BLOCK_SIZE_LOG_2: usize = 12;
    const BLOCK_SIZE: usize = 1 << Self::BLOCK_SIZE_LOG_2;

    pub fn new() -> Self {
        Self {
            blocks: BTreeMap::new()
        }
    }
    fn split_addr(a: u32) -> (u32, usize) {
        let block = a >> Self::BLOCK_SIZE_LOG_2;
        let word = (a as usize) & (Self::BLOCK_SIZE - 1);
        (block, word)
    }

    fn modify_block<T, F: FnOnce(&mut [u8; Self::BLOCK_SIZE]) -> T>(&mut self, addr: u32, f: F) -> T {
        if let Some(b) = self.blocks.get_mut(&addr) {
            f(b)
        }
        else {
            let mut b = [0; Self::BLOCK_SIZE];
            let ret = f(&mut b);
            self.blocks.insert(addr, b);
            ret
        }
    }
}

impl Memory for BTreeMemory {
    fn read_u32(&self, addr: u32) -> MemoryResult<u32> {
        let (block_a, byte_a) = Self::split_addr(addr);
        let block = self.blocks.get(&block_a).ok_or(Uninit)?;
        read_u32_from_slice(block, byte_a)
    }
    fn read_u16(&self, addr: u32) -> MemoryResult<u16> {
        let (block_a, byte_a) = Self::split_addr(addr);
        let block = self.blocks.get(&block_a).ok_or(Uninit)?;
        read_u16_from_slice(block, byte_a)
    }
    fn read_u8(&self, addr: u32) -> MemoryResult<u8> {
        let (block_a, byte_a) = Self::split_addr(addr);
        let block = self.blocks.get(&block_a).ok_or(Uninit)?;
        Ok(block[byte_a])
    }

    fn read_slice(&self, addr: u32, len: u32) -> MemoryResult<&[u8]> {
        let (block_a, byte_a) = Self::split_addr(addr);
        let block = self.blocks.get(&block_a).ok_or(Uninit)?;

        let len = len as usize;

        if Self::BLOCK_SIZE - byte_a < len {
            Ok(&block[byte_a..Self::BLOCK_SIZE])
        }
        else {
            Ok(&block[byte_a..byte_a + len])
        }
    }

    fn write_u32(&mut self, addr: u32, v: u32) -> MemoryResult<()> {
        let (block_a, byte_a) = Self::split_addr(addr);
        self.modify_block(block_a, |block| {
            write_u32_to_slice(block, byte_a, v)
        })
    }
    fn write_u16(&mut self, addr: u32, v: u16) -> MemoryResult<()> {
        let (block_a, byte_a) = Self::split_addr(addr);
        self.modify_block(block_a, |block| {
            write_u16_to_slice(block, byte_a, v)
        })
    }
    fn write_u8(&mut self, addr: u32, v: u8) -> MemoryResult<()> {
        let (block_a, byte_a) = Self::split_addr(addr);
        self.modify_block(block_a, |block| block[byte_a] = v);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::*;

    #[test]
    fn mem() {
        let mut m = BTreeMemory::new();

        for i in 0..256 {
            m.write_u32(i as u32 * 4, i as u32).unwrap();
        }
        for i in 256..0 {
            assert_eq!(m.read_u32(i as u32 * 4), Ok(i))
        }

        assert_eq!(m.write_u32(1, 0), Err(MemoryError::Unaligned));
        assert_eq!(m.read_u32(0x10_0000), Err(MemoryError::Uninit));

        m.write_u32(0, 0x1234_5678).unwrap();
        assert_eq!(m.read_u8(0), Ok(0x78));
        assert_eq!(m.read_u8(1), Ok(0x56));
        assert_eq!(m.read_u8(3), Ok(0x12));
    }
}