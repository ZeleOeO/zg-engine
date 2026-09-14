use zg_graphics::BindGroupCacheHandle;
use zg_utils::{MeshHandle, PipelineID};

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
