use std::collections::hash_map;
use std::sync::Arc;

use wgpu::{
    Backends, BindGroup, BindingResource, Device, DeviceDescriptor, ExperimentalFeatures, Features,
    Instance, Limits, RenderPipeline, RequestAdapterOptions, Surface, SurfaceConfiguration,
};
use wgpu::{Queue, TextureView};
use winit::window::Window;

use crate::graphics::cache::{BindGroupCacheHandle, Cache};
use crate::pipeline::pipeline_id::PipelineID;
use crate::render::buffer::{BindGroupCacheKey, BindGroupResourceType};
use crate::render::render_queue::RenderQueue;
use crate::world::world::World;

#[derive(Debug)]
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

        let cache = Cache::new(&device, &config);

        surface.configure(&device, &config);
        let graphics = Self {
            surface,
            device,
            queue,
            config,
            depth_texture_view,
            cache,
        };
        Ok(graphics)
    }

    pub fn get_or_create_bind_group(&mut self, key: BindGroupCacheKey) -> BindGroupCacheHandle {
        let bind_group = match self.cache.bind_groups_cache_map.entry(key) {
            hash_map::Entry::Occupied(occupied) => *occupied.get(),
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

                let index = self.cache.cached_bind_groups.len() as u32;
                self.cache.cached_bind_groups.push(raw_bind_group);
                let handle = BindGroupCacheHandle(index, key_ref.layout_num);
                vacant.insert(handle);

                handle
            }
        };
        bind_group
    }

    pub fn get_bind_group_by_handle(&self, handle: BindGroupCacheHandle) -> &BindGroup {
        &self.cache.cached_bind_groups[handle.0 as usize]
    }

    pub fn get_pipeline_cache(&self, pipeline_id: PipelineID) -> &RenderPipeline {
        &self.cache.pipelines[pipeline_id as usize]
    }

    pub fn execute(&mut self, world: &mut World, surface_view: &TextureView) {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Encoder"),
            });
        let mut render_queue = world.get_mut::<RenderQueue>();
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: surface_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_queue.flush(&mut render_pass, world);
        }
        self.queue.submit(Some(encoder.finish()));
    }
}
