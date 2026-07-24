use wgpu::{
    BindGroupLayout, Device, Face, PipelineCompilationOptions, PipelineLayoutDescriptor,
    RenderPipeline, RenderPipelineDescriptor, ShaderModule, SurfaceConfiguration, VertexState,
};

use crate::resources::mesh::Vertex;

pub fn opaque_pipeline(
    device: &Device,
    config: &SurfaceConfiguration,
    bg_layouts: &[Option<&BindGroupLayout>],
    shader: &ShaderModule,
) -> anyhow::Result<RenderPipeline> {
    let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label: Some("Opaque Pipeline Layout"),
        bind_group_layouts: bg_layouts,
        immediate_size: 0,
    });

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
            ..Default::default()
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

    Ok(render_pipeline)
}
