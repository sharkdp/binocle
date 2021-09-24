use std::env;
use std::process;

mod binocle;
mod event_loop;
mod gui;
mod settings;

fn main() -> anyhow::Result<()> {
    let mut args = env::args_os();
    args.next();

    if let Some(filename) = args.next() {
        event_loop::run(&filename)
    } else {
        eprintln!("Usage: binocle <file>");
        process::exit(1);
    }
}
