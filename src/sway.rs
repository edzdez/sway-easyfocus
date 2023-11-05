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

// All output nodes, focused or not
pub fn get_all_output_nodes(conn: Arc<Mutex<Connection>>) -> Vec<Node> {
    let mut output_nodes = vec![];
    let mut q = VecDeque::new();
    let root_node = get_tree(conn);

    q.push_back(root_node);

    while !q.is_empty() {
        // We can unwrap because we know the queue is not empty
        let node = q.pop_back().unwrap();

        // If we hav an output node
        if (node.node_type == NodeType::Output) && !node.nodes.is_empty() {
            output_nodes.push(node.clone());
        }

        // Look for more outputs in the children
        for child in node.nodes {
            q.push_back(child.clone());
        }
    }
    output_nodes
}

pub fn get_focused_workspace(output: &Node) -> Node {
    output
        .clone()
        .find_focused(|n| n.node_type == swayipc::NodeType::Workspace)
        .expect("could not find focused workspace")
}

pub fn get_window_nodes_for_one_workspace(workspace: &Node) -> Vec<Node> {
    let mut nodes = vec![];

    let mut q = VecDeque::new();
    q.push_back(workspace.clone());
    while !q.is_empty() {
        // we can unwrap because we know that the queue is not empty
        let node = q.pop_back().unwrap();

        // if we have a window
        if (node.node_type == NodeType::Con || node.node_type == NodeType::FloatingCon)
            && node.nodes.is_empty()
        {
            nodes.push(node.clone());
        }

        // tiled/tabbed/stacked nodes
        for child in node.nodes {
            q.push_back(child.clone());
        }

        /*
        // TODO: floating nodes
        for child in node.floating_nodes {
            q.push_back(child.clone());
        }
        */
    }

    nodes.reverse();
    nodes
}

pub fn focus(con_id: i64) {
    acquire_connection()
        .run_command(format!("[con_id={}] focus", con_id))
        .expect("failed to focus container");
}
