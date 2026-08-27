use std::{sync::Arc, time::Instant};

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
    window::Window,
};

use crate::{app::engine_app::EngineApp, world::world::World};

pub struct App {
    engine_app: Option<EngineApp>,
}

impl App {
    pub fn new() -> anyhow::Result<App> {
        env_logger::init();
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        let mut app = App { engine_app: None };
        event_loop.run_app(&mut app)?;

        Ok(app)
    }

    pub fn add_setup_system<F: FnMut(&mut World) + 'static>(mut self, callback: F) -> Self {
        self.engine_app
            .as_mut()
            .unwrap()
            .systems
            .add_setup_system(callback);
        self
    }

    pub fn add_update_system<F: FnMut(&mut World) + 'static>(&mut self, callback: F) -> &mut Self {
        self.engine_app
            .as_mut()
            .unwrap()
            .systems
            .add_update_system(callback);
        self
    }

    pub fn add_window_event_sytem<F: FnMut(&mut World, &WindowEvent) + 'static>(
        &mut self,
        callback: F,
    ) -> &mut Self {
        self.engine_app
            .as_mut()
            .unwrap()
            .systems
            .add_window_event_sytem(callback);
        self
    }

    pub fn add_device_event_sytem<F: FnMut(&mut World, &DeviceEvent) + 'static>(
        &mut self,
        callback: F,
    ) -> &mut Self {
        self.engine_app
            .as_mut()
            .unwrap()
            .systems
            .add_device_event_sytem(callback);
        self
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Graphics Engine"))
                .unwrap(),
        );
        let mut app = pollster::block_on(EngineApp::new(window.clone()));
        app.world.insert_default_resources(window.clone());
        self.engine_app = Some(app);
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
        let world = &mut app.world;

        app.systems.device_events.execute((world, &event));
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let now = Instant::now();
        let Some(app) = &mut self.engine_app else {
            return;
        };
        // // let dt = now.duration_since(app.last_frame).as_secs_f32();
        // // app.last_frame = Instant::now();
        // app.update(dt);
        // app.window.request_redraw();
    }

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
            _ => {}
        }
    }
}
