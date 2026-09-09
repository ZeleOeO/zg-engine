use std::any::TypeId;

use crate::{
    graphics::gpu::InternalGraphics,
    managers::Assets,
    pipeline::pipeline_id::PipelineID,
    render::{command::RenderCommand, render_queue::RenderQueue},
    systems::system_struct::SystemAggregator,
    world::{
        components::{MaterialComponent, MeshComponent, TransformComponent},
        world::World,
    },
};

pub fn render_items_system(world: &mut World) {
    let components = vec![
        TypeId::of::<MeshComponent>(),
        TypeId::of::<MaterialComponent>(),
        TypeId::of::<TransformComponent>(),
    ];

    let mut render_queue = world.get_mut::<RenderQueue>();
    let assets = &world.get::<Assets>();

    let mut gpu = world.get_mut::<InternalGraphics>();

    let archetype_id = &world.get_archetype_by_type_ids(components).unwrap();
    let archetype = &world.archetypes[archetype_id.archetype_id.0 as usize];

    let item = world
        .get_all_entities_in_archetype::<(MeshComponent, MaterialComponent, TransformComponent)>(
            archetype,
        );

    for (mesh, material, transform) in item {
        let material_bind_group_handle = assets.material_manager.get_material(material.0);
        let transform_bind_group_handle = transform.0.get_or_create_bind_group(gpu.as_mut());
        let mesh_meta_data = assets.mesh_manager.get_mesh_data(mesh.0.0);

        render_queue.commands.push(RenderCommand::SetPipeline {
            pipeline_id: PipelineID::MAIN,
        });

        render_queue.commands.push(RenderCommand::SetVertexBuffer {
            mesh_handle: mesh.0,
        });
        render_queue.commands.push(RenderCommand::SetIndexBuffer {
            index_handle: mesh.0,
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

pub fn system(system: &mut SystemAggregator) {
    system.insert_update_system(render_items_system);
}
