use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferUsages, Device,
    Queue, RenderPipeline,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    camera::camera::CameraUniform,
    math::{Mat4, Vec3, mat4_identity, mat4_transpose, vec3_translation_matrix},
    resources::{
        material::{Material, MaterialType},
        mesh::Mesh,
        texture::CustomTexture,
    },
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
    pub material_type: MaterialType,
    pub item_uniform: ItemUniform,
    item_uniform_buffer: Buffer,
    pub transform_bind_group: BindGroup,
    pub is_light: bool,
}

pub struct Scene {
    pub draw_items: Vec<DrawItem>,
    pub camera_uniform: CameraUniform,
    pub depth_texture: CustomTexture,
}

impl DrawItem {
    pub fn new_with_texture(
        device: &Device,
        layout: &BindGroupLayout,
        pipeline: &Rc<RenderPipeline>,
        material: &Rc<Material>,
        mesh: &Rc<Mesh>,
    ) -> Self {
        let item_uniform = ItemUniform {
            translation: mat4_identity(),
        };
        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Draw Item Buffer Transform"),
            contents: bytemuck::cast_slice(&[item_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let transform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Draw Item Bind Group"),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            layout: layout,
        });

        Self {
            pipeline: Rc::clone(&pipeline),
            mesh: Rc::clone(&mesh),
            material_type: MaterialType::Textured {
                texture: material.texture.clone().unwrap(),
                uniform_buffer_bind_group: material.uniform_buffer_bind_group.clone(),
            },
            item_uniform_buffer: uniform_buffer,
            item_uniform,
            transform_bind_group,
            is_light: false,
        }
    }

    pub fn new_with_color(
        device: &Device,
        layout: &BindGroupLayout,
        pipeline: &Rc<RenderPipeline>,
        material: Material,
        mesh: &Rc<Mesh>,
    ) -> Self {
        let item_uniform = ItemUniform {
            translation: mat4_identity(),
        };

        let uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Draw Item Buffer Transform"),
            contents: bytemuck::cast_slice(&[item_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let transform_bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Draw Item Bind Group"),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            layout: layout,
        });

        Self {
            pipeline: Rc::clone(&pipeline),
            mesh: Rc::clone(&mesh),
            material_type: MaterialType::NonTexture {
                color: material.color.clone().unwrap(),
                uniform_buffer_bind_group: material.uniform_buffer_bind_group,
            },
            item_uniform_buffer: uniform_buffer,
            item_uniform,
            transform_bind_group,
            is_light: false,
        }
    }

    pub fn translate(mut self, translation: &Vec3, queue: &Queue) -> Self {
        self.item_uniform = ItemUniform {
            translation: mat4_transpose(vec3_translation_matrix(*translation)),
        };
        queue.write_buffer(
            &self.item_uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.item_uniform]),
        );
        self
    }

    pub fn is_light(mut self) -> Self {
        self.is_light = true;
        self
    }
}
