use tracy_client::span;
use wgpu::{CurrentSurfaceTexture, TextureView};
use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

use zg_managers::{PointLight, create_light_bind_group};
use zg_utils::time::Time;

use crate::render_queue::RenderQueue;
use crate::{render_command::RenderCommand, render_utils::create_transform_bind_group};
use zg_graphics::*;
use zg_managers::Assets;
use zg_systems::SystemAggregator;
use zg_world::{ResourceMut, World, components::*};

pub fn render_items_system(world: &mut World, _dt: f32) {
    let span = span!("render items");
    span.emit_color(0x3A9453);

    let mut render_queue = world.get_mut::<RenderQueue>();
    let assets = &world.get::<Assets>();

    let mut gpu = world.get_mut::<InternalGraphics>();

    let archetype_id = world.get_archetype_ids::<(MeshComponent, MaterialComponent, Transform)>();
    let archetype = world.get_archetypes_by_id(archetype_id.as_slice());

    let item = world
        .get_all_entities_in_archetypes::<(MeshComponent, MaterialComponent, Transform)>(
            &archetype,
        );

    for (mesh, material, transform) in item {
        let material_bind_group_handle = assets.material_manager.get_material(material.0);
        let transform_bind_group_handle = create_transform_bind_group(&transform, gpu.as_mut());
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

pub fn render_lights_system(world: &mut World, _dt: f32) {
    let _span = span!("render light");
    let mut render_queue = world.get_mut::<RenderQueue>();
    let mut gpu = world.get_mut::<InternalGraphics>();

    let archetype_ids = world.get_archetype_ids::<(PointLight,)>();
    let archetypes = world.get_archetypes_by_id(archetype_ids.as_slice());
    let items = world.get_all_entities_in_archetypes::<(PointLight,)>(&archetypes);

    for (light,) in items {
        let light_bind_group_handle = create_light_bind_group(&mut gpu, light);
        render_queue.commands.push(RenderCommand::SetPipeline {
            pipeline_id: PipelineID::MAIN,
        });

        render_queue.commands.push(RenderCommand::SetBindGroup {
            bind_group_handle: light_bind_group_handle,
        });
    }
}

pub fn execute_frame(
    graphics: &mut InternalGraphics,
    world: &mut World,
    surface_view: &TextureView,
) {
    let _ = span!("execute frame");
    let mut encoder = {
        let _ = span!("create encoder");
        graphics
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Encoder"),
            })
    };
    let mut render_queue = world.get_mut::<RenderQueue>();
    {
        let _ = span!("main render pass");
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
    {
        let span = span!("submit");
        span.emit_color(0xE346B4);
        graphics.queue.submit(Some(encoder.finish()));
    }
}

pub fn graphics_window_event_system(
    world: &mut World,
    event: &WindowEvent,
    _event_loop: &ActiveEventLoop,
) {
    match event {
        WindowEvent::RedrawRequested => {
            let _ = span!("redraw request");
            let mut time = world.get_mut::<Time>();
            time.update();
            let graphics = world.get::<InternalGraphics>();
            let frame = {
                let _ = span!("retrieve current texture");
                match graphics.surface.get_current_texture() {
                    CurrentSurfaceTexture::Success(texture)
                    | CurrentSurfaceTexture::Suboptimal(texture) => texture,
                    CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return,
                    CurrentSurfaceTexture::Outdated | CurrentSurfaceTexture::Lost => {
                        println!("Error");
                        graphics
                            .surface
                            .configure(&graphics.device, &graphics.config);
                        return;
                    }
                    CurrentSurfaceTexture::Validation => return,
                }
            };

            drop(graphics);
            drop(time);

            let view = {
                let _ = span!("create view");
                frame
                    .texture
                    .create_view(&wgpu::TextureViewDescriptor::default())
            };
            world.resource_scope(|world, mut gpu: ResourceMut<InternalGraphics>| {
                execute_frame(&mut gpu, world, &view);
            });

            {
                let _ = span!("present frame");
                frame.present();
                tracy_client::frame_mark();
                // println!("PRESENT");
            }
        }
        _ => {}
    }
}

pub fn system(system: &mut SystemAggregator) {
    system.insert_update_system(render_lights_system);
    system.insert_update_system(render_items_system);
    system.insert_window_event_sytem(graphics_window_event_system);
}
