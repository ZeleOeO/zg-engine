#[derive(Hash, PartialEq, Eq)]
pub enum BindGroupResourceType {
    Buffer { buffer: wgpu::Buffer },
    Texture { texture_view: wgpu::TextureView },
    Sampler { sampler: wgpu::Sampler },
}

#[derive(Hash, PartialEq, Eq)]
pub struct BindGroupCacheKey {
    pub layout_num: u32,
    pub entries: Vec<(u32, BindGroupResourceType)>,
}
