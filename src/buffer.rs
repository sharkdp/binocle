use std::{
    fs::File,
    io::{self, BufReader, Read},
    ops::Index,
    path::Path,
};

pub struct Buffer {
    data: Vec<u8>,
}

impl Buffer {
    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut data: Vec<u8> = vec![];

        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        reader.read_to_end(&mut data)?;

        return Ok(Buffer { data });
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl Index<usize> for Buffer {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
