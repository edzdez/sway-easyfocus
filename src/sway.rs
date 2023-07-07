use std::collections::VecDeque;

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

pub fn get_all_windows(workspace: &Node) -> Vec<Node> {
    let mut nodes = vec![];

    let mut q = VecDeque::new();
    q.push_back(workspace.clone());
    while !q.is_empty() {
        let node = q.pop_back().unwrap(); // we know that the queue is not empty

        if node.nodes.is_empty() {
            // we have a window
            nodes.push(node.clone());
        }

        for child in node.nodes {
            q.push_back(child.clone());
        }
    }

    nodes.reverse();
    nodes
}

pub fn focus(idx: usize) {
    // TODO: REFACTORRRRR
    let mut conn = acquire_connection().expect("failed to connect to sway");
    let windows = get_all_windows(
        &get_focused_workspace(&mut conn).expect("failed to communicate with sway"),
    );

    let con_id = windows[idx].id;
    conn.run_command(format!("[con_id={}] focus", con_id))
        .expect("failed to communicate with sway");
}
