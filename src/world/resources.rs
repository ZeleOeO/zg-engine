use std::{
    any::Any,
    cell::{Ref, RefMut},
    fmt::Debug,
    ops::{Deref, DerefMut},
};

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

pub struct ResourceRef<'a, R: Resource>(pub Ref<'a, R>);
pub struct ResourceMut<'a, R: Resource>(pub RefMut<'a, R>);

impl<R: Resource> Deref for ResourceRef<'_, R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: Resource> AsRef<R> for ResourceRef<'_, R> {
    fn as_ref(&self) -> &R {
        &self.0
    }
}

impl<R: Resource> Deref for ResourceMut<'_, R> {
    type Target = R;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<R: Resource> DerefMut for ResourceMut<'_, R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<R: Resource> AsMut<R> for ResourceMut<'_, R> {
    fn as_mut(&mut self) -> &mut R {
        &mut self.0
    }
}

impl<R: Resource> AsRef<R> for ResourceMut<'_, R> {
    fn as_ref(&self) -> &R {
        &self.0
    }
}
