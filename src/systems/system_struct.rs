use std::any::{Any, TypeId};

use winit::{
    event::{DeviceEvent, WindowEvent},
    event_loop::ActiveEventLoop,
};

use crate::{systems::system_storage::*, world::world::World};

#[derive(Default)]
pub struct Systems {
    pub setups: SystemsStorage<WorldOnly>,
    pub updates: SystemsStorage<WorldOnly>,
    pub window_events: SystemsStorage<WindowSystemEvent>,
    pub device_events: SystemsStorage<DeviceSystemEvent>,
}

pub struct SystemAggregator<'a> {
    pub setups: &'a mut SystemsStorage<WorldOnly>,
    pub updates: &'a mut SystemsStorage<WorldOnly>,
    pub window_events: &'a mut SystemsStorage<WindowSystemEvent>,
    pub device_events: &'a mut SystemsStorage<DeviceSystemEvent>,
}

pub struct SystemID(pub TypeId);

pub struct SystemMut<A: SystemFunction> {
    // This will be used for sorting
    pub id: SystemID,
    pub callback: Box<A::Fntype>,
}

impl<'a> SystemAggregator<'a> {
    pub fn insert_init_system<F: FnMut(&mut World) + 'static>(&mut self, callback: F) -> &mut Self {
        self.setups.insert(Box::new(callback));
        self
    }

    pub fn insert_update_system<F: FnMut(&mut World) + 'static>(
        &mut self,
        callback: F,
    ) -> &mut Self {
        self.updates.insert(Box::new(callback));
        self
    }

    pub fn insert_window_event_sytem<
        F: FnMut(&mut World, &WindowEvent, &ActiveEventLoop) + 'static,
    >(
        &mut self,
        callback: F,
    ) -> &mut Self {
        self.window_events.insert(Box::new(callback));
        self
    }

    pub fn insert_device_event_sytem<F: FnMut(&mut World, &DeviceEvent) + 'static>(
        &mut self,
        callback: F,
    ) -> &mut Self {
        self.device_events.insert(Box::new(callback));
        self
    }
}

impl Systems {
    pub fn add_system<F: FnOnce(&mut SystemAggregator)>(&mut self, function: F) {
        let mut agg = SystemAggregator {
            setups: &mut self.setups,
            updates: &mut self.updates,
            window_events: &mut self.window_events,
            device_events: &mut self.device_events,
        };
        function(&mut agg);
    }
}
