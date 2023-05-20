use crate::memory::*;
use std::io::{
    Read, Write
};
use file::*;
use crate::vm::instruction::InsData;

use std::collections::VecDeque;

mod file;

pub struct IoHandler {
    files: FileTable,

    stdout: VecDeque<u8>,
    stderr: VecDeque<u8>,
    stdin: VecDeque<u8>,
}

impl IoHandler {
    const NUM_VIO: u32 = 32;

    pub fn new() -> Self {
        let files = FileTable::new(Self::NUM_VIO);
        let stdout = VecDeque::new();
        let stderr = VecDeque::new();
        let stdin = VecDeque::new();

        Self {
            files,
            stdin, stdout, stderr
        }
    }
    pub fn io<M: Memory>(&mut self, funct: u32, i: InsData, mem: &mut M) -> IoResult<u32> {
        let fd = i.s2;
        match funct {
            64 => {
                self.write_one(i.s1, fd).map(|_| 0)
            }
            _ => todo!()
        }
    }

    fn write_one(&mut self, v: u32, fd: u32) -> IoResult<()> {
        match fd {
            1 => self.stdout.push_back(v as u8),
            2 => self.stderr.push_back(v as u8),
            x if x >= Self::NUM_VIO => todo!("no real files yet"),
            _x => return Err(IoError::BadFd)
        }

        Ok(())
    }
}

pub type IoResult<T> = Result<T, IoError>;
#[derive(Debug)]
pub enum IoError {
    Other = 1,
    NotFound = 2,
    InvalidParams = 3,
    InvalidData = 4,
    BrokenPipe = 5,
    PermissionDenied = 6,
    BadFd = 7,
    Empty = 8,
}

impl From<std::io::Error> for IoError {
    fn from(e: std::io::Error) -> Self {
        use std::io::ErrorKind as EK;
        use IoError::*;

        match e.kind() {
            EK::PermissionDenied => PermissionDenied,
            EK::NotFound => NotFound,
            EK::InvalidInput => InvalidParams,
            EK::InvalidData => InvalidData,
            EK::BrokenPipe => BrokenPipe,
            _ => Other
        }
    }
}
