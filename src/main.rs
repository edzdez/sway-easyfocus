mod sway;
mod ui;

fn main() -> anyhow::Result<()> {
    let mut conn = sway::acquire_connection()?;
    let focused_workspace = sway::get_focused_workspace(&mut conn)?;
    dbg!(&focused_workspace);

    // ui::run_ui();

    Ok(())
}
