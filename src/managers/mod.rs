use crate::{
    graphics::gpu::InternalGraphics,
    managers::{
        material::{MaterialManager, MaterialType},
        mesh::{MeshData, MeshManager, Vertex},
    },
    world::components::{MaterialComponent, MeshComponent},
};

pub mod camera;
pub mod light;
pub mod material;
pub mod mesh;
pub mod transform;

#[derive(Debug)]
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

    pub fn create_cube(&mut self, graphics: &InternalGraphics) -> MeshComponent {
        let vertices = [
            // Front face (z = 0.5) — normal: [0, 0, 1]
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            // Back face (z = -0.5) — normal: [0, 0, -1]
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            // Right face (x = 0.5) — normal: [1, 0, 0]
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            // Left face (x = -0.5) — normal: [-1, 0, 0]
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [1.0, 0.0],
                normal: [-1.0, 0.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [-1.0, 0.0, 0.0],
            },
            // Top face (y = 0.5) — normal: [0, 1, 0]
            Vertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            Vertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            // Bottom face (y = -0.5) — normal: [0, -1, 0]
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
        ];

        let indices: &[u32] = &[
            0, 1, 2, 0, 2, 3, // Front
            4, 5, 6, 4, 6, 7, // Back
            8, 9, 10, 8, 10, 11, // Right
            12, 13, 14, 12, 14, 15, // Left
            16, 17, 18, 16, 18, 19, // Top
            20, 21, 22, 20, 22, 23, // Bottom
        ];

        let mut mesh_data = MeshData {
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
        };

        let handle = self.mesh_manager.add_mesh_data(&mut mesh_data, graphics);
        MeshComponent(handle)
    }

    pub fn create_prism(&mut self, graphics: &InternalGraphics) -> MeshComponent {
        let vertices: &mut [Vertex] = &mut [
            // Front face (z = 0.5) — normal: [0, 0, 1]
            Vertex {
                position: [0.0, 0.5, 0.5],
                tex_coords: [0.5, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            // Back face (z = -0.5) — normal: [0, 0, -1]
            Vertex {
                position: [0.0, 0.5, -0.5],
                tex_coords: [0.5, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            // Bottom face (y = -0.5) — normal: [0, -1, 0]
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            // Left face (A-B edge) — normal: perpendicular to edge, pointing outward
            // Normal for left face: [-0.894, 0.447, 0] (normalized)
            Vertex {
                position: [0.0, 0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [-0.894, 0.447, 0.0],
            },
            Vertex {
                position: [0.0, 0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [-0.894, 0.447, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [-0.894, 0.447, 0.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [-0.894, 0.447, 0.0],
            },
            // Right face (A-C edge) — normal: [0.894, 0.447, 0]
            Vertex {
                position: [0.0, 0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.894, 0.447, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.894, 0.447, 0.0],
            },
            Vertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.894, 0.447, 0.0],
            },
            Vertex {
                position: [0.0, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.894, 0.447, 0.0],
            },
        ];

        let indices: &[u32] = &[
            0, 1, 2, // Front
            3, 4, 5, // Back
            6, 7, 8, 6, 8, 9, // Bottom
            10, 11, 12, 10, 12, 13, // Left
            14, 15, 16, 14, 16, 17, // Right
        ];

        let mut mesh_data = MeshData {
            vertices: vertices.to_vec(),
            indices: indices.to_vec(),
        };

        let handle = self.mesh_manager.add_mesh_data(&mut mesh_data, graphics);
        MeshComponent(handle)
    }

    pub fn create_material(
        &mut self,
        gpu: &mut InternalGraphics,
        material_type: MaterialType,
    ) -> MaterialComponent {
        let handle = self.material_manager.add_new_material(material_type, gpu);
        MaterialComponent(handle)
    }
}
