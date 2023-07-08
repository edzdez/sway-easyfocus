use std::sync::{Arc, Mutex};

mod sway;
mod ui;

fn main() {
    let conn = Arc::new(Mutex::new(sway::acquire_connection()));
    ui::run_ui(conn);
}
