use std::any::TypeId;

use zg_utils::NodeTrait;

use crate::{World, systems::system_sort::SystemSort};

#[derive(Debug)]
pub struct SystemID(pub TypeId);

pub trait SystemFunction<Args> {
    fn call<'w>(&self, context: &'w World);
}

// #[derive(Clone)]
// pub struct Context {}

pub struct System {
    pub id: SystemID,
    pub sorts: Vec<SystemSort>,
    pub system: Box<dyn Fn(&World)>,
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
