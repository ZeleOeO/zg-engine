use winit::{
    event::{DeviceEvent, WindowEvent},
    event_loop::ActiveEventLoop,
};

use crate::world::world::World;

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

// This is how I want System to work
// I have a function
// fn system(system: System) {
// system.init()
// system.whatever()
// }
//
// and then when I add a system, it goes through each one and slots it
// to handle schedule I can do
//
// fn system(system: System) {
// system.init().after() // insert system it should go before or after
// system.whatever().before()
// This means we will need a sorter of some sort but we will get to that later
// }
//
// pub fn add_system(system: SystemAgg) {
//  self.setup.push(system.setup);
//  self.setup.push(system.setup);
//  self.setup.push(system.setup);
// }

impl<'a> SystemAggregator<'a> {
    pub fn insert_init_system<F: FnMut(&mut World) + 'static>(&mut self, callback: F) -> &mut Self {
        self.setups.systems.push(Box::new(callback));
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

    pub fn add_setup_system<F: FnMut(&mut World) + 'static>(&mut self, callback: F) -> &mut Self {
        self.setups.systems.push(Box::new(callback));
        self
    }

    pub fn add_update_system<F: FnMut(&mut World) + 'static>(&mut self, callback: F) -> &mut Self {
        self.updates.insert(Box::new(callback));
        self
    }

    pub fn add_window_event_sytem<
        F: FnMut(&mut World, &WindowEvent, &ActiveEventLoop) + 'static,
    >(
        &mut self,
        callback: F,
    ) -> &mut Self {
        self.window_events.insert(Box::new(callback));
        self
    }

    pub fn add_device_event_sytem<F: FnMut(&mut World, &DeviceEvent) + 'static>(
        &mut self,
        callback: F,
    ) -> &mut Self {
        self.device_events.insert(Box::new(callback));
        self
    }
}

// Made this so I can have a .execute()
pub struct SystemsStorage<A: SystemFunction> {
    pub systems: Vec<Box<A::Fntype>>,
}

impl<A: SystemFunction> SystemsStorage<A> {
    pub fn insert(&mut self, item: Box<A::Fntype>) {
        self.systems.push(item);
    }

    pub fn execute(&mut self, mut args: A::Args<'_, '_>) {
        for system in &mut self.systems {
            A::execute(system, &mut args);
        }
    }
}

impl<A: SystemFunction> Default for SystemsStorage<A> {
    fn default() -> Self {
        Self {
            systems: Vec::default(),
        }
    }
}

pub trait SystemFunction {
    type Fntype: ?Sized;
    type Args<'a, 'b>;

    // changed the lifetimes cause I need to know it's differnt lol
    fn execute<'e, 'f>(function: &mut Box<Self::Fntype>, args: &mut Self::Args<'e, 'f>);
}

pub struct WorldOnly {}
pub struct WindowSystemEvent {}
pub struct DeviceSystemEvent {}
pub struct System {}

impl SystemFunction for WorldOnly {
    type Fntype = dyn FnMut(&mut World);
    type Args<'a, 'b> = &'a mut World;

    fn execute<'e, 'f>(function: &mut Box<Self::Fntype>, args: &mut Self::Args<'e, 'f>) {
        function(args)
    }
}

impl SystemFunction for WindowSystemEvent {
    type Fntype = dyn FnMut(&mut World, &WindowEvent, &ActiveEventLoop);
    type Args<'a, 'b> = (&'a mut World, &'b WindowEvent, &'b ActiveEventLoop);

    fn execute<'e, 'f>(function: &mut Box<Self::Fntype>, args: &mut Self::Args<'e, 'f>) {
        function(args.0, args.1, args.2)
    }
}
impl SystemFunction for DeviceSystemEvent {
    type Fntype = dyn FnMut(&mut World, &DeviceEvent);
    type Args<'a, 'b> = (&'a mut World, &'b DeviceEvent);

    fn execute<'e, 'f>(function: &mut Box<Self::Fntype>, args: &mut Self::Args<'e, 'f>) {
        function(args.0, args.1)
    }
}
