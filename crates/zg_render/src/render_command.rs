use zg_graphics::{BindGroupCacheHandle, PipelineID};
use zg_world::components::MeshHandle;

#[derive(Debug)]
pub enum RenderCommand {
    SetVertexBuffer {
        mesh_handle: MeshHandle,
    },
    SetIndexBuffer {
        index_handle: MeshHandle,
    },
    SetPipeline {
        pipeline_id: PipelineID,
    },
    SetBindGroup {
        bind_group_handle: BindGroupCacheHandle,
    },
    DrawIndexed {
        num_to_draw: u32,
    },
}
