use std::sync::Arc;

use anyhow::Ok;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window,
};

use crate::{
    app::engine_app::EngineApp,
    camera::system::{
        camera_controller_device_sytem, camera_controller_update_system, camera_init_system,
        camera_update_system, camera_window_event,
    },
    graphics::system::graphics_window_event_system,
    render::system::render_items_system,
    world::world::World,
};

pub struct App {
    engine_app: Option<EngineApp>,
}

impl App {
    pub fn new() -> anyhow::Result<App> {
        env_logger::init();

        let engine_app = pollster::block_on(EngineApp::new());
        let app = App {
            engine_app: Some(engine_app),
        };

        Ok(app)
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        self.insert_default_systems();
        event_loop.run_app(self)?;
        Ok(())
    }

    pub fn add_setup_system<F: FnMut(&mut World) + 'static>(&mut self, callback: F) -> &mut Self {
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

    pub fn add_window_event_sytem<
        F: FnMut(&mut World, &WindowEvent, &ActiveEventLoop) + 'static,
    >(
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

    pub fn insert_default_systems(&mut self) {
        self.add_setup_system(camera_init_system);
        self.add_update_system(camera_update_system);
        self.add_update_system(render_items_system);
        self.add_update_system(camera_controller_update_system);
        self.add_window_event_sytem(camera_window_event);
        self.add_window_event_sytem(graphics_window_event_system);
        self.add_device_event_sytem(camera_controller_device_sytem);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_title("Graphics Engine"))
                .unwrap(),
        );

        let Some(app) = &mut self.engine_app else {
            return;
        };

        app.add_window(window.clone());
        let world = &mut app.world;
        world.insert_default_resources(window.clone());
        app.systems.setups.execute(world);
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
        let Some(app) = &mut self.engine_app else {
            return;
        };
        let world = &mut app.world;
        app.systems.updates.execute(world);
        let Some(window) = &mut app.window else {
            return;
        };
        window.request_redraw();
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

        let world = &mut app.world;
        app.systems
            .window_events
            .execute((world, &event, event_loop));
    }
}
