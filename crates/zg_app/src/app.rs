use std::sync::Arc;

use anyhow::Ok;
use winit::{
    application::ApplicationHandler, event::DeviceEvent, event_loop::EventLoop, window::Window,
};
use zg_utils::time::Time;

use crate::engine_app::EngineApp;
use zg_camera::system as camera_system;
use zg_render::system as render_system;
use zg_systems::SystemAggregator;

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

    pub fn insert_system<F: FnOnce(&mut SystemAggregator)>(mut self, callback: F) -> Self {
        self.engine_app
            .as_mut()
            .unwrap()
            .systems
            .add_system(callback);
        self
    }

    pub fn insert_default_systems(&mut self) {
        self.engine_app
            .as_mut()
            .unwrap()
            .systems
            .add_system(camera_system);
        self.engine_app
            .as_mut()
            .unwrap()
            .systems
            .add_system(render_system);
    }

    pub fn sort_all_systems(&mut self) {
        let item = &mut self.engine_app.as_mut().unwrap().systems;
        item.setups.sort();
        item.updates.sort();
        item.window_events.sort();
        item.device_events.sort();
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
        app.insert_default_resources(window.clone());
        app.systems.setups.execute(&mut app.world);
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
        let mut time = world.get_mut::<Time>();
        time.update();
        let delta = time.time_delta_secs();
        println!("FPS: {:#?}", time.fps());
        drop(time);

        app.systems.updates.execute((world, delta));
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
