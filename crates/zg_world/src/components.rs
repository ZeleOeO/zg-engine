use std::{any::Any, fmt::Debug};

use crate::resources::Resource;
use zg_utils::math::Vec3;

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

pub trait ComponentColumn: Any + Debug {
    fn len(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn get_entity(&self, row: u32) -> &dyn Any;
}

#[derive(Clone, Copy, Debug)]
pub struct MaterialHandle(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct MeshHandle(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct Transform {
    pub position: Vec3,
    // rotation: Quat,
    // pub scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: [0.0, 0.0, 0.0],
        }
    }
}

impl Transform {
    pub fn new(position: Vec3) -> Self {
        Self { position }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MeshComponent(pub MeshHandle);

#[derive(Clone, Copy, Debug)]
pub struct MaterialComponent(pub MaterialHandle);
