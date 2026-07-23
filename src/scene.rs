use std::rc::Rc;

use wgpu::{
    BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferUsages, Device, RenderPipeline,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    camera::camera::CameraUniform,
    resources::{material::Material, mesh::Mesh},
};

#[derive(Clone)]
pub struct ItemUniform {
    transform: [f32; 4],
}

#[derive(Clone)]
pub struct DrawItem {
    pub pipeline: Rc<RenderPipeline>,
    pub mesh: Rc<Mesh>,
    pub material: Rc<Material>,
    // pub item_uniform: ItemUniform,
}

pub struct Scene {
    pub draw_items: Vec<DrawItem>,
    pub camera_uniform: CameraUniform,
}

impl DrawItem {
    pub fn new(
        &self,
        device: &Device,
        transform: &[f32; 4],
        layout: &BindGroupLayout,
        pipeline: &Rc<RenderPipeline>,
        material: &Rc<Material>,
        mesh: &Rc<Mesh>,
    ) -> Self {
        let item_uniform = ItemUniform {
            transform: *transform,
        };
        let uniform = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Draw Item Buffer Descriptor"),
            contents: bytemuck::cast_slice(transform),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Draw Item Bind Group"),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform.as_entire_binding(),
            }],
            layout: layout,
        });

        Self {
            pipeline: Rc::clone(&pipeline),
            mesh: Rc::clone(&mesh),
            material: Rc::clone(&material),
            // item_uniform,
        }
    }
}
