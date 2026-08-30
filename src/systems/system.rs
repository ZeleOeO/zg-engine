use std::any::TypeId;

use crate::{
    graphics::gpu::InternalGraphics,
    managers::Manager,
    pipeline::pipeline_id::PipelineID,
    render::{command::RenderCommand, render_queue::RenderQueue},
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

    let mut render_queue = world.get_mut::<RenderQueue>();
    let manager = &world.get::<Manager>();
    let mut gpu = world.get_mut::<InternalGraphics>();

    let archetype_id = &world.get_archetype_by_type_ids(components).unwrap();
    let archetype = &world.archetypes[archetype_id.archetype_id.0 as usize];

    let mesh_column: &Vec<MeshComponent> =
        archetype.get_column_by_type_id(TypeId::of::<MeshComponent>());
    let material_column: &Vec<MaterialComponent> =
        archetype.get_column_by_type_id(TypeId::of::<MaterialComponent>());

    let transform_column: &Vec<TransformComponent> =
        archetype.get_column_by_type_id(TypeId::of::<TransformComponent>());

    // TODO change this to use new querying system

    let item_iter = mesh_column
        .iter()
        .zip(material_column.iter().zip(transform_column.iter()));

    for (mesh, (material, transform)) in item_iter {
        let material_bind_group_handle = manager.material_manager.get_material(material.0);
        let transform_bind_group_handle = transform.0.get_or_create_bind_group(gpu.as_mut());
        let mesh_meta_data = manager.mesh_manager.get_mesh_data(mesh.0.0);

        render_queue.commands.push(RenderCommand::SetPipeline {
            pipeline_id: PipelineID::MAIN,
        });

        render_queue.commands.push(RenderCommand::SetVertexBuffer {
            mesh_handle: mesh.0,
        });
        render_queue.commands.push(RenderCommand::SetBindGroup {
            bind_group_handle: material_bind_group_handle,
        });
        render_queue.commands.push(RenderCommand::SetBindGroup {
            bind_group_handle: transform_bind_group_handle,
        });
        render_queue.commands.push(RenderCommand::DrawIndexed {
            num_to_draw: mesh_meta_data.index_count,
        });
    }
}
