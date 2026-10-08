use std::collections::HashMap;

use wgpu::{BindGroup, BindGroupLayout};

use crate::buffer::BindGroupCacheKey;
use crate::layout::*;
use crate::pipeline::*;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
// position, bind_group_slot
pub struct BindGroupCacheHandle(pub u32, pub u32);

#[derive(Debug)]
pub struct Cache {
    // All the layouts used
    pub(crate) layouts: Vec<BindGroupLayout>,
    // Hashmap for matching the bind group cachekey and the bind group cache handle
    pub bind_groups_cache_map: HashMap<BindGroupCacheKey, BindGroupCacheHandle>,
    pub pipelines: [wgpu::RenderPipeline; 2],
    // Dense Array of bind groups
    pub cached_bind_groups: Vec<BindGroup>,
    
    pub cached_transform_bind_groups: Vec<(wgpu::Buffer, BindGroupCacheHandle)>,
    pub transform_bind_groups_index: usize,
    
    pub cached_light_bind_groups: Vec<(wgpu::Buffer, BindGroupCacheHandle)>,
    pub light_bind_groups_index: usize,
    
    pub default_light_material: Option<BindGroupCacheHandle>,
}

impl Cache {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        let camera_layout = create_camera_layout(&device);
        let material_layout = create_material_bg_layout(&device);
        let item_uniform_layout = create_item_uniform_layout(&device);
        let light_layout = create_light_uniform_layout(&device);

        let bg_layouts = [
            Some(&camera_layout),
            Some(&material_layout),
            Some(&item_uniform_layout),
            Some(&light_layout),
        ];

        let pipelines = [
            main_pipeline(device, config, &bg_layouts).unwrap(),
            light_pipeline(device, config, &bg_layouts).unwrap(),
        ];

        let layouts = vec![
            camera_layout,
            material_layout,
            item_uniform_layout,
            light_layout,
        ];
        Cache {
            layouts,
            bind_groups_cache_map: HashMap::default(),
            cached_bind_groups: Vec::default(),
            pipelines,
            cached_transform_bind_groups: Vec::default(),
            transform_bind_groups_index: 0,
            cached_light_bind_groups: Vec::default(),
            light_bind_groups_index: 0,
            default_light_material: None,
        }
    }
}
