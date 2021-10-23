use memmap2::Mmap;
use std::io::{BufReader, Read};
use std::{fs::File, io, path::Path};

pub struct MMapBacking {
    _file: File,
    pub mmap: Mmap,
}

impl MMapBacking {
    pub fn new(_file: File, mmap: Mmap) -> Self {
        MMapBacking { _file, mmap }
    }
}

pub enum Buffer {
    VecBuffer(Vec<u8>),
    MmapBuffer(MMapBacking),
}

impl Buffer {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut data: Vec<u8> = vec![];

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        reader.read_to_end(&mut data)?;

        return Ok(Buffer::VecBuffer(data));
    }

    pub fn from_mmap<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        return Ok(Buffer::MmapBuffer(MMapBacking::new(file, mmap)));
    }

    pub fn len(&self) -> usize {
        match self {
            Buffer::VecBuffer(data) => data.len(),
            Buffer::MmapBuffer(mmap) => mmap.mmap.len(),
        }
    }

    pub fn data(&self) -> &[u8] {
        match self {
            Buffer::VecBuffer(data) => &data,
            Buffer::MmapBuffer(mmap) => &mmap.mmap,
        }
    }
}
