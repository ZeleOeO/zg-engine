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

    pub fn set_order(&mut self, system_set_a: impl SystemSet, system_set_b: impl SystemSet) {
        let a_set_id = SetID(system_set_a.type_id());
        let b_set_id = SetID(system_set_b.type_id());
        self.set_mut(a_set_id).next_ids.push(b_set_id);
        self.set_mut(b_set_id).prev_ids.push(a_set_id);
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
