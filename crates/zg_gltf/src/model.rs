use zg_graphics::InternalGraphics;
use zg_managers::{Assets, material::MaterialType, mesh::MeshData};
use zg_utils::ModelVertex;
use zg_world::{
    Entity, World,
    components::{MaterialComponent, MeshComponent, TransformComponent},
};

pub struct Model;

impl Model {
    pub fn load_obj(world: &mut World, location: &str) -> anyhow::Result<Vec<Entity>> {
        let components_to_spawn = {
            let mut graphics = world.get_mut::<InternalGraphics>();
            let mut assets = world.get_mut::<Assets>();

            let (models, material_res) = tobj::load_obj(
                location,
                &tobj::LoadOptions {
                    single_index: true,
                    triangulate: true,
                    ..Default::default()
                },
            )?;

            let materials = material_res.unwrap_or_default();
            let mut extracted_components = Vec::with_capacity(models.len());

            for model in models {
                let mesh = &model.mesh;
                let vertices: Vec<ModelVertex> = (0..mesh.positions.len() / 3)
                    .map(|i| {
                        let normals = if !mesh.normals.is_empty() {
                            [
                                mesh.normals[i * 3],
                                mesh.normals[i * 3 + 1],
                                mesh.normals[i * 3 + 2],
                            ]
                        } else {
                            [0.0, 0.0, 0.0]
                        };

                        let tex_coords = if !mesh.texcoords.is_empty() {
                            [mesh.texcoords[i * 2], mesh.texcoords[i * 2 + 1]]
                        } else {
                            [0.0, 0.0]
                        };

                        ModelVertex {
                            position: [
                                mesh.positions[i * 3],
                                mesh.positions[i * 3 + 1],
                                mesh.positions[i * 3 + 2],
                            ],
                            tex_coords,
                            normal: normals,
                        }
                    })
                    .collect();

                let indices = mesh.indices.clone();
                let mut mesh_data = MeshData { vertices, indices };
                let mesh_handle = assets.mesh_manager.add_mesh_data(&mut mesh_data, &graphics);
                let mesh_component = MeshComponent(mesh_handle);
                println!("Material: {:#?}", mesh.material_id);

                let material_component = if let Some(mat_id) = mesh.material_id {
                    let diffuse_texture = materials[mat_id].clone().diffuse_texture;
                    if diffuse_texture.is_none() {
                        assets.create_material(
                            &mut graphics,
                            MaterialType::NonTexture {
                                color: [0.0, 0.3, 0.5],
                            },
                        )
                    } else {
                        let normalized_texture = diffuse_texture.unwrap().replace("\\", "/");
                        let obj_path = std::path::Path::new(location);
                        let parent_dir = obj_path.parent().unwrap_or(std::path::Path::new(""));
                        let full_texture_path = parent_dir.join(normalized_texture);
                        let diffuse_texture_path = full_texture_path.to_string_lossy().to_string();

                        println!("Diffuse: {:#?}", diffuse_texture_path);
                        let material_handle = assets
                            .material_manager
                            .add_obj_material(&mut graphics, &diffuse_texture_path)?;
                        MaterialComponent(material_handle)
                    }
                } else {
                    println!("No diffuse texture");
                    assets.create_material(
                        &mut graphics,
                        MaterialType::NonTexture {
                            color: [0.0, 0.3, 0.5],
                        },
                    )
                };

                extracted_components.push((mesh_component, material_component));
            }

            Ok::<_, anyhow::Error>(extracted_components)
        }?;

        let mut spawned_entities = Vec::new();
        for (mesh, material) in components_to_spawn {
            let entity = world.spawn((mesh, material, TransformComponent::default()));
            spawned_entities.push(entity);
        }

        Ok(spawned_entities)
    }
}
