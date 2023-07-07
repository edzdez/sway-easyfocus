mod sway;
mod ui;

fn main() -> anyhow::Result<()> {
    let conn = sway::acquire_connection()?;
    ui::run_ui(conn)
}
