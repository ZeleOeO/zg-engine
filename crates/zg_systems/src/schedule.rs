use std::{any::TypeId, collections::HashMap};

use zg_utils::sort_vector;

use crate::{
    schedule_label::ScheduleLabel,
    system::{Context, System, SystemFunction},
    system_struct::SystemID,
};

#[derive(Default)]
pub struct Schedule {
    systems: Vec<System>,
}

impl Schedule {
    fn add<Args, F>(&mut self, function: F)
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

    fn run(&self, context: &Context) {
        for system in &self.systems {
            (system.system)(context)
        }
    }
}

//dummy syntax
// fn(so so and so with sytemparam)
// app.add_system(schedule_label, (fn))
// app.schedule.sort()
// app.schedule.run()

// This will be what app would take in, instead of Systems
pub struct ScheduleSystems {
    schedules: HashMap<Box<dyn ScheduleLabel>, Schedule>,
    context: Context,
}

impl ScheduleSystems {
    fn entry(&mut self, label: impl ScheduleLabel) -> &mut Schedule {
        self.schedules
            .entry(Box::new(label))
            .or_insert_with(|| Schedule::default())
    }

    pub fn add_system<Args, F>(&mut self, label: impl ScheduleLabel, function: F)
    where
        F: SystemFunction<Args> + 'static,
    {
        self.entry(label).add(function);
    }

    pub fn execute(&mut self, label: impl ScheduleLabel) {
        // something something borrow checker
        //  free me
        let schedule = self
            .schedules
            .entry(Box::new(label))
            .or_insert_with(|| Schedule::default());

        schedule.run(&self.context);
    }
}
