use std::any::TypeId;

use crate::{
    managers::Manager,
    render::render::{RenderCommand, RenderQueue},
    world::{
        components::{MaterialComponent, MeshComponent, TransformComponent},
        world::World,
    },
};

pub fn draw_items(world: &mut World) {
    let components = vec![
        TypeId::of::<MeshComponent>(),
        TypeId::of::<MaterialComponent>(),
        TypeId::of::<TransformComponent>(),
    ];

    let render_queue = &mut world.get_mut::<RenderQueue>();

    let archetype_id = &world.get_archetype_by_type_ids(components).unwrap();
    let archetype = &world.archetypes[archetype_id.archetype_id.0 as usize];

    let mesh_column: &Vec<MeshComponent> =
        archetype.get_column_by_type_id(TypeId::of::<MeshComponent>());
    let material_column: &Vec<MaterialComponent> =
        archetype.get_column_by_type_id(TypeId::of::<MaterialComponent>());

    let transform_column: &Vec<TransformComponent> =
        archetype.get_column_by_type_id(TypeId::of::<TransformComponent>());

    let manager = &world.get::<Manager>();
    let item_iter = mesh_column
        .iter()
        .zip(material_column.iter().zip(transform_column.iter()));

    for (mesh, (material, transform)) in item_iter {
        render_queue.commands.push(RenderCommand::SetVertexBuffer {
            mesh_handle: mesh.0,
        });
        render_queue.commands.push(RenderCommand::SetBindGroup {
            slot: 0,
            bind_group: manager.material_manager.get_material(material.0).clone(),
        });
    }
}
