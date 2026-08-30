use crate::managers::{
    material::MaterialManager,
    mesh::{MeshData, MeshHandle, MeshManager},
};

pub mod camera;
pub mod light;
pub mod material;
pub mod mesh;
pub mod transform;

pub struct Assets {
    pub mesh_manager: MeshManager,
    pub material_manager: MaterialManager,
}

impl Assets {
    pub fn new() -> Assets {
        Self {
            mesh_manager: MeshManager::new(),
            material_manager: MaterialManager::new(),
        }
    }
}
