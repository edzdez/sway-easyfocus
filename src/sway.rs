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

pub fn get_focused_output(conn: Arc<Mutex<Connection>>) -> Node {
    let root_node = get_tree(conn);
    root_node
        .find_focused(|n| n.node_type == swayipc::NodeType::Output)
        .expect("could not find focused output")
}

pub fn get_focused_workspace(output: &Node) -> Node {
    output
        .clone()
        .find_focused(|n| n.node_type == swayipc::NodeType::Workspace)
        .expect("could not find focused workspace")
}

pub fn get_all_windows(workspace: &Node) -> Vec<Node> {
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
        for child in &node.nodes {
            let mut c = child.clone();
            // a bit of a hack to keep track of stacked/tabbed layouts:
            // if we're stacked, we set our childrens' layouts to stacked and change the decorator
            // height.
            if node.node_type == NodeType::Con && node.layout == NodeLayout::Stacked {
                c.layout = node.layout;
                // change the decoration height to be the *total height* of all the decorations in
                // the stacked container.
                c.deco_rect.height *= node.nodes.len() as i32;
            }
            q.push_back(c);
        }

        /*
        // floating nodes
        for child in node.floating_nodes {
            q.push_back(child.clone());
        }
        */
    }

    nodes.reverse();
    // dbg!(&nodes);
    nodes
}

pub fn focus(conn: Arc<Mutex<Connection>>, con_id: i64) {
    let mut conn_lock = conn.lock().unwrap();
    conn_lock
        .run_command(format!("[con_id={}] focus", con_id))
        .expect("failed to focus container");
}

pub fn swap(conn: Arc<Mutex<Connection>>, con_id: i64) {
    let mut conn_lock = conn.lock().unwrap();
    conn_lock
        .run_command(format!("swap container with con_id {}", con_id))
        .expect("failed to swap container");
}
