use std::{any::TypeId, cell::RefCell, collections::HashMap};

use zg_utils::sort_vector;

use crate::{
    World,
    systems::{
        schedule_label::ScheduleLabel,
        system::{System, SystemContext, SystemFunction, SystemID},
        system_set::SystemSet,
    },
};

pub struct SystemsSchedule<'w> {
    schedules: HashMap<Box<dyn ScheduleLabel>, Schedule>,
    pub context: SystemContext<'w>,
}

impl SystemsSchedule<'_> {
    pub fn new<'w>(world: &'w World) -> SystemsSchedule<'w> {
        let schedules: HashMap<Box<dyn ScheduleLabel>, Schedule> = HashMap::new();
        let context = SystemContext {
            world,
            queue: RefCell::new(Vec::new()),
            event_queue: Vec::new(),
        };

        SystemsSchedule { schedules, context }
    }

    fn entry(&mut self, label: impl ScheduleLabel) -> &mut Schedule {
        self.schedules
            .entry(Box::new(label))
            .or_insert_with(|| Schedule::default())
    }

    pub fn add_system<'w, Args, F>(&mut self, label: impl ScheduleLabel, function: F)
    where
        F: SystemFunction<Args> + 'static,
    {
        self.entry(label).add(function);
    }

    pub fn execute(&mut self, label: impl ScheduleLabel) {
        let schedule = self
            .schedules
            .entry(Box::new(label))
            .or_insert_with(|| Schedule::default());

        schedule.run(&self.context);
    }
    pub fn add_system_set(&mut self, system_set: &mut SystemSet) {
        for (k, mut v) in system_set.schedules.drain() {
            if let Some(schedule) = self.schedules.get_mut(&k) {
                schedule.systems.append(&mut v.systems);
            } else {
                self.schedules.insert(k, v);
            }
        }
    }
}

#[derive(Default)]
pub struct Schedule {
    systems: Vec<System>,
}

impl Schedule {
    pub fn add<'w, Args, F>(&mut self, function: F)
    where
        F: SystemFunction<Args> + 'static,
    {
        let system = System {
            id: SystemID(TypeId::of::<F>()),
            sorts: Vec::new(),
            system: Box::new(move |ctx| function.call(ctx)),
        };
        self.systems.push(system);
    }

    pub fn sort(&mut self) {
        // println!("ID Before Sort {:#?}", self);
        if !sort_vector(&mut self.systems) {
            panic!("Cyclic dependency")
        };
    }

    fn run(&self, context: &SystemContext) {
        for system in &self.systems {
            (system.system)(context)
        }
    }
}
