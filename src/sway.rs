use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use swayipc::*;

pub fn acquire_connection() -> Connection {
    swayipc::Connection::new().expect("failed to connect to sway")
}

pub fn get_tree(conn: Arc<Mutex<Connection>>) -> Node {
    let mut conn_lock = conn.lock().unwrap();
    conn_lock
        .get_tree()
        .expect("failed to communicate with sway")
}

pub fn get_focused_workspace(conn: Arc<Mutex<Connection>>) -> Node {
    let root_node = get_tree(conn);
    let focused_workspace = root_node
        .find_focused(|n| n.node_type == swayipc::NodeType::Output)
        .expect("could not find focused output")
        .find_focused(|n| n.node_type == swayipc::NodeType::Workspace)
        .expect("could not find focused workspace");

    focused_workspace
}

pub fn get_all_windows(workspace: &Node) -> Vec<Node> {
    let mut nodes = vec![];

    let mut q = VecDeque::new();
    q.push_back(workspace.clone());
    while !q.is_empty() {
        // we can unwrap because we know that the queue is not empty
        let node = q.pop_back().unwrap();

        // if we have a window
        if node.node_type == NodeType::Con && node.nodes.is_empty() {
            nodes.push(node.clone());
        }

        for child in node.nodes {
            q.push_back(child.clone());
        }
    }

    nodes.reverse();
    nodes
}

pub fn focus(conn: Arc<Mutex<Connection>>, windows: &[Node], idx: usize) {
    let con_id = windows[idx].id;
    let mut conn_lock = conn.lock().unwrap();
    conn_lock
        .run_command(format!("[con_id={}] focus", con_id))
        .expect("failed to focus container");
}
