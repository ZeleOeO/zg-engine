use std::fmt::Debug;

use crate::{systems::system_struct::*, utils::topo_sort::sort_vector};

pub struct SystemsStorage<A: SystemFunction + 'static> {
    pub systems: Vec<SystemMut<A>>,
}

impl<A: SystemFunction + 'static> Debug for SystemsStorage<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for system in &self.systems {
            write!(f, "{:?}", system.id.0)?;
        }

        Ok(())
    }
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

    pub fn sort(&mut self) {
        println!("ID Before Sort {:#?}", self);
        if !sort_vector(&mut self.systems) {
            panic!("Cyclic dependency")
        };
    }
}
