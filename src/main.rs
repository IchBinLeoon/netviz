use std::process::exit;

use clap::Parser;

use crate::app::App;

mod app;
mod history;
mod utils;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli;

fn main() {
    Cli::parse();

    let mut app = App::init();

    if let Err(e) = app.start() {
        eprintln!("{e}");
        exit(1);
    }
}
