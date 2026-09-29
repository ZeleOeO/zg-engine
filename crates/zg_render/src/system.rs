use tracy_client::span;
use wgpu::{CurrentSurfaceTexture, TextureView};
use winit::{event::WindowEvent, event_loop::ActiveEventLoop};

use zg_managers::{PointLight, create_light_bind_group, get_or_create_default_light_material};
use zg_utils::time::Time;

use crate::RenderQueue;
use crate::render_command::{DrawItem, FrameBinding};
use crate::render_utils::create_transform_bind_group;
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
        let mesh_meta_data = assets.mesh_manager.get_mesh_data(mesh.0.0);
        render_queue.draw_items.push(DrawItem {
            layer: crate::render_command::RenderLayer::Opaque,
            pipeline: PipelineID::MAIN,
            material: assets.material_manager.get_material(material.0),
            transform: create_transform_bind_group(&transform, gpu.as_mut()),
            mesh: mesh.0,
            index_count: mesh_meta_data.index_count,
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
        render_queue.frame_binding.push(FrameBinding {
            bind_group: create_light_bind_group(&mut gpu, light),
        });
    }
}

pub fn render_light_gizmo_system(world: &mut World, _dt: f32) {
    let _span = span!("render light gizmos");
    let mut render_queue = world.get_mut::<RenderQueue>();
    let mut gpu = world.get_mut::<InternalGraphics>();
    let assets = world.get_mut::<Assets>();

    let archetype_ids = world.get_archetype_ids::<(PointLight, MeshComponent)>();
    let archetypes = world.get_archetypes_by_id(archetype_ids.as_slice());
    let items = world.get_all_entities_in_archetypes::<(PointLight, MeshComponent)>(&archetypes);

    for (light, mesh) in items {
        let mesh_meta_data = assets.mesh_manager.get_mesh_data(mesh.0.0);
        let transform =
            Transform::from_translation(light.position[0], light.position[1], light.position[2]);
        render_queue.draw_items.push(DrawItem {
            layer: crate::RenderLayer::Overlay,
            pipeline: PipelineID::LIGHT,
            material: get_or_create_default_light_material(&mut gpu),
            transform: create_transform_bind_group(&transform, &mut gpu),
            mesh: mesh.0,
            index_count: mesh_meta_data.index_count,
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
    let assets = world.get::<Assets>();
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
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
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

        render_queue.flush(&mut render_pass, &assets, graphics);
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
    system.insert_update_system(render_light_gizmo_system);
    system.insert_window_event_sytem(graphics_window_event_system);
}
