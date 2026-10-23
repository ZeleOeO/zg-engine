use wgpu::{Buffer, CommandEncoder, ComputePass};
use zg_graphics::{ComputePipelineID, InternalGraphics};
use zg_world::Resource;

use crate::{BufferCopy, ComputeItem};

#[derive(Debug, Default, Resource)]
pub struct ComputeQueue {
    compute_items: Vec<ComputeItem>,
    buffers_to_copy: Vec<BufferCopy>,
}

impl ComputeQueue {
    pub fn flush(&mut self, compute_pass: &mut ComputePass, graphics: &InternalGraphics) {
        self.compute_items
            .sort_unstable_by(|a, b| a.pipeline_id.cmp(&b.pipeline_id));
        let current_pipeline: Option<ComputePipelineID> = None;

        // then we can drain it to get the pipeline and add the stuff
        for item in self.compute_items.drain(..) {
            if Some(item.pipeline_id) != current_pipeline {
                let pipeline = graphics.get_compute_pipeline_cache(item.pipeline_id);
                compute_pass.set_pipeline(pipeline);
                let bind_group_cached = graphics.get_bind_group_by_handle(item.bind_group_handle);
                compute_pass.set_bind_group(item.bind_group_handle.1, bind_group_cached, &[]);
                compute_pass.dispatch_workgroups(2, 1, 1);
            }
        }
    }

    pub fn copy_to_copy(&mut self, encoder: &mut CommandEncoder) {
        for buffer_copy in self.buffers_to_copy.drain(..) {
            encoder.copy_buffer_to_buffer(
                &buffer_copy.copy_src,
                0,
                &buffer_copy.copy_dst,
                0,
                buffer_copy.copy_src.size(),
            );
        }
    }

    pub fn add_compute_item(&mut self, compute_item: ComputeItem) {
        self.compute_items.push(compute_item);
    }

    pub fn add_copy_to_copy_buffers(&mut self, buffer_src: Buffer, buffer_dst: Buffer) {
        self.buffers_to_copy.push(BufferCopy {
            copy_src: buffer_src,
            copy_dst: buffer_dst,
        });
    }
}
