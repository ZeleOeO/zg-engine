use std::any::{Any, TypeId};

use winit::{
    event::{DeviceEvent, WindowEvent},
    event_loop::ActiveEventLoop,
};

use crate::{
    systems::{system_sort::SystemSort, system_storage::*},
    world::world::World,
};

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

#[derive(Debug)]
pub struct SystemID(pub TypeId);

#[derive(Debug)]
pub struct SystemMut<A: SystemFunction> {
    pub id: SystemID,
    pub sorts: Vec<SystemSort>,
    pub callback: Box<A::Fntype>,
}

impl<A: SystemFunction + 'static> SystemMut<A> {
    pub fn before<F, M>(&mut self, callback: F)
    where
        M: SystemFunction,
        F: IntoSystem<M>,
    {
        self.sorts
            .push(SystemSort::Before(SystemID(callback.type_id())));
    }
    pub fn after<F, M>(&mut self, callback: F)
    where
        M: SystemFunction,
        F: IntoSystem<M>,
    {
        self.sorts
            .push(SystemSort::After(SystemID(callback.type_id())));
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

impl<'a> SystemAggregator<'a> {
    pub fn insert_init_system<F: FnMut(&mut World) + 'static>(
        &mut self,
        callback: F,
    ) -> &mut SystemMut<WorldOnly> {
        let system_mut: SystemMut<WorldOnly> = SystemMut {
            id: SystemID(TypeId::of::<F>()),
            sorts: Vec::new(),
            callback: Box::new(callback),
        };
        let item = self.setups.insert(system_mut);
        item
    }

    pub fn insert_update_system<F: FnMut(&mut World) + 'static>(
        &mut self,
        callback: F,
    ) -> &mut SystemMut<WorldOnly> {
        let system_mut: SystemMut<WorldOnly> = SystemMut {
            id: SystemID(TypeId::of::<F>()),
            sorts: Vec::new(),
            callback: Box::new(callback),
        };
        let item = self.updates.insert(system_mut);
        item
    }

    pub fn insert_window_event_sytem<
        F: FnMut(&mut World, &WindowEvent, &ActiveEventLoop) + 'static,
    >(
        &mut self,
        callback: F,
    ) -> &mut SystemMut<WindowSystemEvent> {
        let system_mut: SystemMut<WindowSystemEvent> = SystemMut {
            id: SystemID(TypeId::of::<F>()),
            sorts: Vec::new(),
            callback: Box::new(callback),
        };
        let item = self.window_events.insert(system_mut);
        item
    }

    pub fn insert_device_event_sytem<F: FnMut(&mut World, &DeviceEvent) + 'static>(
        &mut self,
        callback: F,
    ) -> &mut SystemMut<DeviceSystemEvent> {
        let system_mut: SystemMut<DeviceSystemEvent> = SystemMut {
            id: SystemID(TypeId::of::<F>()),
            sorts: Vec::new(),
            callback: Box::new(callback),
        };
        let item = self.device_events.insert(system_mut);
        item
    }
}

pub trait SystemFunction: Sized {
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

pub trait IntoSystem<M: SystemFunction>: 'static {}

impl<F> IntoSystem<WorldOnly> for F where F: FnMut(&mut World) + 'static {}

impl<F> IntoSystem<WindowSystemEvent> for F where
    F: FnMut(&mut World, &WindowEvent, &ActiveEventLoop) + 'static
{
}

impl<F> IntoSystem<DeviceSystemEvent> for F where F: FnMut(&mut World, &DeviceEvent) + 'static {}
