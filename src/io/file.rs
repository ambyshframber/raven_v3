use std::collections::BTreeMap;
use std::fs::{File, ReadDir};
use super::IoResult;

/// maps raven fds onto underlying system files
/// 
/// DOES NOT handle virtual io files
pub struct FileTable {
    files: BTreeMap<u32, RFile>,
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

    pub fn get_mut(&mut self, fd: u32) -> Option<&mut RFile> {
        self.files.get_mut(&fd)
    }
    pub fn close(&mut self, fd: u32) -> IoResult<()> {
        self.returned_ids.push(fd);
        if let Some(f) = self.files.remove(&fd) {
            if let RFile::File(f) = f {
                f.sync_all()?
            }; // catch any close errors that would be ignored when dropping the File
        }

        Ok(())
    }
}

pub enum RFile {
    File(File),
    Directory(ReadDir)
}


