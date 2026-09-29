use zg_graphics::{BindGroupCacheHandle, PipelineID};
use zg_world::components::MeshHandle;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum RenderLayer {
    Opaque,
    Transparent,
    Overlay, // this is for UI and others
}

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
