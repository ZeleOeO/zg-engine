use bytemuck::{Pod, Zeroable};
use wgpu::{
    BufferAddress, CommandEncoderDescriptor, Device, Face, Origin3d, PipelineCompilationOptions,
    PipelineLayoutDescriptor, Queue, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipeline, RenderPipelineDescriptor, SurfaceConfiguration, TexelCopyBufferLayout,
    TexelCopyTextureInfo, TextureAspect, VertexAttribute, VertexBufferLayout, VertexFormat,
    VertexState,
};
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode};

use std::mem;

use crate::renderer::resources::{mesh::Mesh, texture::CustomTexture};
pub mod context;
pub mod resources;

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}

impl Vertex {
    fn desc() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    shader_location: 0,
                    offset: 0,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: mem::size_of::<[f32; 3]>() as BufferAddress,
                    shader_location: 1,
                },
            ],
        }
    }
}

#[derive(Debug)]
pub struct AppGraphicsEngine {
    pub render_pipeline: RenderPipeline,
    pub texture: CustomTexture,
    pub shapes: Mesh,
    pub frame: f32,
}

impl AppGraphicsEngine {
    pub fn new(
        queue: &wgpu::Queue,
        device: &Device,
        config: &SurfaceConfiguration,
    ) -> anyhow::Result<AppGraphicsEngine> {
        let custom_texture =
            CustomTexture::from_bytes(device, include_bytes!("../happy-tree.png"))?;

        let shader = device.create_shader_module(wgpu::include_wgsl!("../shaders/shader.wgsl"));

        let shapes = Mesh::cube(device);

        // Which means this layout is only for our texure
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &[
                shapes.uniform_buffer_bind_group_layout.as_ref(),
                Some(&custom_texture.bind_group_layout),
            ],
            immediate_size: 0,
        });

        // Layout is only for texture render
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[Vertex::desc()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview_mask: None,
            cache: None,
        });

        queue.write_texture(
            TexelCopyTextureInfo {
                texture: &custom_texture.texture,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &custom_texture.image_rba,
            TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(custom_texture.image_rba.dimensions().0 * 4),
                rows_per_image: Some(custom_texture.image_rba.dimensions().1),
            },
            custom_texture.texture_size,
        );

        let state = Self {
            render_pipeline,
            texture: custom_texture,
            shapes,
            frame: 0.0,
        };
        Ok(state)
    }

    pub fn update(&mut self, queue: &Queue) {
        self.frame += 1.0;
        queue.write_buffer(
            &self.shapes.uniform_buffer.as_ref().unwrap(),
            0,
            bytemuck::bytes_of(&self.frame),
        );
    }

    pub fn render(&mut self, queue: &wgpu::Queue, device: &wgpu::Device, view: &wgpu::TextureView) {
        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
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
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.shapes.vertex_buffer.slice(..));
            if let Some(bind_group) = &self.shapes.uniform_buffer_bind_group {
                render_pass.set_bind_group(0, bind_group, &[]);
            }
            render_pass.set_bind_group(1, Some(&self.texture.bind_group), &[]);
            if self.shapes.index_buffer.is_some() {
                render_pass.set_index_buffer(
                    self.shapes.index_buffer.as_ref().unwrap().slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(0..self.shapes.num_to_draw, 0, 0..1);
            } else {
                render_pass.draw(0..self.shapes.num_to_draw, 0..1);
            }
        }

        queue.submit(Some(encoder.finish()));
    }

    pub fn handle_key(&self, code: KeyCode, is_pressed: bool, event_loop: &ActiveEventLoop) {
        match (code, is_pressed) {
            (KeyCode::Escape, true) => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}
