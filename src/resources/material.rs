use std::rc::Rc;
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, Device, Queue,
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
