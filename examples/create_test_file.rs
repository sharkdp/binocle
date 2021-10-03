use std::fs::File;
use std::io::prelude::*;
use std::io::Result;

struct Writer {
    file: File,
}

impl Writer {
    fn new() -> Result<Writer> {
        Ok(Writer {
            file: File::create("binocle_test_file")?,
        })
    }

    fn section(&mut self, title: &str) -> Result<()> {
        self.file.write_all(&vec![0x42; 10])?;
        self.file.write_all(title.as_bytes())?;
        self.file.write_all(&vec![0x42; 10])?;
        Ok(())
    }

    fn add(&mut self, buf: &[u8]) -> Result<()> {
        self.file.write_all(buf)?;
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let mut writer = Writer::new()?;
    {
        writer.section("u8")?;
        let numbers = (u8::MIN..=u8::MAX).collect::<Vec<_>>();
        writer.add(&numbers)?;
    }
    {
        writer.section("i8")?;
        let numbers = (i8::MIN..=i8::MAX)
            .flat_map(|i| i.to_le_bytes())
            .collect::<Vec<_>>();
        writer.add(&numbers)?;
    }
    {
        writer.section("u16 LE")?;
        let numbers = (u16::MIN..=u16::MAX)
            .step_by(64)
            .flat_map(|i| i.to_le_bytes())
            .collect::<Vec<_>>();
        writer.add(&numbers)?;
    }
    {
        writer.section("u32 BE")?;
        let numbers = (0u32..=100u32)
            .flat_map(|i| i.to_be_bytes())
            .collect::<Vec<_>>();
        writer.add(&numbers)?;
    }
    Ok(())
}
