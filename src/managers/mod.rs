use crate::managers::{material::MaterialManager, mesh::MeshManager};

pub mod light;
pub mod material;
pub mod mesh;
pub mod transform;

pub struct Manager {
    pub mesh_manager: MeshManager,
    pub material_manager: MaterialManager,
}

impl Manager {
    pub fn new() -> Manager {
        Self {
            mesh_manager: MeshManager::new(),
            material_manager: MaterialManager::new(),
        }
    }
}
