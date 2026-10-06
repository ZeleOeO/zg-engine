use std::{
    any::Any,
    cell::{Ref, RefMut},
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{SystemContext, systems::SystemParam};

pub trait Resource: 'static + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

// impl<T: 'static + Debug> Resource for T {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//
//     fn as_any_mut(&mut self) -> &mut dyn Any {
//         self
//     }
// }

#[derive(Debug)]
pub struct Res<'a, R: Resource>(pub Ref<'a, R>);

#[derive(Debug)]
pub struct ResMut<'a, R: Resource>(pub RefMut<'a, R>);

impl<R: Resource> Deref for Res<'_, R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: Resource> AsRef<R> for Res<'_, R> {
    fn as_ref(&self) -> &R {
        &self.0
    }
}

impl<R: Resource> Deref for ResMut<'_, R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: Resource> DerefMut for ResMut<'_, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<R: Resource> AsMut<R> for ResMut<'_, R> {
    fn as_mut(&mut self) -> &mut R {
        &mut self.0
    }
}

impl<R: Resource> AsRef<R> for ResMut<'_, R> {
    fn as_ref(&self) -> &R {
        &self.0
    }
}

impl<R: Resource> SystemParam for ResMut<'_, R> {
    type Param<'w> = ResMut<'w, R>;
    type State = ();
    fn init_state() -> Self::State {
        ()
    }
    fn extract_params<'w>(world: &'w crate::World, _state: &'w mut Self::State) -> Self::Param<'w> {
        ResMut(world.get_resource_mut::<R>())
    }

    fn reset(_world: &mut crate::World, _state: &mut Self::State) {}
}

impl<R: Resource> SystemParam for Res<'_, R> {
    type Param<'w> = Res<'w, R>;
    type State = ();
    fn init_state() -> Self::State {
        ()
    }
    fn extract_params<'w>(world: &'w crate::World, _state: &'w mut Self::State) -> Self::Param<'w> {
        Res(world.get_resource::<R>())
    }
    fn reset(_world: &mut crate::World, _state: &mut Self::State) {}
}
