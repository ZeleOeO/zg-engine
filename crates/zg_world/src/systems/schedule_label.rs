use std::hash::{Hash, Hasher};

use zg_utils::DynHash;

pub trait ScheduleLabel: DynHash + 'static {}

impl Hash for dyn ScheduleLabel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.dyn_hash(state);
    }
}

impl PartialEq for dyn ScheduleLabel {
    fn eq(&self, other: &Self) -> bool {
        self.dyn_eq(other)
    }
}

impl Eq for dyn ScheduleLabel {}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Setup;
impl ScheduleLabel for Setup {}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Update;
impl ScheduleLabel for Update {}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct PreUpdate;
impl ScheduleLabel for PreUpdate {}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct DeviceSystemEvent;
impl ScheduleLabel for DeviceSystemEvent {}
