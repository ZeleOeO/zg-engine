use std::collections::hash_map;
use std::sync::Arc;

use slotmap::secondary::Entry::Occupied;
use wgpu::{
    Backends, BindingResource, Device, DeviceDescriptor, ExperimentalFeatures, Features, Instance,
    Limits, RequestAdapterOptions, Surface, SurfaceConfiguration,
};
use wgpu::{Queue, TextureView};
use winit::window::Window;

use crate::core::cache::Cache;
use crate::render::buffer::{BindGroupCacheKey, BindGroupResourceType};

pub struct InternalGraphics {
    pub surface: Surface<'static>,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
    pub depth_texture_view: TextureView,
    pub cache: Cache,
}

impl InternalGraphics {
    pub async fn new(window: &Arc<Window>) -> anyhow::Result<Self> {
        let size = window.inner_size();
        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: Backends::PRIMARY,
            ..wgpu::InstanceDescriptor::new_without_display_handle()
        });

        let surface = instance.create_surface(window.clone())?;
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await?;

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                experimental_features: ExperimentalFeatures::disabled(),
                required_limits: Limits::default(),
                trace: wgpu::Trace::Off,
                memory_hints: Default::default(),
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|format| format.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        let size = wgpu::Extent3d {
            width: config.width.max(1),
            height: config.height.max(1),
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            format: wgpu::TextureFormat::Depth32Float,
            mip_level_count: 1,
            size: size,
            dimension: wgpu::TextureDimension::D2,
            sample_count: 1,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });

        let depth_texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        // let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        //     address_mode_u: wgpu::AddressMode::ClampToEdge,
        //     address_mode_v: wgpu::AddressMode::ClampToEdge,
        //     address_mode_w: wgpu::AddressMode::ClampToEdge,
        //     mag_filter: wgpu::FilterMode::Linear,
        //     min_filter: wgpu::FilterMode::Linear,
        //     mipmap_filter: wgpu::MipmapFilterMode::Nearest,
        //     compare: Some(wgpu::CompareFunction::LessEqual),
        //     ..Default::default()
        // });

        let cache = Cache::new(&device);

        surface.configure(&device, &config);
        let state = Self {
            surface,
            device,
            queue,
            config,
            depth_texture_view,
            cache,
        };
        Ok(state)
    }

    pub fn get_or_create_bind_group(&mut self, key: BindGroupCacheKey) -> Arc<wgpu::BindGroup> {
        let bind_group = match self.cache.bind_groups.entry(key) {
            hash_map::Entry::Occupied(occupied) => occupied.get().clone(),
            hash_map::Entry::Vacant(vacant) => {
                let key_ref = vacant.key();
                let layout = &self.cache.layouts[key_ref.layout_num as usize];
                let entries = key_ref
                    .entries
                    .iter()
                    .map(|(binding, resource_type)| {
                        let resource = match resource_type {
                            BindGroupResourceType::Buffer { buffer } => {
                                BindingResource::Buffer(buffer.as_entire_buffer_binding())
                            }
                            BindGroupResourceType::Texture { texture_view } => {
                                BindingResource::TextureView(&texture_view)
                            }
                            BindGroupResourceType::Sampler { sampler } => {
                                BindingResource::Sampler(&sampler)
                            }
                        };
                        wgpu::BindGroupEntry {
                            binding: *binding,
                            resource,
                        }
                    })
                    .collect::<Vec<_>>();
                let raw_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout,
                    entries: &entries,
                });
                let saved_bind_group = Arc::new(raw_bind_group);
                vacant.insert(saved_bind_group.clone());
                saved_bind_group
            }
        };
        bind_group
    }

    pub fn stuff() {}
}
