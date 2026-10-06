use std::sync::Arc;
use winit::window::{CursorGrabMode, Window};

use zg_camera::CameraController;
use zg_graphics::InternalGraphics;
use zg_render::{RenderQueue, WorldRenderer};

use zg_managers::Assets;
use zg_time::Time;
use zg_window::{EngineWindowEvents, WindowRes};
use zg_world::{Events, ScheduleLabel, SystemsSchedule, Update, World};

pub(crate) struct EngineApp {
    pub world: World,
    pub systems: SystemsSchedule,
    pub window: Option<Arc<Window>>,
}

impl EngineApp {
    pub(crate) async fn new() -> Self {
        let world = World::new();
        let systems = SystemsSchedule::new();
        Self {
            world,
            systems,
            window: None,
        }
    }

    pub fn add_window(&mut self, window: Arc<Window>) {
        window.set_cursor_visible(false);
        window
            .set_cursor_grab(CursorGrabMode::Confined)
            .or_else(|_e| window.set_cursor_grab(CursorGrabMode::Locked))
            .unwrap();
        self.window = Some(window)
    }

    pub(crate) fn insert_default_resources(&mut self, window: Arc<Window>) {
        let internal_graphics = pollster::block_on(InternalGraphics::new(&window)).unwrap();
        let assets = Assets::new();
        let camera_controller = CameraController::new(2.0, 0.2);
        let render_queue = RenderQueue::default();
        let renderer = WorldRenderer::new(&internal_graphics);
        let time = Time::new();
        let window_res = WindowRes::new(window);

        self.world.insert(window_res);
        self.world.insert(internal_graphics);
        self.world.insert(assets);
        self.world.insert(camera_controller);
        self.world.insert(render_queue);
        self.world.insert(renderer);
        self.world.insert(time);
    }

    pub(crate) fn execute_schedule(&mut self, label: impl ScheduleLabel) {
        self.systems.execute(label, &self.world);
    }

    pub(crate) fn send_window_events(&mut self) {
        // forward events
        let world = &self.world;
        let mut window_res = world.get_resource_mut::<WindowRes>();
        let window_events = window_res.events.drain(..).collect::<Vec<_>>();
        let mut window_event_res = world.get_resource_mut::<Events<EngineWindowEvents>>();
        window_event_res.write_batch(window_events);
    }
}
