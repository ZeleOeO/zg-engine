use wgpu::util::{BufferInitDescriptor, DeviceExt};

use bytemuck::{Pod, Zeroable};
use zg_graphics::*;
use zg_utils::math::{
    Mat4, mat4_mul, mat4_transpose, vec3_general_rotation_matrix, vec3_scaling_matrix,
    vec3_translation_matrix,
};
use zg_world::components::Transform;

#[derive(Clone, Pod, Copy, Zeroable)]
#[repr(C)]
pub struct ItemUniform {
    transform: Mat4,
}

pub fn create_transform_bind_group(
    transform: &Transform,
    gpu: &mut InternalGraphics,
) -> BindGroupCacheHandle {
    // so we want to get the mat 4 of the transform
    // which we create with the components
    // we can create a way to convert the stuff to a mat4 in here sha
    // sort of like
    // let matrix = scale(rotation(vec3_translation_matrix(transform.position)))
    // or we do scalematrix and then rotation matrix and then translation
    // and then matrix multiply
    // and then transpose
    // seems fair?
    // maybe if we were to make it async, we would spawn threads for them wait for the results and
    // then work
    let matrix = {
        let scale_matrix = vec3_scaling_matrix(transform.scale());
        let rotation_matrix = vec3_general_rotation_matrix(transform.rotation());
        let translation_matrix = vec3_translation_matrix(transform.position());

        mat4_mul(translation_matrix, mat4_mul(rotation_matrix, scale_matrix))
    };
    let item_uniform = ItemUniform {
        transform: mat4_transpose(matrix),
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
