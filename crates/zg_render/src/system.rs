use tracy_client::span;
use wgpu::{CurrentSurfaceTexture, TextureView};
use winit::event::WindowEvent;

use zg_managers::{PointLight, create_light_bind_group, get_or_create_default_light_material};
use zg_time::Time;

use crate::RenderQueue;
use crate::render_command::{DrawItem, FrameBinding};
use crate::render_utils::create_transform_bind_group;
use zg_graphics::*;
use zg_managers::Assets;
use zg_world::{Query, Res, ResMut, components::*};

pub fn render_items_system(
    query: Query<(MeshComponent, MaterialComponent, Transform)>,
    mut render_queue: ResMut<RenderQueue>,
    assets: Res<Assets>,
    mut graphics: ResMut<InternalGraphics>,
) {
    let span = span!("render items");
    span.emit_color(0x3A9453);

    for (mesh, material, transform) in query.get_all_entities() {
        let mesh_meta_data = assets.mesh_manager.get_mesh_data(mesh.0.0);
        render_queue.draw_items.push(DrawItem {
            layer: crate::RenderLayer::Opaque,
            pipeline: PipelineID::MAIN,
            material: assets.material_manager.get_material(material.0),
            transform: create_transform_bind_group(&transform, &mut graphics.0),
            mesh: mesh.0,
            index_count: mesh_meta_data.index_count,
        });
    }
}

pub fn render_lights_system(
    mut render_queue: ResMut<RenderQueue>,
    mut graphics: ResMut<InternalGraphics>,
    query: Query<(PointLight,)>,
) {
    let _span = span!("render light");
    for (light,) in query.get_all_entities() {
        render_queue.frame_binding.push(FrameBinding {
            bind_group: create_light_bind_group(&mut graphics.0, light),
        });
    }
}

pub fn render_light_gizmo_system(
    mut render_queue: ResMut<RenderQueue>,
    mut graphics: ResMut<InternalGraphics>,
    assets: Res<Assets>,
    query: Query<(PointLight, MeshComponent)>,
) {
    let _span = span!("render light gizmos");

    for (light, mesh) in query.get_all_entities() {
        let mesh_meta_data = assets.mesh_manager.get_mesh_data(mesh.0.0);
        let transform =
            Transform::from_translation(light.position[0], light.position[1], light.position[2]);
        render_queue.draw_items.push(DrawItem {
            layer: crate::RenderLayer::Overlay,
            pipeline: PipelineID::LIGHT,
            material: get_or_create_default_light_material(&mut graphics),
            transform: create_transform_bind_group(&transform, &mut graphics),
            mesh: mesh.0,
            index_count: mesh_meta_data.index_count,
        });
    }
}

pub fn execute_frame(
    graphics: &mut InternalGraphics,
    render_queue: &mut RenderQueue,
    assets: &Assets,
    surface_view: &TextureView,
) {
    span!("execute frame");
    let mut encoder = {
        span!("create encoder");
        graphics
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Encoder"),
            })
    };
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

// NOTE: run this un updates
pub fn graphics_render_system(
    mut time: ResMut<Time>,
    mut graphics: ResMut<InternalGraphics>,
    mut render_queue: ResMut<RenderQueue>,
    assets: Res<Assets>,
) {
    span!("redraw request");
    time.update();
    let frame = {
        span!("retrieve current texture");
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

    let surface_view = {
        span!("create view");
        frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default())
    };

    execute_frame(&mut graphics, &mut render_queue, &assets, &surface_view);

    {
        let _ = span!("present frame");
        frame.present();
        tracy_client::frame_mark();
    }
}

// pub fn graphics_draw_system(
//     event: EventRef,
//     mut time: ResMut<Time>,
//     mut graphics: ResMut<InternalGraphics>,
//     mut render_queue: ResMut<RenderQueue>,
//     assets: Res<Assets>,
// ) {
//     if let EngineEvents::Window(WindowEvent::RedrawRequested) = event.0 {}
// }
