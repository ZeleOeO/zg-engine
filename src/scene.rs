use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferUsages, Device,
    Queue, RenderPipeline,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    camera::camera::CameraUniform,
    core::gpu::GPU,
    layouts::create_light_uniform_layout,
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

#[derive(Clone, Pod, Copy, Zeroable)]
#[repr(C)]
pub struct LightUniform {
    position: Vec3,
    emmisive: f32,
    color: Vec3,
    _padding2: f32,
}

fn default_light_uniform(gpu: &GPU) -> BindGroup {
    let light_uniform = LightUniform {
        position: [0.0, 0.0, 0.0],
        color: [0.0, 0.0, 0.0],
        emmisive: 0.0,
        _padding2: 0.0,
    };

    let light_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
        label: Some("Light Uniform"),
        contents: bytemuck::cast_slice(&[light_uniform]),
        usage: BufferUsages::UNIFORM,
    });

    gpu.device.create_bind_group(&BindGroupDescriptor {
        label: None,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: light_buffer.as_entire_binding(),
        }],
        layout: &create_light_uniform_layout(&gpu.device),
    })
}

#[derive(Clone)]
pub struct DrawItem {
    pub pipeline: Rc<RenderPipeline>,
    pub mesh: Rc<Mesh>,
    pub material_type: MaterialType,
    pub item_uniform: ItemUniform,
    pub position: Vec3,
    item_uniform_buffer: Buffer,
    pub light_uniform_bind_group: BindGroup,
    pub transform_bind_group: BindGroup,
}

pub struct Scene {
    pub draw_items: Vec<DrawItem>,
    pub camera_uniform: CameraUniform,
    pub depth_texture: CustomTexture,
}

impl DrawItem {
    pub fn new_with_texture(
        gpu: &GPU,
        layout: &BindGroupLayout,
        pipeline: &Rc<RenderPipeline>,
        material: &Rc<Material>,
        mesh: &Rc<Mesh>,
    ) -> Self {
        let item_uniform = ItemUniform {
            translation: mat4_identity(),
        };
        let uniform_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Draw Item Buffer Transform"),
            contents: bytemuck::cast_slice(&[item_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let transform_bind_group = gpu.device.create_bind_group(&BindGroupDescriptor {
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
            position: [0.0, 0.0, 0.0],
            light_uniform_bind_group: default_light_uniform(gpu),
            item_uniform_buffer: uniform_buffer,
            item_uniform,
            transform_bind_group,
        }
    }

    pub fn new_with_color(
        gpu: &GPU,
        layout: &BindGroupLayout,
        pipeline: &Rc<RenderPipeline>,
        material: Material,
        mesh: &Rc<Mesh>,
    ) -> Self {
        let item_uniform = ItemUniform {
            translation: mat4_identity(),
        };

        let uniform_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Draw Item Buffer Transform"),
            contents: bytemuck::cast_slice(&[item_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let transform_bind_group = gpu.device.create_bind_group(&BindGroupDescriptor {
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
            position: [0.0, 0.0, 0.0],
            light_uniform_bind_group: default_light_uniform(gpu),
            item_uniform_buffer: uniform_buffer,
            item_uniform,
            transform_bind_group,
        }
    }

    pub fn translate(mut self, translation: &Vec3, gpu: &GPU) -> Self {
        self.position = *translation;
        self.item_uniform = ItemUniform {
            translation: mat4_transpose(vec3_translation_matrix(*translation)),
        };
        gpu.queue.write_buffer(
            &self.item_uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.item_uniform]),
        );
        self
    }

    pub fn is_light(mut self, gpu: &GPU) -> Self {
        match self.material_type {
            MaterialType::NonTexture {
                color,
                uniform_buffer_bind_group: _,
            } => {
                let light_uniform = LightUniform {
                    position: self.position,
                    color,
                    emmisive: 1.0,
                    _padding2: 0.0,
                };

                let light_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
                    label: Some("Light Uniform"),
                    contents: bytemuck::cast_slice(&[light_uniform]),
                    usage: BufferUsages::UNIFORM,
                });

                let light_bind_group = gpu.device.create_bind_group(&BindGroupDescriptor {
                    label: None,
                    entries: &[BindGroupEntry {
                        binding: 0,
                        resource: light_buffer.as_entire_binding(),
                    }],
                    layout: &create_light_uniform_layout(&gpu.device),
                });
                self.light_uniform_bind_group = light_bind_group;
            }
            _ => {}
        }
        self
    }
}
