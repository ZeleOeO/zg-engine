use std::{
    any::Any,
    cell::{Ref, RefMut},
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::{World, systems::SystemParam};

pub trait Resource: 'static + Debug {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static + Debug> Resource for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

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
    type Item<'w> = ResMut<'w, R>;
    fn extract_world_context<'w>(context: &'w World) -> Self::Item<'w> {
        context.get_mut::<R>()
    }
}

impl<R: Resource> SystemParam for Res<'_, R> {
    type Item<'w> = Res<'w, R>;
    fn extract_world_context<'w>(context: &'w World) -> Self::Item<'w> {
        context.get::<R>()
    }
}
