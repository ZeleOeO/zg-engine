use bytemuck::{Pod, Zeroable};

use wgpu::util::{BufferInitDescriptor, DeviceExt};
use zg_graphics::{
    BindGroupCacheHandle, BindGroupCacheKey, BindGroupResourceType, InternalGraphics,
};
use zg_utils::math::Vec3;

#[derive(Debug)]
pub struct PointLight {
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

impl PointLight {
    pub fn new(position: Vec3, color: Vec3) -> PointLight {
        PointLight { position, color }
    }
}

pub fn create_light_bind_group(
    gpu: &mut InternalGraphics,
    light: &PointLight,
) -> BindGroupCacheHandle {
    let light_uniform = LightUniform {
        position: light.position,
        color: light.color,
        _padding: 0.0,
        _padding2: 0.0,
    };

    let light_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[light_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let cache_key = BindGroupCacheKey {
        layout_num: 3,
        entries: vec![(
            0,
            BindGroupResourceType::Buffer {
                buffer: light_buffer,
            },
        )],
    };

    gpu.get_or_create_bind_group(cache_key)
}
