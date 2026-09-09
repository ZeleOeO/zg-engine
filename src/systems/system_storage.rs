use crate::systems::system_struct::*;

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
        item
    }

    pub fn execute(&mut self, mut args: A::Args<'_, '_>) {
        for system in &mut self.systems {
            A::execute(&mut system.callback, &mut args);
        }
    }

    pub fn sort() {}
}
