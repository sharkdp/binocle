use memmap2::Mmap;
use std::{fs::File, io, path::Path};

pub struct Buffer {
    mmap: Mmap,
}

impl Buffer {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        return Ok(Buffer { mmap });
    }

    pub fn len(&self) -> usize {
        self.mmap.len()
    }

    pub fn data(&self) -> &[u8] {
        &self.mmap
    }
}
