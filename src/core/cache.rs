use std::{collections::HashMap, sync::Arc};

use wgpu::{BindGroup, BindGroupLayout};

use crate::{layouts::*, render::buffer::BindGroupCacheKey};

pub struct Cache {
    pub layouts: Vec<BindGroupLayout>,
    pub bind_groups: HashMap<BindGroupCacheKey, Arc<BindGroup>>,
}

impl Cache {
    pub fn new(device: &wgpu::Device) -> Self {
        let camera_layout = create_camera_layout(&device);
        let material_layout = create_material_bg_layout(&device);
        let item_uniform_layout = create_item_uniform_layout(&device);
        let light_uniform_layout = create_light_uniform_layout(&device);
        let layouts = vec![
            camera_layout,
            material_layout,
            item_uniform_layout,
            light_uniform_layout,
        ];
        Cache {
            layouts,
            bind_groups: HashMap::default(),
        }
    }
}
