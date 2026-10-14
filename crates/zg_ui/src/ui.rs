use std::{fmt::Debug, sync::Arc};

use egui::{Context, TextureId};
use egui_wgpu::Renderer;
use egui_winit::State;
use wgpu::{Extent3d, TextureDescriptor};
use winit::window::Window;
use zg_graphics::InternalGraphics;
use zg_world::Resource;

#[derive(Resource)]
pub struct EngineUI {
    pub context: Context,
    pub renderer: Renderer,
    pub(crate) state: State,
    pub(crate) target_viewport_tex_id: TextureId,
    pub viewport_target: ViewportTarget,
}

impl Debug for EngineUI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EngineUiRenderer").finish()
    }
}

impl EngineUI {
    pub fn new(graphics: &InternalGraphics, window: Arc<Window>) -> Self {
        let context = Context::default();
        let mut renderer = Renderer::new(
            &graphics.device,
            graphics.surface_format,
            egui_wgpu::RendererOptions::default(),
        );
        let state = State::new(
            context.clone(),
            context.viewport_id(),
            &window.clone(),
            Some(window.scale_factor() as f32),
            window.theme(),
            None,
        );

        let viewport_target = create_viewport_target(graphics, [1280, 720]);
        let target_viewport_tex_id = renderer.register_native_texture(
            &graphics.device,
            &viewport_target.view,
            wgpu::FilterMode::Linear,
        );

        Self {
            context,
            renderer,
            state,
            target_viewport_tex_id,
            viewport_target,
        }
    }
}

pub struct ViewportTarget {
    pub view: wgpu::TextureView,
    // texture: wgpu::Texture,
    pub depth_view: wgpu::TextureView,
    pub size_ppx: [u32; 2],
}

pub fn create_viewport_target(graphics: &InternalGraphics, size: [u32; 2]) -> ViewportTarget {
    let size = Extent3d {
        width: size[0].max(1),
        height: size[1].max(1),
        depth_or_array_layers: 1,
    };

    let texture = graphics.device.create_texture(&TextureDescriptor {
        label: Some("Viewport Texture"),
        size,
        mip_level_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: graphics.surface_format,
        sample_count: 1,
        view_formats: &[],
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
    });

    let view = texture.create_view(&Default::default());

    let depth_texture = graphics.device.create_texture(&TextureDescriptor {
        label: Some("Viewport Texture Depth"),
        size,
        mip_level_count: 1,
        sample_count: 1,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Depth32Float,
        view_formats: &[],
    });

    let depth_view = depth_texture.create_view(&Default::default());

    ViewportTarget {
        view,
        // texture,
        depth_view,
        size_ppx: [size.width, size.height],
    }
}
