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
pub struct Setup1 {}
impl ScheduleLabel for Setup1 {}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Update1 {}
impl ScheduleLabel for Update1 {}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct WindowSystemEvent1 {}
impl ScheduleLabel for WindowSystemEvent1 {}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct DeviceSystemEvent1 {}
impl ScheduleLabel for DeviceSystemEvent1 {}
