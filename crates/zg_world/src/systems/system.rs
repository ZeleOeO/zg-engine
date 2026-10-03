use std::{any::TypeId, cell::RefCell};
use winit::event_loop::ActiveEventLoop;
use zg_utils::NodeTrait;

use crate::{World, events::EngineEvents, scene::Command, systems::system_sort::SystemSort};

pub struct SystemContext<'s> {
    pub world: &'s World,
    pub mut_world: &'s mut World,
    pub event_loop: Option<&'s ActiveEventLoop>,
    pub event: &'s EngineEvents,
    pub queue: RefCell<Vec<Command>>,
}

#[derive(Debug)]
pub struct SystemID(pub TypeId);

pub struct System {
    pub id: SystemID,
    pub sorts: Vec<SystemSort>,
    pub system: Box<dyn Fn(&SystemContext)>,
}

impl NodeTrait for System {
    type ID = TypeId;

    fn node_id(&self) -> &Self::ID {
        &self.id.0
    }

    fn prev_ids(&self) -> Vec<Self::ID> {
        self.sorts
            .iter()
            .filter_map(|sort| match sort {
                SystemSort::After(id) => Some(id.0),
                _ => None,
            })
            .collect()
    }

    fn next_ids(&self) -> Vec<Self::ID> {
        self.sorts
            .iter()
            .filter_map(|sort| match sort {
                SystemSort::Before(id) => Some(id.0),
                _ => None,
            })
            .collect()
    }
}

pub trait SystemFunction<Args> {
    fn call<'w>(&self, context: &'w SystemContext);
}

pub trait SystemParam {
    type Item<'w>;
    fn extract_world_context<'w>(context: &'w SystemContext) -> Self::Item<'w>;
}

macro_rules! impl_for_system_function {
    ($($param:ident),*) => {
        impl<T, $($param),*> $crate::systems::system::SystemFunction<($($param),*)> for T
        where
            T: Fn($($param),*),
            T: for<'w> Fn($(<$param as $crate::systems::system::SystemParam>::Item<'w>),*),
            $($param: $crate::systems::system::SystemParam),*
        {
            fn call<'w>(&self, context: &'w $crate::systems::system::SystemContext) {
                (self)($($param::extract_world_context(context)),*)
            }
        }
    };
}

impl_for_system_function!(A);
impl_for_system_function!(A, B);
impl_for_system_function!(A, B, C);
impl_for_system_function!(A, B, C, D);
impl_for_system_function!(A, B, C, D, E);
impl_for_system_function!(A, B, C, D, E, F);
