use std::{
    any::{Any, TypeId},
    collections::HashMap,
    marker::PhantomData,
};

use zg_utils::sort_vector;

use crate::{
    SystemFunction, World,
    systems::{
        schedule_label::ScheduleLabel,
        system::{ExecFunction, IntoSystemConfig, System, SystemID},
        system_set::{SetID, SystemSet, SystemSetNode},
    },
};

pub struct SystemsSchedule {
    sets: HashMap<SetID, SystemSetNode>,
    sorted_schedules: HashMap<Box<dyn ScheduleLabel>, Vec<System>>,
}

impl SystemsSchedule {
    pub fn new() -> SystemsSchedule {
        SystemsSchedule {
            sets: HashMap::new(),
            sorted_schedules: HashMap::new(),
        }
    }

    fn set_mut(&mut self, id: SetID) -> &mut SystemSetNode {
        self.sets
            .entry(id)
            .or_insert_with(|| SystemSetNode::new(id))
    }

    pub fn add_system_set(&mut self, system_set: impl SystemSet) {
        let id = SetID(system_set.type_id());
        self.set_mut(id);
    }

    // pub fn add_system<'w, Args, F, S>(
    //     &mut self,
    //     label: impl ScheduleLabel,
    //     function: F,
    // ) -> &mut Self
    // where
    //     F: SystemFunction<Args, State = S> + 'static,
    //     S: 'static,
    //     Args: 'static,

    pub fn add_system<C, M>(&mut self, label: impl ScheduleLabel, config: C)
    where
        C: IntoSystemConfig<M> + 'static,
        <C as IntoSystemConfig<M>>::Func: 'static,
        M: 'static,
    {
        let (function, set) = config.into_config();

        let system = System {
            id: SystemID(TypeId::of::<C::Func>()),
            sorts: Vec::new(),
            system: Box::new(ExecFunction {
                function,
                state: C::Func::init(),
                _phantom_data: PhantomData,
            }),
        };
        let set_id = SetID(set.type_id());
        self.set_mut(set_id)
            .systems
            .entry(Box::new(label))
            .or_default()
            .push(system);
    }

    // pub fn add_system<Args, F, S>(
    //     &mut self,
    //     label: impl ScheduleLabel,
    //     set: impl SystemSet,
    //     function: F,
    // ) where
    //     F: SystemFunction<Args, State = S> + 'static,
    //     S: 'static,
    //     Args: 'static,
    // {
    //     let system = System {
    //         id: SystemID(TypeId::of::<F>()),
    //         sorts: Vec::new(),
    //         system: Box::new(ExecFunction {
    //             function,
    //             state: F::init(),
    //             _phantom_data: PhantomData,
    //         }),
    //     };
    //     let set_id = SetID(set.type_id());
    //     self.set_mut(set_id)
    //         .systems
    //         .entry(Box::new(label))
    //         .or_default()
    //         .push(system);
    // }

    pub fn set_before(&mut self, a: SetID, b: SetID) {
        self.set_mut(a).next_ids.push(b);
        self.set_mut(b).prev_ids.push(a);
    }

    pub fn sort(&mut self) {
        let mut nodes: Vec<SystemSetNode> = self.sets.drain().map(|(_, n)| n).collect();

        if !sort_vector(&mut nodes) {
            panic!("Cyclic dependency");
        }

        for mut node in nodes {
            for (label, systems) in node.systems.drain() {
                self.sorted_schedules
                    .entry(label)
                    .or_default()
                    .extend(systems);
            }
        }
    }

    pub fn execute(&mut self, label: impl ScheduleLabel, world: &mut World) {
        let label: Box<dyn ScheduleLabel> = Box::new(label);

        if let Some(systems) = self.sorted_schedules.get_mut(&label) {
            for system in systems {
                system.run(world);
            }
        }
    }
}
//
// #[derive(Default)]
// pub struct Schedule {
//     systems: Vec<System>,
// }
//
// impl Schedule {
//     // pub fn add<'w, Args, F, S>(&mut self, function: F) -> &mut System
//     // where
//     //     F: SystemFunction<Args, State = S> + 'static,
//     //     S: 'static,
//     //     Args: 'static,
//     // {
//     //     let system = System {
//     //         id: SystemID(TypeId::of::<F>()),
//     //         sorts: Vec::new(),
//     //         system: Box::new(ExecFunction {
//     //             function,
//     //             state: F::init(),
//     //             _phantom_data: PhantomData,
//     //         }),
//     //     };
//     //     self.systems.push_mut(system)
//     // }
//     //
//     // pub fn sort(&mut self) {
//     //     if !sort_vector(&mut self.systems) {
//     //         panic!("Cyclic dependency")
//     //     };
//     // }
//
//     fn run(&mut self, world: &mut World) {
//         for system in &mut self.systems {
//             system.run(world);
//         }
//     }
// }
