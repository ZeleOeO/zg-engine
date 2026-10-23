use wgpu::Buffer;
use zg_graphics::{BindGroupCacheHandle, ComputePipelineID, PipelineID};
use zg_world::components::MeshHandle;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RenderLayer {
    Opaque,
    Transparent,
    Overlay, // this is for UI and others
}

// TODO: Rename to RenderItem
#[derive(Debug)]
pub struct DrawItem {
    pub layer: RenderLayer,
    pub pipeline: PipelineID,
    pub material: BindGroupCacheHandle,
    pub transform: BindGroupCacheHandle,
    pub mesh: MeshHandle,
    pub index_count: u32,
}

// for binding cameras and lights, non draw items
//
#[derive(Debug)]
pub struct FrameBinding {
    pub bind_group: BindGroupCacheHandle,
}

#[derive(Debug)]
pub struct ComputeItem {
    pub pipeline_id: ComputePipelineID,
    pub bind_group_handle: BindGroupCacheHandle,
}

#[derive(Debug)]
pub struct BufferCopy {
    pub copy_src: Buffer,
    pub copy_dst: Buffer,
}
