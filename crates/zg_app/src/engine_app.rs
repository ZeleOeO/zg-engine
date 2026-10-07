use std::sync::Arc;
use winit::window::{CursorGrabMode, Window};

use zg_world::{
    EngineWindowEvents, Events, IntoSystemConfig, Resource, System, SystemFunction, SystemSet,
    SystemsSchedule, World, schedule_label::ScheduleLabel,
};

pub(crate) struct EngineApp {
    pub world: World,
    pub schedules: SystemsSchedule,
    pub window: Option<Arc<Window>>,
    pub(crate) window_events: Vec<EngineWindowEvents>,
}

impl EngineApp {
    pub(crate) async fn new() -> Self {
        let world = World::new();
        let systems = SystemsSchedule::new();
        Self {
            world,
            schedules: systems,
            window: None,
            window_events: Vec::new(),
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
        self.schedules.execute(label, &mut self.world);
    }

    pub(crate) fn send_window_events(&mut self) {
        let world = &self.world;
        let window_events = self.window_events.drain(..).collect::<Vec<_>>();
        let mut window_event_res = world.get_resource_mut::<Events<EngineWindowEvents>>();
        window_event_res.write_batch(window_events);
    }

    pub fn add_system<C, M>(&mut self, label: impl ScheduleLabel, config: C)
    where
        C: IntoSystemConfig<M> + 'static,
        <C as IntoSystemConfig<M>>::Func: 'static,
        M: 'static,
    {
        self.schedules.add_system(label, config)
    }

    pub fn add_resource<R: Resource + 'static>(&mut self, resource: R) {
        self.world.insert::<R>(resource);
    }
}
