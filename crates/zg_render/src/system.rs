use tracy_client::span;
use wgpu::{CurrentSurfaceTexture, TextureView};

use zg_managers::{PointLight, create_light_bind_group, get_or_create_default_light_material};
use zg_time::Time;
use zg_ui::{EditorState, EngineUI, UiFrame, create_viewport_target};

use crate::render_command::{DrawItem, FrameBinding};
use crate::render_utils::{CameraUniform, create_transform_bind_group};
use crate::renderer::SceneTargetSize;
use crate::{RenderQueue, WorldRenderer};
use zg_graphics::*;
use zg_managers::Assets;
use zg_world::{Query, Res, ResMut, components::*};

pub fn setup_world_renderer(graphics: Res<InternalGraphics>, mut renderer: ResMut<WorldRenderer>) {
    let buffer = graphics.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: std::mem::size_of::<CameraUniform>() as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        mapped_at_creation: false,
    });
    renderer.camera_buffer = Some(buffer);
}

pub fn render_items_system(
    query: Query<(MeshComponent, MaterialComponent, Transform)>,
    mut render_queue: ResMut<RenderQueue>,
    assets: Res<Assets>,
    mut graphics: ResMut<InternalGraphics>,
) {
    let span = span!("render items");
    span.emit_color(0x3A9453);

    graphics.cache.transform_bind_groups_index = 0;

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
    graphics.cache.light_bind_groups_index = 0;

    let entities = query.get_all_entities();
    if entities.is_empty() {
        let default_light = PointLight::new([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        render_queue.frame_binding.push(FrameBinding {
            bind_group: create_light_bind_group(&mut graphics.0, &default_light),
        });
    } else {
        for (light,) in entities {
            render_queue.frame_binding.push(FrameBinding {
                bind_group: create_light_bind_group(&mut graphics.0, light),
            });
        }
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
    depth_view: &TextureView,
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
                view: depth_view,
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

pub fn graphics_render_system(
    mut time: ResMut<Time>,
    mut graphics: ResMut<InternalGraphics>,
    mut render_queue: ResMut<RenderQueue>,
    assets: Res<Assets>,
    mut engine_ui: ResMut<EngineUI>,
    ui_frame: Res<UiFrame>,
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

    let swap_view = {
        span!("create view");
        frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default())
    };

    execute_frame(
        &mut graphics,
        &mut render_queue,
        &assets,
        &engine_ui.viewport_target.view,
        &engine_ui.viewport_target.depth_view,
    );

    let mut encoder = graphics.device.create_command_encoder(&Default::default());

    for (id, deltas) in &ui_frame.textures_delta.set {
        for delta in deltas {
            engine_ui
                .renderer
                .update_texture(&graphics.device, &graphics.queue, *id, delta);
        }
    }

    let screen = egui_wgpu::ScreenDescriptor {
        size_in_pixels: [graphics.config.width, graphics.config.height],
        pixels_per_point: ui_frame.pixels_per_point,
    };
    let extra = engine_ui.renderer.update_buffers(
        &graphics.device,
        &graphics.queue,
        &mut encoder,
        &ui_frame.triangle_mesh,
        &screen,
    );

    {
        let rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("egui pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &swap_view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        engine_ui.renderer.render(
            &mut rpass.forget_lifetime(),
            &ui_frame.triangle_mesh,
            &screen,
        );
    }

    for id in &ui_frame.textures_delta.free {
        engine_ui.renderer.free_texture(id);
    }

    graphics
        .queue
        .submit(extra.into_iter().chain([encoder.finish()]));

    {
        let _span = span!("present frame");
        graphics.queue.present(frame);
        tracy_client::frame_mark();
    }
}

pub fn viewport_resize_system(
    graphics: Res<InternalGraphics>,
    editor: Res<EditorState>,
    mut engine_ui: ResMut<EngineUI>,
    mut scene_size: ResMut<SceneTargetSize>,
) {
    let wanted = editor.viewport_wanted_px;
    if wanted[0] == 0 || wanted[1] == 0 {
        return;
    }

    let ui = &mut *engine_ui;

    if wanted != ui.viewport_target.size_ppx {
        ui.viewport_target = create_viewport_target(&graphics, wanted);
        ui.renderer.update_egui_texture_from_wgpu_texture(
            &graphics.device,
            &ui.viewport_target.view,
            wgpu::FilterMode::Linear,
            editor.viewport_tex_id,
        );
    }

    scene_size.0.0 = engine_ui.viewport_target.size_ppx;
}
