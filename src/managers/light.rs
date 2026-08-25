use bytemuck::{Pod, Zeroable};

use crate::math::Vec3;

pub struct LightComponent {
    position: Vec3,
    color: Vec3,
}

#[derive(Clone, Pod, Copy, Zeroable)]
#[repr(C)]
pub struct LightUniform {
    position: Vec3,
    _padding: f32,
    color: Vec3,
    _padding2: f32,
}

pub struct LightManager {
    lights: Vec<LightComponent>,
}

impl LightManager {
    pub fn upload_light() {}
}
