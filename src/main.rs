use crate::options::CliOptions;
use clap::Parser;

mod binocle;
mod buffer;
mod datatype;
mod event_loop;
mod gui;
mod options;
mod settings;
mod style;
mod view;

fn main() -> anyhow::Result<()> {
    let options = CliOptions::parse();
    event_loop::run(options)
}
