use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use zg_utils::NodeTrait;
use zg_world_macros::SystemSet;

use crate::{System, schedule_label::ScheduleLabel};

pub trait SystemSet: 'static {
    fn variant(&self) -> usize {
        0
    }

    fn set_id(&self) -> SetID {
        SetID(TypeId::of::<Self>(), self.variant())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct SetID(pub TypeId, pub usize);

pub struct SystemSetNode {
    pub id: SetID,
    pub prev_ids: Vec<SetID>,
    pub next_ids: Vec<SetID>,
    pub systems: HashMap<Box<dyn ScheduleLabel>, Vec<System>>,
}

impl NodeTrait for SystemSetNode {
    type ID = SetID;
    fn node_id(&self) -> &Self::ID {
        &self.id
    }
    fn prev_ids(&self) -> Vec<Self::ID> {
        self.prev_ids.clone()
    }
    fn next_ids(&self) -> Vec<Self::ID> {
        self.next_ids.clone()
    }
}

impl SystemSetNode {
    pub fn new(id: SetID) -> Self {
        Self {
            id,
            prev_ids: Vec::new(),
            next_ids: Vec::new(),
            systems: HashMap::new(),
        }
    }
}

#[derive(SystemSet)]
pub struct DefaultSet;
