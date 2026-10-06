use std::sync::Arc;

use anyhow::Ok;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::PhysicalKey,
    window::Window,
};
use zg_window::WindowRes;
use zg_world::{EngineWindowEvents, KeyboardInputEvent, MouseMotionEvent, Setup, Update};

use crate::engine_app::EngineApp;

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
        tracy_client::Client::start();
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        // self.insert_default_systems();
        event_loop.run_app(self)?;
        Ok(())
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

        app.execute_schedule(Setup);
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
        let world = &app.world;
        let mut window_res = world.get_resource_mut::<WindowRes>();
        match event {
            DeviceEvent::MouseMotion { delta } => {
                let motion_delta = [delta.0 as f32, delta.1 as f32];
                window_res
                    .events
                    .push(EngineWindowEvents::MouseMotion(MouseMotionEvent {
                        delta: motion_delta,
                    }));
            }
            _ => {}
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Some(app) = &mut self.engine_app else {
            return;
        };

        let world = &app.world;
        let mut window_res = world.get_resource_mut::<WindowRes>();

        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state,
                        ..
                    },
                ..
            } => {
                window_res
                    .events
                    .push(EngineWindowEvents::KeyboardInput(KeyboardInputEvent {
                        code: code,
                        key_pressed: state.is_pressed(),
                    }));
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let Some(app) = &mut self.engine_app else {
            return;
        };

        // i want to update
        // send the events to the world
        // then run preupdate and update
        // let world = &mut app.world;
        // let time = world.get_resource_mut::<Time>();
        // let delta = time.time_delta_secs();
        // drop(time);

        // this sends the windows events to the events resource in the world
        // which will then be read by an event reader
        app.send_window_events();
        app.execute_schedule(Update);
        let Some(window) = &mut app.window else {
            return;
        };
        window.request_redraw();
    }
}
