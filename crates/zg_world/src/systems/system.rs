use std::{
    any::{Any, TypeId},
    fmt::Debug,
    marker::PhantomData,
};
use zg_utils::NodeTrait;

use crate::{
    SystemSet, World,
    systems::{
        system_set::{DefaultSet, SetID},
        system_sort::SystemSort,
    },
};

#[derive(Debug)]
pub struct SystemID(pub TypeId);

pub struct System {
    pub id: SystemID,
    pub sorts: Vec<SystemSort>,
    pub system: Box<dyn ErasedExecFunction>,
}

impl System {
    pub fn run(&mut self, mut world: &mut World) {
        self.system.call(&world);
        self.system.reset(&mut world);
    }
}

pub trait ErasedExecFunction {
    fn call(&mut self, world: &World);
    fn reset(&mut self, world: &mut World);
}

pub(crate) struct ExecFunction<F, Args>
where
    F: SystemFunction<Args>,
{
    pub(crate) function: F,
    pub(crate) state: F::State,
    pub(crate) _phantom_data: PhantomData<Args>,
}

impl<F, Args> ErasedExecFunction for ExecFunction<F, Args>
where
    F: SystemFunction<Args>,
{
    fn call(&mut self, world: &World) {
        self.function.call(world, &mut self.state);
    }
    fn reset(&mut self, world: &mut World) {
        self.function.reset(world, &mut self.state);
    }
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

pub trait SystemFunction<Args> {
    type State;

    fn init() -> Self::State;
    fn call<'w>(&self, world: &'w World, state: &'w mut Self::State);
    fn reset(&self, world: &mut World, state: &mut Self::State);
}

pub trait SystemParam {
    type State;
    type Param<'w>;
    fn init_state() -> Self::State;
    fn extract_params<'w>(world: &'w World, state: &'w mut Self::State) -> Self::Param<'w>;
    fn reset(world: &mut World, state: &mut Self::State);
}

pub struct InSet<F> {
    function: F,
    set_id: SetID,
}

// INTO SYSTEM CONFIG

pub trait IntoSystemConfig<Marker> {
    type Args;
    type Func: SystemFunction<Self::Args>;
    fn into_config(self) -> (Self::Func, SetID);
}

// these are so the implementations are different
pub struct InsetMarker;
pub struct FunctionMarker;

impl<F, Args> IntoSystemConfig<(InsetMarker, Args)> for InSet<F>
where
    F: SystemFunction<Args>,
{
    type Args = Args;
    type Func = F;
    fn into_config(self) -> (Self::Func, SetID) {
        (self.function, self.set_id)
    }
}

impl<F, Args> IntoSystemConfig<(FunctionMarker, Args)> for F
where
    F: SystemFunction<Args>,
{
    type Args = Args;
    type Func = F;
    fn into_config(self) -> (F, SetID) {
        (self, DefaultSet.set_id())
    }
}

// System Function Extras
pub trait SystemFunctionExt<Args>: SystemFunction<Args> + Sized {
    fn in_set(self, system_set: impl SystemSet) -> InSet<Self> {
        InSet {
            function: self,
            set_id: system_set.set_id(),
        }
    }
}
impl<F, Args> SystemFunctionExt<Args> for F where F: SystemFunction<Args> {}

macro_rules! impl_for_system_function {
    ($($param:ident),*) => {
        impl<T, $($param),*> $crate::systems::system::SystemFunction<($($param,)*)> for T
        where
            T: Fn($($param),*) + for<'w> Fn($(<$param as $crate::systems::system::SystemParam>::Param<'w>),*),
            $($param: $crate::systems::system::SystemParam),*
        {

            type State = ($($param::State, )*);

            fn init() -> Self::State {
                ($($param::init_state(), )*)
            }

            fn call<'w>(&self, world: &'w $crate::World, state: &'w mut Self::State) {
                let ($($param,)*) = state;
                (self)($($param::extract_params(world, $param)),*)
            }


            fn reset(&self,  world: &mut World,  state: &mut Self::State) {
                let ($($param,)*) = state;
                $($param::reset(world, $param); )*
            }

        }
    };
}

impl_for_system_function!(A);
impl_for_system_function!(A, B);
impl_for_system_function!(A, B, C);
impl_for_system_function!(A, B, C, D);
impl_for_system_function!(A, B, C, D, E);
impl_for_system_function!(A, B, C, D, E, F);
