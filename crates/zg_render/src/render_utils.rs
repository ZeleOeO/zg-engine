use wgpu::util::{BufferInitDescriptor, DeviceExt};

use bytemuck::{Pod, Zeroable};
use zg_graphics::*;
use zg_utils::math::{
    Mat4, Vec3, invert_mat3, mat4_mul, mat4_to_mat3, mat4_transpose, vec3_general_rotation_matrix,
    vec3_scaling_matrix, vec3_translation_matrix,
};
use zg_world::components::Transform;

use crate::WorldRenderer;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct ItemUniform {
    transform: [[f32; 4]; 4],
    normal: [[f32; 4]; 3],
}

pub fn create_transform_bind_group(
    transform: &Transform,
    gpu: &mut InternalGraphics,
) -> BindGroupCacheHandle {
    let matrix = {
        let scale_matrix = vec3_scaling_matrix(transform.scale());
        let rotation_matrix = vec3_general_rotation_matrix(transform.rotation());
        let translation_matrix = vec3_translation_matrix(transform.position());

        mat4_mul(translation_matrix, mat4_mul(rotation_matrix, scale_matrix))
    };

    let normal = invert_mat3(mat4_to_mat3(matrix)).unwrap();
    let item_uniform = ItemUniform {
        transform: mat4_transpose(matrix),
        normal: [
            [normal[0][0], normal[0][1], normal[0][2], 0.0],
            [normal[1][0], normal[1][1], normal[1][2], 0.0],
            [normal[2][0], normal[2][1], normal[2][2], 0.0],
        ],
    };
    let buffer = gpu.device.create_buffer_init(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(&[item_uniform]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    let cache_key = BindGroupCacheKey {
        layout_num: 2,
        entries: vec![(0, BindGroupResourceType::Buffer { buffer })],
    };
    gpu.get_or_create_bind_group(cache_key)
}

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy)]
pub(crate) struct CameraUniform {
    projection_matrix: Mat4,
    position: Vec3,
    _padding: f32,
}

pub fn create_camera_bind_group(
    projection: Mat4,
    position: Vec3,
    gpu: &mut InternalGraphics,
    renderer: &WorldRenderer,
) -> BindGroupCacheHandle {
    let camera_uniform = CameraUniform {
        projection_matrix: projection,
        position: position,
        _padding: 0.0,
    };

    gpu.queue.write_buffer(
        renderer.camera_buffer(),
        0,
        bytemuck::cast_slice(&[camera_uniform]),
    );

    let cache_key = BindGroupCacheKey {
        layout_num: 0,
        entries: vec![(
            0,
            BindGroupResourceType::Buffer {
                buffer: renderer.camera_buffer().clone(),
            },
        )],
    };
    gpu.get_or_create_bind_group(cache_key)
}
