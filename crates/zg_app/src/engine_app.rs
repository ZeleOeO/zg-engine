use std::{fmt::Debug, sync::Arc};
use winit::window::{CursorGrabMode, Window};

use zg_world::{
    EngineWindowEvents, Events, IntoSystemConfig, Resource, SystemSet, SystemsSchedule, World,
    schedule_label::ScheduleLabel,
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

    pub fn set_order(&mut self, system_set_a: impl SystemSet, system_set_b: impl SystemSet) {
        self.schedules.set_order(system_set_a, system_set_b);
    }

    pub fn add_event<E: Debug + Clone + 'static>(&mut self) {
        self.world.add_event::<E>();
    }

    pub(crate) fn sort_schedules(&mut self) {
        self.schedules.sort();
    }

    pub(crate) fn should_exit(&self) -> bool {
        let exit = self.world.get_resource::<AppExit>();
        exit.0
    }
}

#[derive(Debug, Resource)]
pub struct AppExit(pub bool);
impl AppExit {
    pub fn new() -> Self {
        Self(false)
    }
}
