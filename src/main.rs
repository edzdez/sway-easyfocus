use std::sync::{Arc, Mutex};

use clap::Parser;

use cli::Args;

mod cli;
mod sway;
mod ui;
mod utils;

fn main() {
    let args = Arc::new(Args::parse());

    let conn = Arc::new(Mutex::new(sway::acquire_connection()));
    ui::run_ui(conn, args);
}
