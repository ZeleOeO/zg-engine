use std::{any::Any, fmt::Debug};

use crate::{
    managers::{material::MaterialHandle, mesh::MeshHandle, transform::Transform},
    world::resources::Resource,
};

pub trait ComponentColumn: Any + Debug {
    fn len(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_entity(&self, row: u32) -> &dyn Any;
}

impl<T: Debug + 'static> ComponentColumn for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn get_entity(&self, row: u32) -> &dyn Any {
        let ans = &self[row as usize];
        ans.as_any()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MeshComponent(pub MeshHandle);

#[derive(Clone, Copy, Default, Debug)]
pub struct TransformComponent(pub Transform);

#[derive(Clone, Copy, Debug)]
pub struct MaterialComponent(pub MaterialHandle);
