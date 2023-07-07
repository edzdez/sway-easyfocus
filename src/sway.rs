use anyhow::{Context, Result};
use swayipc::*;

pub fn acquire_connection() -> Result<Connection> {
    swayipc::Connection::new().context("failed to connect to sway")
}

pub fn get_tree(conn: &mut Connection) -> Result<Node> {
    conn.get_tree().context("failed to communicate with sway")
}

pub fn get_focused_workspace(conn: &mut Connection) -> Result<Node> {
    let root_node = get_tree(conn)?;
    let focused_workspace = root_node
        .find_focused(|n| n.node_type == swayipc::NodeType::Output)
        .unwrap()
        .find_focused(|n| n.node_type == swayipc::NodeType::Workspace)
        .unwrap();

    Ok(focused_workspace)
}
