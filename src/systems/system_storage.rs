use std::any::TypeId;

use winit::{
    event::{DeviceEvent, WindowEvent},
    event_loop::ActiveEventLoop,
};

use crate::{systems::system_struct::*, world::world::World};

// Made this so I can have a .execute()
pub struct SystemsStorage<A: SystemFunction + 'static> {
    pub systems: Vec<SystemMut<A>>,
}

impl<A: SystemFunction + 'static> SystemsStorage<A> {
    pub fn insert(&mut self, item: Box<A::Fntype>) {
        let systemmut: SystemMut<A> = SystemMut {
            id: SystemID(TypeId::of::<A::Fntype>()),
            callback: item,
        };
        self.systems.push(systemmut);
    }

    pub fn execute(&mut self, mut args: A::Args<'_, '_>) {
        for system in &mut self.systems {
            A::execute(&mut system.callback, &mut args);
        }
    }

    pub fn sort() {
        // i need to sort this by the systemID
        // I need the SystemMut to store a before and after thing
        // we go through each one and then arrange them in that way
        // alredy have a note with a naive approach
        // it goes
        // 1 -> before 3 after 2
        // 2, 1, 3
        // 2 -> after 3, but it'll move 2 behind 3 without thinking about why it's there
        // so this may need a data structure
        // going to  implement my own topo sort
    }
}

impl<A: SystemFunction> Default for SystemsStorage<A> {
    fn default() -> Self {
        Self {
            systems: Vec::default(),
        }
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
