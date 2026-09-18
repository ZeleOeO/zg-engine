use std::any::TypeId;

use wgpu::{CurrentSurfaceTexture, TextureView};
use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

use crate::render_queue::RenderQueue;
use crate::{render_command::RenderCommand, render_utils::create_transform_bind_group};
use zg_graphics::*;
use zg_managers::Assets;
use zg_systems::SystemAggregator;
use zg_world::{ResourceMut, World, components::*};

pub fn render_items_system(world: &mut World) {
    let components = vec![
        TypeId::of::<MeshComponent>(),
        TypeId::of::<MaterialComponent>(),
        TypeId::of::<Transform>(),
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
        let transform_bind_group_handle = create_transform_bind_group(&transform.0, gpu.as_mut());
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

pub fn execute_frame(
    graphics: &mut InternalGraphics,
    world: &mut World,
    surface_view: &TextureView,
) {
    let mut encoder = graphics
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Encoder"),
        });
    let mut render_queue = world.get_mut::<RenderQueue>();
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: surface_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &graphics.depth_texture_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        render_queue.flush(&mut render_pass, world, graphics);
    }
    graphics.queue.submit(Some(encoder.finish()));
}

pub fn graphics_window_event_system(
    world: &mut World,
    event: &WindowEvent,
    _event_loop: &ActiveEventLoop,
) {
    match event {
        WindowEvent::RedrawRequested => {
            let graphics = world.get::<InternalGraphics>();
            let frame = match graphics.surface.get_current_texture() {
                CurrentSurfaceTexture::Success(texture)
                | CurrentSurfaceTexture::Suboptimal(texture) => texture,
                CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return,
                CurrentSurfaceTexture::Outdated | CurrentSurfaceTexture::Lost => {
                    graphics
                        .surface
                        .configure(&graphics.device, &graphics.config);
                    return;
                }
                CurrentSurfaceTexture::Validation => return,
            };

            drop(graphics);
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            world.resource_scope(|world, mut gpu: ResourceMut<InternalGraphics>| {
                execute_frame(&mut gpu, world, &view);
            });
            frame.present();
        }
        _ => {}
    }
}

pub fn system(system: &mut SystemAggregator) {
    system.insert_update_system(render_items_system);
    system.insert_window_event_sytem(graphics_window_event_system);
}
