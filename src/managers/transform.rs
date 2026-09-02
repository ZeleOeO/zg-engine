use bytemuck::{Pod, Zeroable};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::{
    graphics::{cache::BindGroupCacheHandle, gpu::InternalGraphics},
    math::{Mat4, Vec3, mat4_transpose, vec3_translation_matrix},
    render::buffer::{BindGroupCacheKey, BindGroupResourceType},
};

#[derive(Clone, Pod, Copy, Zeroable)]
#[repr(C)]
pub struct ItemUniform {
    transform: Mat4,
}

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
    pub fn get_or_create_bind_group(&self, gpu: &mut InternalGraphics) -> BindGroupCacheHandle {
        let item_uniform = ItemUniform {
            transform: mat4_transpose(vec3_translation_matrix(self.position)),
        };
        let buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[item_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let cache_key = BindGroupCacheKey {
            layout_num: 2,
            entries: vec![(0, BindGroupResourceType::Buffer { buffer })],
        };
        gpu.get_or_create_bind_group(cache_key)
    }

    pub fn new(position: Vec3) -> Self {
        Self { position: position }
    }
}
