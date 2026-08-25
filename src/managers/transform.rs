use wgpu::Buffer;

use crate::math::Vec3;

#[derive(Debug)]
pub struct TransformBuffer {
    buffer: Buffer,
    capacity: u32,
}

#[derive(Clone, Copy)]
pub struct Transform {
    position: Vec3,
    // rotation: Quat,
    scale: Vec3,
}

impl TransformBuffer {
    pub fn create_bind_group(transform: Transform) {}
}
