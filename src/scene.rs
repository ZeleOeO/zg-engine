use std::rc::Rc;

use bytemuck::{Pod, Zeroable};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, BufferUsages,
    RenderPipeline,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::{
    camera::camera::CameraUniform,
    core::gpu::GPU,
    math::{
        Mat4, Vec3, compute_normal_matrix, mat4_identity, mat4_transpose, vec3_translation_matrix,
    },
    resources::{
        material::{Material, MaterialType},
        mesh::Mesh,
        texture::CustomTexture,
    },
};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Mat3Padded {
    col0: [f32; 3],
    _pad0: f32,
    col1: [f32; 3],
    _pad1: f32,
    col2: [f32; 3],
    _pad2: f32,
}

fn to_padded(m: [[f32; 3]; 3]) -> Mat3Padded {
    Mat3Padded {
        col0: [m[0][0], m[1][0], m[2][0]],
        _pad0: 0.0,
        col1: [m[0][1], m[1][1], m[2][1]],
        _pad1: 0.0,
        col2: [m[0][2], m[1][2], m[2][2]],
        _pad2: 0.0,
    }
}
#[derive(Clone, Pod, Copy, Zeroable)]
#[repr(C)]
pub struct ItemUniform {
    translation: Mat4,
    normal_matrix: Mat3Padded,
}

#[derive(Clone, Pod, Copy, Zeroable)]
#[repr(C)]
pub struct LightUniform {
    position: Vec3,
    _padding: f32,
    color: Vec3,
    _padding2: f32,
}

#[derive(Clone)]
pub struct Object {
    pub pipeline: Rc<RenderPipeline>,
    pub mesh: Rc<Mesh>,
    pub material_type: MaterialType,
    pub item_uniform: ItemUniform,
    pub position: Vec3,
    item_uniform_buffer: Buffer,
    pub transform_bind_group: BindGroup,
    pub is_light: bool,
}

pub struct Scene {
    pub draw_items: Vec<Object>,
    pub camera_uniform: CameraUniform,
    pub depth_texture: CustomTexture,
    pub light_buffer: Buffer,
    pub light_bind_group: BindGroup,
}

impl Scene {
    pub fn new(
        camera_uniform: CameraUniform,
        depth_texture: CustomTexture,
        draw_items: Vec<Object>,
        gpu: &GPU,
        light_uniform_layout: &BindGroupLayout,
    ) -> Self {
        let light_uniform = LightUniform {
            position: [0.0, 0.0, 0.0],
            _padding: 0.0,
            color: [0.0, 0.0, 0.0],
            _padding2: 0.0,
        };

        let light_buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Light Buffer Init"),
            contents: bytemuck::cast_slice(&[light_uniform]),
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
        });

        let bind_group = gpu.device.create_bind_group(&BindGroupDescriptor {
            label: Some("Bind Group Light Uniform"),
            entries: &[BindGroupEntry {
                binding: 0,
                resource: light_buffer.as_entire_binding(),
            }],
            layout: light_uniform_layout,
        });
        Self {
            draw_items,
            camera_uniform,
            depth_texture,
            light_buffer,
            light_bind_group: bind_group,
        }
    }

    pub fn render_light(&self, gpu: &GPU) {
        if let Some(item_light) = self.draw_items.iter().find(|item| item.is_light) {
            let light_uniform = LightUniform {
                position: item_light.position,
                _padding: 0.0,
                color: item_light.get_color(),
                _padding2: 0.0,
            };
            gpu.queue.write_buffer(
                &self.light_buffer,
                0,
                bytemuck::cast_slice(&[light_uniform]),
            );
        } else {
            println!("No light found");
        }
    }
}

impl Object {
    pub fn new_with_texture(
        gpu: &GPU,
        layout: &BindGroupLayout,
        pipeline: &Rc<RenderPipeline>,
        material: &Rc<Material>,
        mesh: &Rc<Mesh>,
    ) -> Self {
        let item_uniform = ItemUniform {
            translation: mat4_identity(),
            normal_matrix: to_padded(compute_normal_matrix(mat4_identity())),
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
            item_uniform_buffer: uniform_buffer,
            item_uniform,
            transform_bind_group,
            is_light: false,
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
            normal_matrix: to_padded(compute_normal_matrix(mat4_identity())),
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
            item_uniform_buffer: uniform_buffer,
            item_uniform,
            transform_bind_group,
            is_light: false,
        }
    }

    pub fn translate(mut self, translation: &Vec3, gpu: &GPU) -> Self {
        self.position = *translation;
        self.item_uniform = ItemUniform {
            translation: mat4_transpose(vec3_translation_matrix(*translation)),
            normal_matrix: to_padded(compute_normal_matrix(vec3_translation_matrix(*translation))),
        };
        gpu.queue.write_buffer(
            &self.item_uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.item_uniform]),
        );
        self
    }

    pub fn is_light(mut self, light_pipeline: &Rc<RenderPipeline>) -> Self {
        self.pipeline = Rc::clone(light_pipeline);
        self.is_light = true;
        self
    }

    pub fn get_color(&self) -> Vec3 {
        match self.material_type {
            MaterialType::NonTexture {
                color,
                uniform_buffer_bind_group: _,
            } => {
                return color;
            }
            _ => return [0.0, 0.0, 0.0],
        }
    }
}
