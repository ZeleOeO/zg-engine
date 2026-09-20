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
    position: Vec3,
    rotation: Vec3,
    scale: Vec3,
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Transform {
    pub const IDENTITY: Self = Self {
        position: [0.0, 0.0, 0.0],
        rotation: [0.0, 0.0, 0.0],
        scale: [1.0, 1.0, 1.0],
    };
    pub fn from_translation(x: f32, y: f32, z: f32) -> Self {
        let position = [x, y, z];
        Self {
            position,
            ..Default::default()
        }
    }
    pub fn with_scale(mut self, x_scale: f32, y_scale: f32, z_scale: f32) -> Self {
        self.scale = [x_scale, y_scale, z_scale];
        self
    }
    pub fn position(&self) -> Vec3 {
        self.position
    }
    pub fn scale(&self) -> Vec3 {
        self.scale
    }
    pub fn rotation(&self) -> Vec3 {
        self.rotation
    }
}

#[derive(Clone, Copy, Debug)]
pub struct MeshComponent(pub MeshHandle);

#[derive(Clone, Copy, Debug)]
pub struct MaterialComponent(pub MaterialHandle);
