mod binocle;
mod event_loop;
mod gui;
mod settings;

fn main() -> Result<(), pixels::Error> {
    event_loop::run()
}
