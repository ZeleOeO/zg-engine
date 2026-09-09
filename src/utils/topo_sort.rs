use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct NodeID(pub u32);

#[derive(Clone)]
pub struct Node {
    pub node_id: NodeID,
    pub prev_nodes: Vec<NodeID>,
    pub next_nodes: Vec<NodeID>,
}

#[derive(Clone)]
pub struct Graph {
    pub nodes: HashMap<NodeID, Node>,
}

pub fn sort_node(graph: &mut Graph) {
    let mut edges: Vec<(NodeID, NodeID)> = graph
        .nodes
        .iter()
        .flat_map(|(&id, node)| node.prev_nodes.iter().map(move |&prev_id| (prev_id, id)))
        .collect();

    edges.sort_by_key(|&(prev_id, id)| (prev_id.0, id.0));
    edges.dedup();

    for (prev_id, id) in edges {
        if let Some(prev_node) = graph.nodes.get_mut(&prev_id) {
            if !prev_node.next_nodes.contains(&id) {
                prev_node.next_nodes.push(id);
            }
        }
    }
}

pub fn topological_sort(graph: &mut Graph) -> Option<Vec<NodeID>> {
    sort_node(graph);

    let mut in_degree: HashMap<NodeID, usize> = graph.nodes.keys().map(|&id| (id, 0)).collect();
    for node in graph.nodes.values() {
        for &next_id in &node.next_nodes {
            *in_degree.entry(next_id).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<NodeID> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&id, _)| id)
        .collect();

    let mut order = Vec::with_capacity(graph.nodes.len());

    while let Some(id) = queue.pop_front() {
        order.push(id);
        if let Some(node) = graph.nodes.get(&id) {
            for &next_id in &node.next_nodes {
                let deg = in_degree.get_mut(&next_id).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(next_id);
                }
            }
        }
    }

    if order.len() == graph.nodes.len() {
        Some(order)
    } else {
        None
    }
}
