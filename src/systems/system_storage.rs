use winit::{
    event::{DeviceEvent, WindowEvent},
    event_loop::ActiveEventLoop,
};

use crate::{systems::system_struct::*, world::world::World};

pub struct SystemsStorage<A: SystemFunction + 'static> {
    pub systems: Vec<SystemMut<A>>,
}

impl<A: SystemFunction> Default for SystemsStorage<A> {
    fn default() -> Self {
        Self {
            systems: Vec::default(),
        }
    }
}

impl<A: SystemFunction + 'static> SystemsStorage<A> {
    pub fn insert(&mut self, system_mut: SystemMut<A>) -> &mut SystemMut<A> {
        let item = self.systems.push_mut(system_mut);
        println!("Type ID: {:#?}", item.id);
        item
    }

    pub fn execute(&mut self, mut args: A::Args<'_, '_>) {
        for system in &mut self.systems {
            A::execute(&mut system.callback, &mut args);
        }
    }

    pub fn sort() {}
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
