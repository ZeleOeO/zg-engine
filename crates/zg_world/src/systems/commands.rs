use std::fmt::Debug;

use crate::{Entity, Resource, SystemParam, World, bundle::Bundle, systems::query::QueryData};

pub type MutWorldCommand = Box<dyn FnOnce(&mut World)>;

pub struct Commands<'c> {
    pub world: &'c World,
    pub command_queue: &'c mut Vec<MutWorldCommand>,
    pub ids: u32,
}

impl Commands<'_> {
    pub fn spawn<T: Bundle + Debug + 'static>(&mut self, bundle: T) -> Entity {
        let entity = Entity(self.ids);
        self.ids += 1;
        self.command_queue.push(Box::new(move |world| {
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

    pub fn insert_resource<R: Resource + 'static>(&mut self, resource: R) {
        self.command_queue
            .push(Box::new(move |world| world.insert(resource)));
    }
}

impl SystemParam for Commands<'_> {
    type Param<'w> = Commands<'w>;
    type State = Vec<MutWorldCommand>;
    fn init_state() -> Self::State {
        Vec::new()
    }
    fn extract_params<'w>(world: &'w World, state: &'w mut Self::State) -> Self::Param<'w> {
        Commands {
            world: world,
            command_queue: state,
            ids: 0,
        }
    }
    fn reset(world: &mut crate::World, state: &mut Self::State) {
        for cmd in state.drain(..) {
            cmd(world)
        }
    }
}
