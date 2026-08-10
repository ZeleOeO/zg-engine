use bytemuck::{Pod, Zeroable};
use std::rc::Rc;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, BufferUsages,
    Device, Queue,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{math::Vec3, resources::texture::CustomTexture};

#[derive(Clone)]
pub enum MaterialType {
    Textured {
        texture: Rc<CustomTexture>,
        uniform_buffer_bind_group: BindGroup,
    },
    NonTexture {
        color: Vec3,
        uniform_buffer_bind_group: BindGroup,
    },
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct MaterialUniform {
    color: Vec3,
    has_texture: f32,
    emmisive: f32,
    _padding: Vec3,
}

#[derive(Clone)]
pub struct Material {
    pub texture: Option<Rc<CustomTexture>>,
    pub color: Option<Vec3>,
    pub uniform_buffer_bind_group: BindGroup,
}

impl Material {
    pub fn new_texture(
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
        texture_location: &str,
    ) -> anyhow::Result<Self> {
        let texture = Rc::new(CustomTexture::from_location(
            device,
            queue,
            texture_location,
        )?);

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Buffer Init Descriptor Matieral Color"),
            contents: bytemuck::cast_slice(&[MaterialUniform {
                color: [0.0, 0.0, 0.0],
                has_texture: 1.0,
                emmisive: 0.0,
                _padding: [0.0, 0.0, 0.0],
            }]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let uniform_buffer_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uniform Buffer Bind Group Texture Material"),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&texture.sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Buffer(buffer.as_entire_buffer_binding()),
                },
            ],
            layout: &layout,
        });

        Ok(Self {
            texture: Some(texture),
            color: None,
            uniform_buffer_bind_group,
        })
    }

    pub fn new_color(
        device: &Device,
        layout: &BindGroupLayout,
        color: Vec3,
        emmisive: f32,
    ) -> anyhow::Result<Self> {
        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Buffer Init Descriptor Matieral Color"),
            contents: bytemuck::cast_slice(&[MaterialUniform {
                color,
                has_texture: 0.0,
                emmisive,
                _padding: [0.0, 0.0, 0.0],
            }]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let dummy_texture = &CustomTexture::create_dummy_texture(device);

        let uniform_buffer_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uniform Buffer Bind Group Texture Material"),
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&dummy_texture.view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&dummy_texture.sampler),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Buffer(buffer.as_entire_buffer_binding()),
                },
            ],
            layout: &layout,
        });
        Ok(Self {
            texture: None,
            color: Some(color),
            uniform_buffer_bind_group,
        })
    }
}
