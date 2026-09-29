use bytemuck::{Pod, Zeroable};

use wgpu::util::{BufferInitDescriptor, DeviceExt};
use zg_graphics::{
    BindGroupCacheHandle, BindGroupCacheKey, BindGroupResourceType, InternalGraphics,
};
use zg_utils::math::Vec3;

use crate::material::{MaterialManager, MaterialUniform};

#[derive(Debug)]
pub struct PointLight {
    pub position: Vec3,
    pub color: Vec3,
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

pub fn get_or_create_default_light_material(gpu: &mut InternalGraphics) -> BindGroupCacheHandle {
    let buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Buffer Init Descriptor Matieral Color"),
        contents: bytemuck::cast_slice(&[MaterialUniform {
            color: [1.0, 1.0, 1.0],
            has_texture: 0.0,
        }]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let dummy_texture = MaterialManager::create_dummy_texture(gpu);

    let cache_key = BindGroupCacheKey {
        layout_num: 1,
        entries: vec![
            (2, BindGroupResourceType::Buffer { buffer }),
            (
                1,
                BindGroupResourceType::Sampler {
                    sampler: dummy_texture.sampler,
                },
            ),
            (
                0,
                BindGroupResourceType::Texture {
                    texture_view: dummy_texture.view,
                },
            ),
        ],
    };

    gpu.get_or_create_bind_group(cache_key)
}
