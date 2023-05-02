use std::collections::BTreeMap;
use std::fs::File;

/// maps raven fds onto underlying system files
/// 
/// DOES NOT handle virtual io files
pub struct FileTable {
    files: BTreeMap<u32, File>,
    next_id: u32,
    returned_ids: Vec<u32>
}
impl FileTable {
    fn next_id(&mut self) -> u32 {
        self.returned_ids.pop().unwrap_or_else(|| {
            let id = self.next_id;
            self.next_id = self.next_id + 1;
            if self.next_id > 2u32.pow(31) {
                panic!("out of file descriptors!")
            }
            id
        })
    }

    /// set first_id to the lowest non-vio file descriptor
    pub fn new(first_id: u32) -> Self {
        Self {
            files: BTreeMap::new(),
            next_id: first_id,
            returned_ids: Vec::new()
        }
    }

    pub fn get_mut(&mut self, fd: u32) -> Option<&mut File> {
        self.files.get_mut(&fd)
    }
    pub fn close(&mut self, fd: u32) -> FileResult<()> {
        self.returned_ids.push(fd);
        if let Some(f) = self.files.remove(&fd) {
            f.sync_all()?; // catch any close errors that would be ignored when dropping the File
            drop(f); // this would happen anyway but it just makes it clear what's happening
        }

        Ok(())
    }
}

pub type FileResult<T> = Result<T, FileError>;
#[derive(Debug)]
pub enum FileError {
    Other = 1,
    NotFound = 2,
    InvalidParams = 3,
    InvalidData = 4,
    BrokenPipe = 5,
    PermissionDenied = 6,
}

impl From<std::io::Error> for FileError {
    fn from(e: std::io::Error) -> Self {
        use std::io::ErrorKind as EK;
        use FileError::*;

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
