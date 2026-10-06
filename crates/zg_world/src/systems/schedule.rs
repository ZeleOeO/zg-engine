use std::{any::TypeId, collections::HashMap, marker::PhantomData};

use zg_utils::sort_vector;

use crate::{
    World,
    systems::{
        schedule_label::ScheduleLabel,
        system::{ExecFunction, System, SystemFunction, SystemID},
        // system_set::SystemSet,
    },
};

pub struct SystemsSchedule {
    schedules: HashMap<Box<dyn ScheduleLabel>, Schedule>,
}

impl SystemsSchedule {
    pub fn new() -> SystemsSchedule {
        let schedules: HashMap<Box<dyn ScheduleLabel>, Schedule> = HashMap::new();

        SystemsSchedule { schedules }
    }

    fn entry(&mut self, label: impl ScheduleLabel) -> &mut Schedule {
        self.schedules
            .entry(Box::new(label))
            .or_insert_with(|| Schedule::default())
    }

    pub fn add_system<'w, Args, F, S>(&mut self, label: impl ScheduleLabel, function: F)
    where
        F: SystemFunction<Args, State = S> + 'static,
        S: 'static,
        Args: 'static,
    {
        self.entry(label).add(function);
    }

    pub fn execute(&mut self, label: impl ScheduleLabel, world: &mut World) {
        let schedule = self
            .schedules
            .entry(Box::new(label))
            .or_insert_with(|| Schedule::default());

        schedule.sort();
        schedule.run(world);
        // like here
    }
    // pub fn add_system_set(&mut self, system_set: &mut SystemSet) {
    //     for (k, mut v) in system_set.schedules.drain() {
    //         if let Some(schedule) = self.schedules.get_mut(&k) {
    //             schedule.systems.append(&mut v.systems);
    //         } else {
    //             self.schedules.insert(k, v);
    //         }
    //     }
    // }
}

#[derive(Default)]
pub struct Schedule {
    systems: Vec<System>,
}

impl Schedule {
    pub fn add<'w, Args, F, S>(&mut self, function: F)
    where
        F: SystemFunction<Args, State = S> + 'static,
        S: 'static,
        Args: 'static,
    {
        let system = System {
            id: SystemID(TypeId::of::<F>()),
            sorts: Vec::new(),
            system: Box::new(ExecFunction {
                function,
                state: F::init(),
                _phantom_data: PhantomData,
            }),
        };
        self.systems.push(system);
    }

    pub fn sort(&mut self) {
        // println!("ID Before Sort {:#?}", self);
        if !sort_vector(&mut self.systems) {
            panic!("Cyclic dependency")
        };
    }

    fn run(&mut self, world: &mut World) {
        for system in &mut self.systems {
            system.run(world);
        }
    }
}
