use anyhow::Ok;
use image::GenericImageView;
use image::{DynamicImage, ImageBuffer, Rgba};
use wgpu::{BindGroup, BindGroupLayout, Device, Extent3d, Sampler, Texture, TextureView};

use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingResource, BindingType, ShaderStages, TextureDescriptor, TextureUsages,
    TextureViewDescriptor,
};

#[derive(Debug)]
pub struct CustomTexture {
    pub texture: Texture,
    pub view: TextureView,
    pub sampler: Sampler,
    pub texture_size: Extent3d,
    pub bind_group: BindGroup,
    pub bind_group_layout: BindGroupLayout,
    pub image_rba: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl CustomTexture {
    pub fn from_bytes(device: &wgpu::Device, bytes: &[u8]) -> anyhow::Result<Self> {
        let img = image::load_from_memory(bytes)?;
        Self::from_image(device, &img)
    }

    pub fn from_image(device: &Device, image: &DynamicImage) -> anyhow::Result<Self> {
        let dimensions = image.dimensions();

        let img_rgba = image.clone().into_rgba8();

        let texture_size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some("Texture"),
            dimension: wgpu::TextureDimension::D2,
            size: texture_size,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&TextureViewDescriptor::default());
        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Binding Group Layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        count: None,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        count: None,
                        visibility: ShaderStages::FRAGMENT,
                        ty: BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    },
                ],
            });

        let texture_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &texture_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&texture_sampler),
                },
            ],
        });

        Ok(Self {
            texture,
            view: texture_view,
            sampler: texture_sampler,
            bind_group: texture_bind_group,
            texture_size,
            image_rba: img_rgba,
            bind_group_layout: texture_bind_group_layout,
        })
    }
}
