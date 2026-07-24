use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BufferUsages, Device,
    RenderPipeline,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    camera::camera::CameraUniform,
    math::{Mat4, Vec3, mat4_transpose, vec3_translation_matrix},
    resources::{material::Material, mesh::Mesh},
};

#[derive(Clone, Pod, Copy, Zeroable)]
#[repr(C)]
pub struct ItemUniform {
    translation: Mat4,
}

#[derive(Clone)]
pub struct DrawItem {
    pub pipeline: Rc<RenderPipeline>,
    pub mesh: Rc<Mesh>,
    pub material: Rc<Material>,
    pub item_uniform: ItemUniform,
    pub bind_group: BindGroup,
}

pub struct Scene {
    pub draw_items: Vec<DrawItem>,
    pub camera_uniform: CameraUniform,
}

impl DrawItem {
    pub fn new(
        device: &Device,
        translation: &Vec3,
        layout: &BindGroupLayout,
        pipeline: &Rc<RenderPipeline>,
        material: &Rc<Material>,
        mesh: &Rc<Mesh>,
    ) -> Self {
        let item_uniform = ItemUniform {
            translation: mat4_transpose(vec3_translation_matrix(*translation)),
        };
        let uniform = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Draw Item Buffer Transform"),
            contents: bytemuck::cast_slice(&[item_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
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
            item_uniform,
            bind_group,
        }
    }
}
