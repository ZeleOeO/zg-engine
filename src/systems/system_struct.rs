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

pub enum SystemSchedule {
    Setup,
    Update,
    WindowEvent,
}

impl Systems {
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
