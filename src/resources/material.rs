use std::rc::Rc;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, Device, Queue, ShaderStages,
};

use crate::resources::texture::CustomTexture;

pub struct Material {
    pub texture: Rc<CustomTexture>,
    pub uniform_buffer_bind_group: BindGroup,
}

impl Material {
    pub fn new(
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
            ],
            layout: &layout,
        });

        Ok(Self {
            texture: texture,
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
                    view_dimension: wgpu::TextureViewDimension::D2,
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
        ],
    });
    bg_layouts
}
