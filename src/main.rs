use std::sync::Arc;

use wgpu::{CurrentSurfaceTexture, TextureView, TextureViewDescriptor};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::PhysicalKey,
    window::Window,
};

use crate::{
    gpu::GPU,
    pipeline::opaque_pipeline,
    renderer::render,
    resources::{
        material::{Material, Uniform, create_material_bg_layout},
        mesh::Mesh,
    },
    scene::{DrawItem, Scene},
};

use std::rc::Rc;

pub mod gpu;
pub mod key_input;
pub mod pipeline;
pub mod renderer;
pub mod resources;
pub mod scene;

pub struct AppHandler {
    app: Option<App>,
}

pub struct App {
    scene: Scene,
    gpu: GPU,
    window: Arc<Window>,
}

impl App {
    pub async fn new(window: Arc<Window>) -> Self {
        let gpu = pollster::block_on(GPU::new(&window)).unwrap();

        let layout = create_material_bg_layout(&gpu.device);
        let texture_location = "./assets/happy-tree.png";
        let shader = &gpu
            .device
            .create_shader_module(wgpu::include_wgsl!("./shaders/shader.wgsl"));
        print!("stuff");
        let pipeline = Rc::new(opaque_pipeline(&gpu.device, &gpu.config, &layout, shader).unwrap());

        print!("stuff2");

        let mesh = Rc::new(Mesh::cube(&gpu.device));
        let material =
            Rc::new(Material::new(&gpu.device, &gpu.queue, &layout, texture_location).unwrap());

        let uniform = Uniform { rotation: 0.0 };
        let scene = Scene {
            draw_items: vec![DrawItem {
                pipeline: pipeline.clone(),
                mesh: mesh.clone(),
                material: material.clone(),
                uniform: uniform,
            }],
        };
        Self {
            scene,
            gpu: gpu,
            window: window,
        }
    }

    pub fn render(&self, view: &TextureView) {
        render(&self.gpu.device, &self.gpu.queue, view, &self.scene);
    }

    pub fn update(&mut self) {
        for item in &mut self.scene.draw_items {
            item.rotate_item(&self.gpu.queue, 1.0);
        }
    }
}

impl ApplicationHandler for AppHandler {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Graphics Engine"))
                .unwrap(),
        );
        let app = pollster::block_on(App::new(window.clone()));
        self.app = Some(app);
        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(app) = &mut self.app else {
            return;
        };
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                app.window.request_redraw();

                let frame = match app.gpu.surface.get_current_texture() {
                    CurrentSurfaceTexture::Success(texture)
                    | CurrentSurfaceTexture::Suboptimal(texture) => texture,
                    CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return,
                    CurrentSurfaceTexture::Outdated | CurrentSurfaceTexture::Lost => {
                        app.gpu.surface.configure(&app.gpu.device, &app.gpu.config);
                        return;
                    }
                    CurrentSurfaceTexture::Validation => return,
                };

                let view = frame.texture.create_view(&TextureViewDescriptor::default());

                app.render(&view);

                frame.present();
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => key_input::handle_key(code, key_state.is_pressed(), event_loop),
            _ => {}
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    let mut app = AppHandler { app: None };
    event_loop.run_app(&mut app)?;
    Ok(())
}
