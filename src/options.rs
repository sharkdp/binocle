use clap::{ArgEnum, Parser};

#[derive(Parser)]
#[clap(version, about)]
pub struct CliOptions {
    pub filename: String,

    /// Whether to use memory mapping to read the file contents or not
    #[clap(long, arg_enum, default_value = "mmap")]
    pub backing: BackingOption,
}

#[derive(ArgEnum, Copy, Clone)]
pub enum BackingOption {
    File,
    Mmap,
}
