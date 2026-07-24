use std::f32::consts::PI;

use wgpu::{
    BindGroup, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    Buffer, BufferUsages, Device, Queue, ShaderStages,
    util::{BufferInitDescriptor, DeviceExt},
};

use crate::math::*;

// Following, albeit loosely, a tutorial
// Definition: eye is camera in world space
// Target is what I'm looking at
// Up is the hinted direction of where "up" is in world space
//
// Might change variable names later

fn look_at_matrix(eye: Vec3, target: Vec3, up: Vec3) -> Mat4 {
    let forward = vec3_normalize(vec3_sub(target, eye));
    let right = vec3_normalize(vec3_cross_product(forward, up));
    let true_up = vec3_cross_product(right, forward);

    [
        [right[0], right[1], right[2], -vec3_dot_product(right, eye)],
        [
            true_up[0],
            true_up[1],
            true_up[2],
            -vec3_dot_product(true_up, eye),
        ],
        [
            -forward[0],
            -forward[1],
            -forward[2],
            vec3_dot_product(forward, eye),
        ],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

// something further away appears smaller, cool
fn perspective_matrix(fovy: f32, aspect: f32, znear: f32, zfar: f32) -> Mat4 {
    let focal = 1.0 / (fovy * 0.5).tan();
    let depth = 1.0 / (znear - zfar);

    [
        [focal / aspect, 0.0, 0.0, 0.0],
        [0.0, -focal, 0.0, 0.0],
        [
            0.0,
            0.0,
            (zfar + znear) * depth,
            (znear * zfar * 2.0 * depth),
        ],
        [0.0, 0.0, -1.0, 0.0],
    ]
}

pub struct Camera {
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
    pub aspect: f32,
}

pub struct CameraUniform {
    pub view_proj: Mat4,
    pub bind_group: BindGroup,
    pub buffer: Buffer,
}

impl Camera {
    pub fn new(aspect: f32) -> Self {
        Self {
            eye: [0.0, 0.0, -3.0],
            target: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            fovy: PI / 4.0,
            znear: 0.1,
            zfar: 100.0,
            aspect: aspect,
        }
    }

    pub fn build_projection_matrix(&self) -> Mat4 {
        let view = look_at_matrix(self.eye, self.target, self.up);
        let perspective = perspective_matrix(self.fovy, self.aspect, self.znear, self.zfar);
        let view_proj = mat4_mul(perspective, view);

        transpose(view_proj)
    }
}

impl CameraUniform {
    pub fn new(device: &Device, camera: &Camera, camera_layout: &BindGroupLayout) -> Self {
        let view_proj = Camera::build_projection_matrix(&camera);

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera Uniform Buffer"),
            contents: bytemuck::cast_slice(&[view_proj]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Camera Bind Group"),
            layout: camera_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        Self {
            view_proj,
            bind_group,
            buffer,
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera, queue: &Queue) {
        self.view_proj = camera.build_projection_matrix();

        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.view_proj]));
    }
}

pub fn create_camera_layout(device: &Device) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        label: Some("Camera Bind Group Layout"),
        entries: &[BindGroupLayoutEntry {
            binding: 0,
            count: None,
            visibility: ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
        }],
    })
}
