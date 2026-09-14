use wgpu::util::{BufferInitDescriptor, DeviceExt};

use bytemuck::{Pod, Zeroable};
use zg_graphics::*;
use zg_utils::Transform;
use zg_utils::math::{Mat4, mat4_transpose, vec3_translation_matrix};

#[derive(Clone, Pod, Copy, Zeroable)]
#[repr(C)]
pub struct ItemUniform {
    transform: Mat4,
}

pub fn create_transform_bind_group(
    transform: &Transform,
    gpu: &mut InternalGraphics,
) -> BindGroupCacheHandle {
    let item_uniform = ItemUniform {
        transform: mat4_transpose(vec3_translation_matrix(transform.position)),
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
