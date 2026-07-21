use std::rc::Rc;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, Buffer, BufferAddress, BufferDescriptor, BufferUsages,
    Device, Queue, ShaderStages,
};

use crate::resources::texture::CustomTexture;

#[derive(Clone)]
pub struct Uniform {
    pub rotation: f32,
}

pub struct Material {
    pub texture: Rc<CustomTexture>,
    pub uniform_buffer: Buffer,
    pub uniform_buffer_bind_group: BindGroup,
}

impl Material {
    pub fn new(
        device: &Device,
        queue: &Queue,
        layout: &BindGroupLayout,
        texture_location: &str,
    ) -> anyhow::Result<Self> {
        let texture =
            Rc::new(CustomTexture::from_location(device, queue, texture_location).unwrap());

        let uniform_buffer = device.create_buffer(&BufferDescriptor {
            label: Some("Shape Uniform Buffer"),
            size: std::mem::size_of::<Uniform>() as BufferAddress,
            mapped_at_creation: false,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let uniform_buffer_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Uniform Buffer Bind Group"),
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
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
            layout: &layout,
        });

        Ok(Self {
            texture: texture,
            uniform_buffer,
            uniform_buffer_bind_group,
        })
    }
}

pub fn create_material_bg_layout(device: &Device) -> BindGroupLayout {
    let bg_layouts = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Bind Group Layout"),
        entries: &[
            BindGroupLayoutEntry {
                binding: 0,
                count: None,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D3,
                    multisampled: false,
                },
                visibility: ShaderStages::FRAGMENT,
            },
            BindGroupLayoutEntry {
                binding: 1,
                count: None,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                visibility: ShaderStages::FRAGMENT,
            },
            BindGroupLayoutEntry {
                binding: 2,
                count: None,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                visibility: ShaderStages::VERTEX,
            },
        ],
    });
    bg_layouts
}
