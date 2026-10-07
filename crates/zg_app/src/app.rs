use std::{fmt::Debug, sync::Arc};

use anyhow::Ok;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::PhysicalKey,
    window::Window,
};
use zg_world::{
    EngineWindowEvents, IntoSystemConfig, KeyboardInputEvent, MouseMotionEvent, Resource,
    SystemSet,
    schedule_label::{PreUpdate, ScheduleLabel, Setup, Update},
};

use crate::{
    addons::{Addon, WindowHandle},
    engine_app::EngineApp,
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
        tracy_client::Client::start();
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        self.sort_schedules();
        event_loop.run_app(self)?;
        Ok(())
    }

    pub(crate) fn sort_schedules(&mut self) {
        let Some(app) = &mut self.engine_app else {
            panic!("No engine app found");
        };
        app.sort_schedules();
    }

    pub fn window(&self) -> Arc<Window> {
        let Some(app) = &self.engine_app else {
            panic!("No engine app found");
        };
        app.window.clone().unwrap()
    }

    pub fn add_addons(&mut self, add_on: impl Addon) -> &mut Self {
        add_on.build(self);
        self
    }

    pub fn add_system<C, M>(&mut self, label: impl ScheduleLabel, config: C) -> &mut Self
    where
        C: IntoSystemConfig<M> + 'static,
        <C as IntoSystemConfig<M>>::Func: 'static,
        M: 'static,
    {
        let Some(app) = &mut self.engine_app else {
            panic!("No engine app found");
        };
        app.add_system(label, config);
        self
    }

    pub fn add_resource<R: Resource + 'static>(&mut self, resource: R) -> &mut Self {
        let Some(app) = &mut self.engine_app else {
            panic!("No engine app found");
        };
        app.add_resource::<R>(resource);
        self
    }

    pub fn set_order(
        &mut self,
        system_set_a: impl SystemSet,
        system_set_b: impl SystemSet,
    ) -> &mut Self {
        let Some(app) = &mut self.engine_app else {
            panic!("No engine app found");
        };
        app.set_order(system_set_a, system_set_b);
        self
    }

    pub fn add_event<E: Debug + Clone + 'static>(&mut self) -> &mut Self {
        let Some(app) = &mut self.engine_app else {
            panic!("No engine app found");
        };
        app.add_event::<E>();
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

        let Some(app) = &mut self.engine_app else {
            return;
        };

        if app.window.is_some() {
            return;
        }

        app.add_window(window.clone());
        app.add_resource(WindowHandle(window.clone()));

        app.execute_schedule(Setup);
        window.request_redraw();
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
                app.window_events
                    .push(EngineWindowEvents::KeyboardInput(KeyboardInputEvent {
                        code: code,
                        key_pressed: state.is_pressed(),
                    }));
            }
            _ => {}
        }
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
            DeviceEvent::MouseMotion { delta } => {
                let motion_delta = [delta.0 as f32, delta.1 as f32];
                app.window_events
                    .push(EngineWindowEvents::MouseMotion(MouseMotionEvent {
                        delta: motion_delta,
                    }));
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let Some(app) = &mut self.engine_app else {
            return;
        };

        app.send_window_events();
        app.execute_schedule(PreUpdate);
        app.execute_schedule(Update);

        if app.should_exit() {
            event_loop.exit();
            return;
        }

        let Some(window) = &mut app.window else {
            return;
        };
        window.request_redraw();
    }
}
