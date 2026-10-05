use std::collections::HashMap;

use crate::systems::{schedule::Schedule, schedule_label::ScheduleLabel, system::SystemFunction};

pub struct SystemSet {
    pub(crate) schedules: HashMap<Box<dyn ScheduleLabel>, Schedule>,
}

impl SystemSet {
    pub fn new() -> Self {
        Self {
            schedules: HashMap::new(),
        }
    }
    pub fn add_system<'w, Args, F>(mut self, label: impl ScheduleLabel, function: F) -> Self
    where
        F: SystemFunction<Args> + 'static,
    {
        let schedule = self
            .schedules
            .entry(Box::new(label))
            .or_insert_with(|| Schedule::default());

        schedule.add(function);
        self
    }
}

