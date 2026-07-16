use std::sync::Arc;

use wgpu::{CurrentSurfaceTexture, TextureViewDescriptor};
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    keyboard::PhysicalKey,
    window::Window,
};

use crate::renderer::{AppGraphicsEngine, context::RendererContext};

pub mod renderer;

pub struct App {
    engine: Option<AppGraphicsEngine>,
    renderer: Option<RendererContext>,
    window: Arc<Window>,
}

impl App {
    pub fn new(window: Arc<Window>) -> Self {
        Self {
            engine: None,
            renderer: None,
            window: window,
        }
    }

    fn run() -> anyhow::Result<()> {
        let event_loop = EventLoop::with_user_event().build()?;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Graphics Engine"))
                .unwrap(),
        );
        let mut app = App::new(window);
        event_loop.run_app(&mut app)?;

        Ok(())
    }
}

impl ApplicationHandler<AppGraphicsEngine> for App {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.renderer =
            Some(pollster::block_on(RendererContext::new(self.window.clone())).unwrap());
        self.engine = Some(
            AppGraphicsEngine::new(
                &self.renderer.as_ref().unwrap().queue,
                &self.renderer.as_ref().unwrap().device,
                &self.renderer.as_ref().unwrap().config,
            )
            .unwrap(),
        );
        self.window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let state = match &mut self.engine {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.window.request_redraw();
                let engine = self.engine.as_mut().unwrap();
                let app_window = self.renderer.as_ref().unwrap();

                let frame = match app_window.surface.get_current_texture() {
                    CurrentSurfaceTexture::Success(texture)
                    | CurrentSurfaceTexture::Suboptimal(texture) => texture,
                    CurrentSurfaceTexture::Timeout | CurrentSurfaceTexture::Occluded => return,
                    CurrentSurfaceTexture::Outdated | CurrentSurfaceTexture::Lost => {
                        app_window
                            .surface
                            .configure(&app_window.device, &app_window.config);
                        return;
                    }
                    CurrentSurfaceTexture::Validation => return,
                };

                let view = frame.texture.create_view(&TextureViewDescriptor::default());

                engine.render(&app_window.queue, &app_window.device, &view);
                frame.present();
                engine.update(&app_window.queue);
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => state.handle_key(code, key_state.is_pressed(), event_loop),
            _ => {}
        }
    }
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    App::run()?;
    Ok(())
}
