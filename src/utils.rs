use crate::memory::{MemoryError::*, MemoryResult};

pub fn extract_5_bits(i: u32, idx: u32) -> u32 {
    (i & (0b1_1111 << idx)) >> idx
}

pub trait ResultInner<T> {
    fn inner(self) -> T;
}
impl<T> ResultInner<T> for Result<T, T> {
    fn inner(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => e
        }
    }
}

macro_rules! read_t_from_slice {
    ($name:ident, $t:ty) => {
        pub fn $name(s: &[u8], idx: usize) -> MemoryResult<$t> {
            if s.len() < idx + std::mem::size_of::<$t>() {
                return Err(OutOfBounds)
            }
            if idx % std::mem::size_of::<$t>() != 0 {
                return Err(Unaligned)
            }
            unsafe {
                let wptr = s.as_ptr().offset(idx as isize) as *mut $t;
                Ok(<$t>::from_le(*wptr))
            }
        }
    };
}
macro_rules! write_t_to_slice {
    ($name:ident, $t:ty) => {
        pub fn $name(s: &mut [u8], idx: usize, v: $t) -> MemoryResult<()> {
            if s.len() < idx + std::mem::size_of::<$t>() {
                return Err(OutOfBounds)
            }
            if idx % std::mem::size_of::<$t>() != 0 {
                return Err(Unaligned)
            }
            unsafe {
                let wptr = s.as_mut_ptr().offset(idx as isize) as *mut $t;
                *wptr = v.to_le()
            }
            Ok(())
        }
    };
}

read_t_from_slice!(read_u32_from_slice, u32);
read_t_from_slice!(read_u16_from_slice, u16);
write_t_to_slice!(write_u32_to_slice, u32);
write_t_to_slice!(write_u16_to_slice, u16);
