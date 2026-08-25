use winit::event::WindowEvent;

use crate::world::world::World;

type SetupFn = fn();
type UpdateFn = fn();

pub struct System {
    pub setups: Vec<Box<dyn FnMut(&mut World)>>,
    pub updates: Vec<Box<dyn FnMut(&mut World)>>,
    pub window_events: Vec<Box<dyn FnMut(&mut World, WindowEvent)>>,
}

pub enum SystemSchedule {
    Setup,
    Update,
    WindowEvent,
}

impl System {
    pub fn new() -> Self {
        Self {
            setups: Vec::new(),
            updates: Vec::new(),
            window_events: Vec::new(),
        }
    }

    pub fn add_setup_system<F: FnMut(&mut World) + 'static>(&mut self, callback: F) -> &mut Self {
        self.setups.push(Box::new(callback));
        self
    }

    pub fn add_update_system<F: FnMut(&mut World) + 'static>(&mut self, callback: F) -> &mut Self {
        self.updates.push(Box::new(callback));
        self
    }

    pub fn add_window_event_sytem<F: FnMut(&mut World, WindowEvent) + 'static>(
        &mut self,
        callback: F,
    ) -> &mut Self {
        self.window_events.push(Box::new(callback));
        self
    }

    // pub fn add_system(schedule_time: SystemSchedule ) {
    // }
}
