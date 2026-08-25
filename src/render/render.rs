use wgpu::BindGroup;

use crate::managers::mesh::MeshHandle;

pub struct RenderQueue {
    pub commands: Vec<RenderCommand>,
}

pub enum RenderCommand {
    SetVertexBuffer { mesh_handle: MeshHandle },
    SetIndexBuffer { index_handle: MeshHandle },
    SetPipeline { pipeline: wgpu::RenderPipeline },
    SetBindGroup { slot: u32, bind_group: BindGroup },
}
