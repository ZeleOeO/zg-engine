use zg_managers::Assets;

use zg_graphics::{BindGroupCacheHandle, InternalGraphics, PipelineID};
use zg_world::{Resource, components::MeshHandle};

use crate::render_command::{DrawItem, FrameBinding};

#[derive(Default, Debug, Resource)]
pub struct RenderQueue {
    pub frame_binding: Vec<FrameBinding>,
    pub draw_items: Vec<DrawItem>,
}

impl RenderQueue {
    pub fn new() -> Self {
        Self {
            frame_binding: Vec::new(),
            draw_items: Vec::new(),
        }
    }

    pub fn flush(
        &mut self,
        render_pass: &mut wgpu::RenderPass,
        assets: &Assets,
        graphics: &InternalGraphics,
    ) {
        for binding in self.frame_binding.drain(..) {
            let bind_group_cached = graphics.get_bind_group_by_handle(binding.bind_group);
            render_pass.set_bind_group(binding.bind_group.1, bind_group_cached, &[]);
        }

        self.draw_items.sort_unstable_by(|a, b| {
            a.layer.cmp(&b.layer).then_with(|| match a.layer {
                _ => a
                    .pipeline
                    .cmp(&b.pipeline)
                    .then(a.material.cmp(&b.material)),
            })
        });

        let mut current_pipeline: Option<PipelineID> = None;
        let mut current_material: Option<BindGroupCacheHandle> = None;
        let mut current_mesh: Option<MeshHandle> = None;

        for item in self.draw_items.drain(..) {
            if Some(item.pipeline) != current_pipeline {
                let pipeline = graphics.get_pipeline_cache(item.pipeline);
                render_pass.set_pipeline(pipeline);
                current_pipeline = Some(item.pipeline);
            }

            if Some(item.material) != current_material {
                let bind_group_cached = graphics.get_bind_group_by_handle(item.material);
                render_pass.set_bind_group(item.material.1, bind_group_cached, &[]);
                current_material = Some(item.material);
            }

            if Some(item.mesh) != current_mesh {
                let vertex_buffer = assets.mesh_manager.get_vertex_buffers(item.mesh.0);
                let index_buffer = assets.mesh_manager.get_index_buffers(item.mesh.0);
                render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                current_mesh = Some(item.mesh);
            }

            let bind_group_cached = graphics.get_bind_group_by_handle(item.transform);
            render_pass.set_bind_group(item.transform.1, bind_group_cached, &[]);

            render_pass.draw_indexed(0..item.index_count, 0, 0..1);
        }
    }
}
