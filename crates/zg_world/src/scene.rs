use std::{cell::RefCell, fmt::Debug};

use crate::{Entity, SystemParam, World, bundle::Bundle, query::QueryData};

pub type MutWorldCommand = Box<dyn FnOnce(&mut World)>;

pub struct Scene<'c> {
    pub world: &'c World,
    pub command_queue: &'c RefCell<Vec<MutWorldCommand>>,
    pub ids: u32,
}

impl Scene<'_> {
    pub fn spawn<T: Bundle + Debug + 'static>(&mut self, bundle: T) -> Entity {
        let entity = Entity(self.ids);
        self.ids += 1;
        self.command_queue.borrow_mut().push(Box::new(move |world| {
            world.spawn(bundle);
        }));
        entity
    }

    pub fn get_entity<'w, D>(&'w self, entity: Entity) -> D::Output
    where
        D: QueryData<'w> + 'static,
    {
        self.world.get_entity::<D>(entity)
    }
}

impl SystemParam for Scene<'_> {
    type Item<'w> = Scene<'w>;
    fn extract_world_context<'w>(context: &'w crate::SystemContext) -> Self::Item<'w> {
        Scene {
            world: &context.world,
            command_queue: &context.queue,
            ids: 0,
        }
    }
}
