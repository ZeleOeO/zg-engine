use std::f32::consts::PI;

use wgpu::{BindGroup, Buffer};

use crate::math::*;

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

    pub fn build_projection_matrix(&self) -> Mat4 {
        let view = Self::look_at_matrix(self.eye, self.target, self.up);
        let perspective = Self::perspective_matrix(self.fovy, self.aspect, self.znear, self.zfar);
        let view_proj = mat4_mul(perspective, view);

        mat4_transpose(view_proj)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            eye: [0.0, 0.0, -3.0],
            target: [0.0, 0.0, 0.0],
            up: [0.0, 1.0, 0.0],
            fovy: PI / 4.0,
            znear: 0.1,
            zfar: 100.0,
            aspect: 16.0 / 9.0,
        }
    }
}
