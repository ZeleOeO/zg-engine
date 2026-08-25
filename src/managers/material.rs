use bytemuck::{Pod, Zeroable};
use image::{DynamicImage, GenericImageView, ImageReader};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindingResource, BufferUsages,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{core::gpu::InternalGraphics, layouts::create_material_bg_layout, math::Vec3};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
struct MaterialUniform {
    color: Vec3,
    has_texture: f32,
    emmisive: f32,
    _padding: Vec3,
}

#[derive(Clone)]
pub enum MaterialType {
    Textured { location: String },
    NonTexture { color: Vec3 },
}

#[derive(Debug, Clone)]
pub struct TextureData {
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

#[derive(Clone, Copy)]
pub struct MaterialHandle(pub u32);

pub struct MaterialManager {
    material_bind_group: Vec<BindGroup>,
    layout: BindGroupLayout,
}

impl MaterialManager {
    pub fn new(gpu: &InternalGraphics) -> MaterialManager {
        let layout = create_material_bg_layout(&gpu.device);
        Self {
            material_bind_group: Vec::new(),
            layout: layout,
        }
    }

    pub fn get_material(&self, material_handle: MaterialHandle) -> &BindGroup {
        &self.material_bind_group[material_handle.0 as usize]
    }

    pub fn add_new_material(
        &mut self,
        material_type: MaterialType,
        gpu: &InternalGraphics,
    ) -> MaterialHandle {
        let bind_group = match material_type {
            MaterialType::Textured { location } => {
                let texture = Self::from_location(gpu, &location).unwrap();
                let buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
                    label: Some("Buffer Init Descriptor Matieral Color"),
                    contents: bytemuck::cast_slice(&[MaterialUniform {
                        color: [0.0, 0.0, 0.0],
                        has_texture: 1.0,
                        emmisive: 0.0,
                        _padding: [0.0, 0.0, 0.0],
                    }]),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                });
                gpu.device.create_bind_group(&BindGroupDescriptor {
                    label: Some("Texture Bind Group"),
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
                    layout: &self.layout,
                })
            }
            MaterialType::NonTexture { color } => {
                let buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
                    label: Some("Buffer Init Descriptor Matieral Color"),
                    contents: bytemuck::cast_slice(&[MaterialUniform {
                        color,
                        has_texture: 0.0,
                        emmisive: 1.0, // this is for selection in shader
                        _padding: [0.0, 0.0, 0.0],
                    }]),
                    usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                });

                let dummy_texture = &Self::create_dummy_texture(gpu);

                gpu.device.create_bind_group(&BindGroupDescriptor {
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
                    layout: &self.layout,
                })
            }
        };
        self.material_bind_group.push(bind_group);
        MaterialHandle(self.material_bind_group.len() as u32 - 1)
    }

    fn from_location(gpu: &InternalGraphics, location: &str) -> anyhow::Result<TextureData> {
        let img = ImageReader::open(location)?.decode()?;

        Self::from_image(gpu, &img)
    }

    fn from_image(gpu: &InternalGraphics, image: &DynamicImage) -> anyhow::Result<TextureData> {
        let dimensions = image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Texture"),
            dimension: wgpu::TextureDimension::D2,
            size: texture_size,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        Ok(TextureData {
            view: texture_view,
            sampler: texture_sampler,
        })
    }

    fn create_dummy_texture(gpu: &InternalGraphics) -> TextureData {
        let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Dummy Texture"),
            size: wgpu::Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = gpu.device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Dummy Sampler"),
            ..Default::default()
        });

        TextureData {
            view: texture_view,
            sampler,
        }
    }
}
