use std::sync::Arc;
use winit::window::{CursorGrabMode, Window};

use zg_window::WindowRes;
use zg_world::{EngineWindowEvents, Events, ScheduleLabel, SystemsSchedule, World};

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

    pub(crate) fn execute_schedule(&mut self, label: impl ScheduleLabel) {
        self.systems.execute(label, &mut self.world);
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
