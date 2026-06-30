use wgpu::{
    CommandEncoderDescriptor, Device, Face, PipelineCompilationOptions, PipelineLayoutDescriptor,
    Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline,
    RenderPipelineDescriptor, SurfaceConfiguration, VertexState,
};
use winit::{event_loop::ActiveEventLoop, keyboard::KeyCode};

pub mod example_object;
use crate::app_graphics_engine::example_object::ExampleObject;

#[derive(Debug)]
pub struct AppGraphicsEngine {
    pub render_pipeline: RenderPipeline,
    pub example_object: ExampleObject,
}

impl AppGraphicsEngine {
    pub fn new(
        device: &Device,
        config: &SurfaceConfiguration,
    ) -> anyhow::Result<AppGraphicsEngine> {
        let example_object = ExampleObject::create_indexed_example(device);

        let shader =
            device.create_shader_module(wgpu::include_wgsl!("../shaders/uniform_shader.wgsl"));
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &[Some(&example_object.bind_group_layout)],
            immediate_size: 0,
        });
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &example_object.vertex_buffer_layout,
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

        let state = Self {
            render_pipeline,
            example_object,
        };
        Ok(state)
    }

    pub fn update(&mut self, queue: &Queue) {
        self.example_object.update(queue);
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
            render_pass.set_bind_group(0, Some(&self.example_object.bind_group), &[]);

            for (i, buffer) in self.example_object.vertex_buffer.iter().enumerate() {
                render_pass.set_vertex_buffer(i as u32, buffer.slice(..));
            }

            if self.example_object.index_buffer.is_none() {
                render_pass.draw(
                    0..self.example_object.num_to_draw,
                    0..self.example_object.instances,
                );
            } else {
                render_pass.set_index_buffer(
                    self.example_object.index_buffer.clone().unwrap()[0].slice(..),
                    wgpu::IndexFormat::Uint32,
                );
                render_pass.draw_indexed(
                    0..self.example_object.num_to_draw,
                    0,
                    0..self.example_object.instances,
                );
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
