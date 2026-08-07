use wgpu::{
    Device, LoadOp, Operations, Queue, RenderPassColorAttachment, RenderPassDepthStencilAttachment,
    RenderPassDescriptor, StoreOp, TextureView, wgt::CommandEncoderDescriptor,
};

use crate::{resources::material::MaterialType, scene::Scene};

pub fn render(device: &Device, queue: &Queue, view: &TextureView, scene: &Scene) {
    let items = &scene.draw_items;
    let camera = &scene.camera_uniform;
    let depth_texture = &scene.depth_texture;
    let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor {
        label: Some("Encoder"),
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
            depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                view: &depth_texture.view,
                depth_ops: Some(Operations {
                    load: LoadOp::Clear(1.0),
                    store: StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });

        render_pass.set_bind_group(0, &camera.bind_group, &[]);
        for item in items {
            render_pass.set_vertex_buffer(0, item.mesh.vertex_buffer.slice(..));
            render_pass.set_pipeline(&item.pipeline);
            match &item.material_type {
                MaterialType::Textured {
                    uniform_buffer_bind_group,
                    ..
                }
                | MaterialType::NonTexture {
                    uniform_buffer_bind_group,
                    ..
                } => {
                    render_pass.set_bind_group(1, uniform_buffer_bind_group, &[]);
                }
            }

            render_pass.set_bind_group(2, &item.transform_bind_group, &[]);

            if let Some(index_buffer) = &item.mesh.index_buffer {
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.draw_indexed(0..item.mesh.index_num_to_draw, 0, 0..1);
            } else {
                render_pass.draw(0..item.mesh.num_to_draw, 0..1);
            }
        }
    }
    queue.submit(Some(encoder.finish()));
}
