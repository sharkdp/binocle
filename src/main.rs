use std::env;
use std::process;

mod binocle;
mod buffer;
mod event_loop;
mod gui;
mod settings;
mod style;
mod view;

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
