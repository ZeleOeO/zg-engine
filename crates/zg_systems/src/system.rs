use std::any::TypeId;

use zg_utils::NodeTrait;

use crate::{system_sort::SystemSort, system_struct::SystemID};

pub trait SystemFunction<Args> {
    fn call(&self, context: &Context);
}

#[derive(Clone)]
pub struct Context {}

pub struct System {
    pub id: SystemID,
    pub sorts: Vec<SystemSort>,
    pub system: Box<dyn Fn(&Context)>,
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
