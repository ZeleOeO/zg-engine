use std::sync::Arc;

use anyhow::Ok;
use wgpu::{CurrentSurfaceTexture, TextureViewDescriptor};
use winit::{
    application::ApplicationHandler,
    event::{
        DeviceEvent::{self, MouseMotion},
        KeyEvent, WindowEvent,
    },
    event_loop::EventLoop,
    keyboard::PhysicalKey,
    window::{CursorGrabMode, Window},
};

use crate::world::world::World;

pub mod camera;
pub mod graphics;
pub mod key_input;
pub mod layouts;
pub mod managers;
pub mod math;
pub mod pipeline;
pub mod render;
pub mod systems;
pub mod utils;
pub mod world;

pub struct NewApp {
    engine_app: Option<EngineApp>,
}

pub struct EngineApp {
    pub world: World,
    window: Arc<Window>,
}

impl EngineApp {
    pub async fn new(window: Arc<Window>) -> Self {
        window.set_cursor_visible(false);
        window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();
        Self {
            world: World::new(),
            window: window,
        }
    }
}

impl NewApp {
    pub fn new() -> anyhow::Result<NewApp> {
        env_logger::init();
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let mut app = NewApp { engine_app: None };
        event_loop.run_app(&mut app)?;

        Ok(app)
    }
}

impl ApplicationHandler for NewApp {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Graphics Engine"))
                .unwrap(),
        );
        let mut app = pollster::block_on(EngineApp::new(window.clone()));
        self.engine_app = Some(app);
        // app.world.insert_default_resources(window.clone());
        window.request_redraw();
    }
    fn device_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        let Some(app) = &mut self.engine_app else {
            return;
        };

        match event {
            MouseMotion { delta } => {
                app.world
                    .camera_controller
                    .handle_mouse(delta.0 as f32, delta.1 as f32);
            }
            _ => {}
        }
    }

    // fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
    //     let now = Instant::now();
    //     let Some(app) = &mut self.engine_app else {
    //         return;
    //     };
    //     let dt = now.duration_since(app.last_frame).as_secs_f32();
    //     app.last_frame = Instant::now();
    //     app.update(dt);
    //     app.window.request_redraw();
    // }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(app) = &mut self.engine_app else {
            return;
        };
        match event {
            WindowEvent::Resized(size) => {
                let Some(app) = &mut self.engine_app else {
                    return;
                };
                // app.camera.aspect = size.width as f32 / size.height as f32;
                // app.scene.depth_texture =
                // CustomTexture::create_depth_texture(&app.gpu.device, &app.gpu.config);
            }
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
            } => key_input::handle_key_system(
                code,
                key_state.is_pressed(),
                event_loop,
                &mut app.world.camera_controller,
            ),
            _ => {}
        }
    }
}

fn main() -> anyhow::Result<()> {
    let app = NewApp::new();
    Ok(())
}
