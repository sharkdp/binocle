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

    let _puffin_server = if cfg!(feature = "trace") {
        let server_addr = format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT);
        eprintln!("Serving profile data on {}", server_addr);
        puffin::set_scopes_on(true);
        Some(puffin_http::Server::new(&server_addr)?)
    } else {
        None
    };

    event_loop::run(options)
}
