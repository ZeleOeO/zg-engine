use zg_graphics::InternalGraphics;

use crate::{
    material::{MaterialManager, MaterialType},
    mesh::{MeshData, MeshManager},
};
use zg_utils::ModelVertex;
use zg_world::components::{MaterialComponent, MeshComponent};

pub mod light;
pub mod material;
pub mod mesh;

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
            ModelVertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            ModelVertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            ModelVertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            ModelVertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            // Back face (z = -0.5) — normal: [0, 0, -1]
            ModelVertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            ModelVertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            ModelVertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            ModelVertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            // Right face (x = 0.5) — normal: [1, 0, 0]
            ModelVertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [1.0, 0.0, 0.0],
            },
            ModelVertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [1.0, 0.0, 0.0],
            },
            ModelVertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            ModelVertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [1.0, 0.0, 0.0],
            },
            // Left face (x = -0.5) — normal: [-1, 0, 0]
            ModelVertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
            },
            ModelVertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [-1.0, 0.0, 0.0],
            },
            ModelVertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [1.0, 0.0],
                normal: [-1.0, 0.0, 0.0],
            },
            ModelVertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [-1.0, 0.0, 0.0],
            },
            // Top face (y = 0.5) — normal: [0, 1, 0]
            ModelVertex {
                position: [-0.5, 0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            ModelVertex {
                position: [0.5, 0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 1.0, 0.0],
            },
            ModelVertex {
                position: [0.5, 0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            ModelVertex {
                position: [-0.5, 0.5, -0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, 1.0, 0.0],
            },
            // Bottom face (y = -0.5) — normal: [0, -1, 0]
            ModelVertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            ModelVertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            ModelVertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            ModelVertex {
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
        let vertices: &mut [ModelVertex] = &mut [
            // Front face (z = 0.5) — normal: [0, 0, 1]
            ModelVertex {
                position: [0.0, 0.5, 0.5],
                tex_coords: [0.5, 0.0],
                normal: [0.0, 0.0, 1.0],
            },
            ModelVertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            ModelVertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, 1.0],
            },
            // Back face (z = -0.5) — normal: [0, 0, -1]
            ModelVertex {
                position: [0.0, 0.5, -0.5],
                tex_coords: [0.5, 0.0],
                normal: [0.0, 0.0, -1.0],
            },
            ModelVertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            ModelVertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, 0.0, -1.0],
            },
            // Bottom face (y = -0.5) — normal: [0, -1, 0]
            ModelVertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            ModelVertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.0, -1.0, 0.0],
            },
            ModelVertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            ModelVertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [0.0, -1.0, 0.0],
            },
            // Left face (A-B edge) — normal: perpendicular to edge, pointing outward
            // Normal for left face: [-0.894, 0.447, 0] (normalized)
            ModelVertex {
                position: [0.0, 0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [-0.894, 0.447, 0.0],
            },
            ModelVertex {
                position: [0.0, 0.5, -0.5],
                tex_coords: [1.0, 1.0],
                normal: [-0.894, 0.447, 0.0],
            },
            ModelVertex {
                position: [-0.5, -0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [-0.894, 0.447, 0.0],
            },
            ModelVertex {
                position: [-0.5, -0.5, 0.5],
                tex_coords: [0.0, 0.0],
                normal: [-0.894, 0.447, 0.0],
            },
            // Right face (A-C edge) — normal: [0.894, 0.447, 0]
            ModelVertex {
                position: [0.0, 0.5, 0.5],
                tex_coords: [0.0, 1.0],
                normal: [0.894, 0.447, 0.0],
            },
            ModelVertex {
                position: [0.5, -0.5, 0.5],
                tex_coords: [1.0, 1.0],
                normal: [0.894, 0.447, 0.0],
            },
            ModelVertex {
                position: [0.5, -0.5, -0.5],
                tex_coords: [1.0, 0.0],
                normal: [0.894, 0.447, 0.0],
            },
            ModelVertex {
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

    // pub fn load_obj_model(
    //     &mut self,
    //     graphics: &mut InternalGraphics,
    //     location: &str,
    // ) -> anyhow::Result<(MeshComponent, MaterialComponent)> {
    //     let (models, material_res) = tobj::load_obj(
    //         location,
    //         &tobj::LoadOptions {
    //             single_index: true,
    //             triangulate: true,
    //             ..Default::default()
    //         },
    //     )?;
    //
    //     let materials = material_res.unwrap_or_default();
    //
    //     // let's do something different
    //     // let's add the asset to the world and have it spawn it...
    //     // but how would I handle transform
    //     // I would add it to it...
    //     // but
    //     println!("Mesh: {:#?}", models.len());
    //     println!("Materials: {:#?}", materials.len());
    //
    //     let mesh = &models[0].mesh;
    //     let vertices: Vec<ModelVertex> = (0..mesh.positions.len() / 3)
    //         .map(|i| {
    //             let normals = if !mesh.normals.is_empty() {
    //                 [
    //                     mesh.normals[i * 3],
    //                     mesh.normals[i * 3 + 1],
    //                     mesh.normals[i * 3 + 2],
    //                 ]
    //             } else {
    //                 [0.0, 0.0, 0.0]
    //             };
    //
    //             let tex_coords = if !mesh.texcoords.is_empty() {
    //                 [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]]
    //             } else {
    //                 [0.0, 0.0]
    //             };
    //
    //             ModelVertex {
    //                 position: [
    //                     mesh.positions[i * 3],
    //                     mesh.positions[i * 3 + 1],
    //                     mesh.positions[i * 3 + 2],
    //                 ],
    //                 tex_coords,
    //                 normal: normals,
    //             }
    //         })
    //         .collect();
    //     let indices = mesh.indices.clone();
    //     let mut mesh_data = MeshData { vertices, indices };
    //     let mesh_handle = self.mesh_manager.add_mesh_data(&mut mesh_data, graphics);
    //
    //     if !materials.is_empty() {
    //         let diffuse_texture = materials[0].clone().diffuse_texture;
    //         // this is incase the material is not empty but diffuse is
    //         if diffuse_texture.is_none() {
    //             let component = self.create_material(
    //                 graphics,
    //                 MaterialType::NonTexture {
    //                     color: [0.0, 0.3, 0.5],
    //                 },
    //             );
    //             return Ok((MeshComponent(mesh_handle), component));
    //         }
    //
    //         // #[cfg(target_os = "macOS")]
    //         let normalized_texture = diffuse_texture.unwrap().replace("\\", "/");
    //
    //         let obj_path = std::path::Path::new(location);
    //         let parent_dir = obj_path.parent().unwrap_or(std::path::Path::new(""));
    //
    //         let full_texture_path = parent_dir.join(normalized_texture);
    //         let diffuse_texture_path = full_texture_path.to_string_lossy().to_string();
    //
    //         println!("Diffuse: {:#?}", diffuse_texture_path);
    //
    //         let material_handle = self
    //             .material_manager
    //             .add_obj_material(graphics, &diffuse_texture_path)?;
    //
    //         return Ok((
    //             MeshComponent(mesh_handle),
    //             MaterialComponent(material_handle),
    //         ));
    //     } else {
    //         let component = self.create_material(
    //             graphics,
    //             MaterialType::NonTexture {
    //                 color: [0.0, 0.3, 0.5],
    //             },
    //         );
    //         return Ok((MeshComponent(mesh_handle), component));
    //     }
    // }
}
