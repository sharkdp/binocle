use clap::{Parser, crate_version};
use std::str::FromStr;

#[derive(Parser)]
#[clap(version = crate_version!(), author = "David Peter <mail@david-peter.de>")]
pub struct CliOptions {
    pub filename: String,
    #[clap(
        name = "backing",
        long,
        default_value = "mmap",
        about = "Valid options: mmap, file"
    )]
    pub backing: BackingOption,
}

pub enum BackingOption {
    File,
    Mmap,
}

impl FromStr for BackingOption {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "file" => Ok(BackingOption::File),
            "mmap" => Ok(BackingOption::Mmap),
            _ => Err(anyhow::Error::msg("Could not parse backing option")),
        }
    }
}
