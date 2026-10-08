use std::{cell::RefMut, fmt::Debug};

use crate::{Events, MutWorldCommand, SystemParam};

pub struct EventReader<'e, E: Debug + Clone + 'static> {
    pub events: RefMut<'e, Events<E>>,
    pub command_queue: &'e mut Vec<MutWorldCommand>,
}

impl<E: Debug + Clone + 'static> SystemParam for EventReader<'_, E> {
    type Param<'w> = EventReader<'w, E>;
    type State = Vec<MutWorldCommand>;
    fn init_state() -> Self::State {
        Vec::new()
    }
    fn extract_params<'w>(world: &'w crate::World, state: &'w mut Self::State) -> Self::Param<'w> {
        EventReader {
            events: world.get_resource_mut::<Events<E>>(),
            command_queue: state,
        }
    }
    fn reset(world: &mut crate::World, state: &mut Self::State) {
        for cmd in state.drain(..) {
            cmd(world)
        }
    }
}

impl<'e, E: Debug + Clone> EventReader<'e, E> {
    pub fn read(&mut self) -> Vec<E> {
        self.events.read()
    }
}
