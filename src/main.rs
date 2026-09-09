use crate::{
    app::app::App,
    graphics::gpu::InternalGraphics,
    managers::{Assets, transform::Transform},
    systems::system_struct::SystemAggregator,
    world::{components::TransformComponent, world::World},
};

pub mod app;
pub mod camera;
pub mod graphics;
pub mod layouts;
pub mod managers;
pub mod pipeline;
pub mod render;
pub mod systems;
pub mod utils;
pub mod world;

fn main() -> anyhow::Result<()> {
    App::new()?.insert_system(show_system).run()
}

pub fn instantiate_mesh(world: &mut World) {
    let mut graphics = world.get_mut::<InternalGraphics>();
    let mut assets = world.get_mut::<Assets>();

    let cube_mesh = assets.create_cube(&graphics);
    let prism_mesh = assets.create_prism(&graphics);
    let tree_material = assets.create_material(
        &mut graphics,
        managers::material::MaterialType::Textured {
            location: "src/assets/happy-tree.png".to_string(),
        },
    );
    let color_mat = assets.create_material(
        &mut graphics,
        managers::material::MaterialType::NonTexture {
            color: [0.0, 0.0, 1.0],
        },
    );

    drop(assets);
    drop(graphics);

    // I could get the vector of the typeId interestingly
    world.spawn((cube_mesh, tree_material, TransformComponent::default()));
    world.spawn((
        prism_mesh,
        color_mat,
        TransformComponent(Transform::new([1.0, 2.0, 3.0])),
    ));
    world.spawn((
        cube_mesh,
        color_mat,
        TransformComponent(Transform::new([8.0, 2.0, 3.0])),
    ));
}

pub fn show_system(system: &mut SystemAggregator) {
    system.insert_init_system(instantiate_mesh);
}

use std::collections::{HashMap, VecDeque};
use std::hash::Hash;

// --- 1. The Generic Utility ---

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

// --- 2. Your System Setup ---

pub type SystemID = String;

pub trait SystemFunction {
    type Fntype: ?Sized;
}

#[derive(Debug, Clone)]
pub enum SystemSort {
    Before(SystemID),
    After(SystemID),
}

// Box<dyn Fn()> doesn't implement Debug, so we manually implement it for SystemMut
pub struct SystemMut<A: SystemFunction> {
    pub id: SystemID,
    pub sorts: Vec<SystemSort>,
    pub callback: Box<A::Fntype>,
}

impl<A: SystemFunction> std::fmt::Debug for SystemMut<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SystemMut")
            .field("id", &self.id)
            .field("sorts", &self.sorts)
            .finish()
    }
}

impl<A: SystemFunction> NodeTrait for SystemMut<A> {
    type ID = SystemID;

    fn node_id(&self) -> &Self::ID {
        &self.id
    }

    fn prev_ids(&self) -> Vec<Self::ID> {
        self.sorts
            .iter()
            .filter_map(|sort| match sort {
                SystemSort::After(id) => Some(id.clone()),
                _ => None,
            })
            .collect()
    }

    fn next_ids(&self) -> Vec<Self::ID> {
        self.sorts
            .iter()
            .filter_map(|sort| match sort {
                SystemSort::Before(id) => Some(id.clone()),
                _ => None,
            })
            .collect()
    }
}
