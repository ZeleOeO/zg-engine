use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

pub trait NodeTrait {
    type ID: Clone + Eq + Hash + Ord;
    fn node_id(&self) -> &Self::ID;

    fn prev_ids(&self) -> Vec<Self::ID>;
    fn next_ids(&self) -> Vec<Self::ID>;
}

pub fn sort_vector<N: NodeTrait>(vec: &mut Vec<N>) -> bool {
    let n = vec.len();

    let mut id_to_index: HashMap<N::ID, usize> = HashMap::new();
    for (i, item) in vec.iter().enumerate() {
        id_to_index.insert(item.node_id().clone(), i);
    }

    let mut outgoing_edges: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut in_degree: Vec<usize> = vec![0; n];

    for (i, item) in vec.iter().enumerate() {
        for prev_id in item.prev_ids() {
            if let Some(&prev_idx) = id_to_index.get(&prev_id) {
                outgoing_edges[prev_idx].push(i);
                in_degree[i] += 1;
            }
        }
        for next_id in item.next_ids() {
            if let Some(&next_idx) = id_to_index.get(&next_id) {
                outgoing_edges[i].push(next_idx);
                in_degree[next_idx] += 1;
            }
        }
    }

    let mut queue: VecDeque<usize> = (0..n).filter(|&i| in_degree[i] == 0).collect();
    let mut sorted_indices = Vec::with_capacity(n);

    while let Some(idx) = queue.pop_front() {
        sorted_indices.push(idx);
        for &next_idx in &outgoing_edges[idx] {
            in_degree[next_idx] -= 1;
            if in_degree[next_idx] == 0 {
                queue.push_back(next_idx);
            }
        }
    }

    if sorted_indices.len() != n {
        return false;
    }

    let mut old_items: Vec<Option<N>> = std::mem::take(vec).into_iter().map(Some).collect();
    let mut new_items = Vec::with_capacity(n);
    for idx in sorted_indices {
        if let Some(item) = old_items[idx].take() {
            new_items.push(item);
        }
    }

    *vec = new_items;
    true
}
