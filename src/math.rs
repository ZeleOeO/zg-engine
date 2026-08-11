use std::f32::consts::PI;

pub type Vec3 = [f32; 3];
pub type Mat4 = [[f32; 4]; 4];
pub type Mat3 = [[f32; 3]; 3];

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformMat3 {
    col0: [f32; 3],
    _pad0: f32,
    col1: [f32; 3],
    _pad1: f32,
    col2: [f32; 3],
    _pad2: f32,
}

pub fn vec3_add(vec_one: Vec3, vec_two: Vec3) -> Vec3 {
    [
        vec_one[0] + vec_two[0],
        vec_one[1] + vec_two[1],
        vec_one[2] + vec_two[2],
    ]
}

pub fn vec3_sub(vec_one: Vec3, vec_two: Vec3) -> Vec3 {
    [
        vec_one[0] - vec_two[0],
        vec_one[1] - vec_two[1],
        vec_one[2] - vec_two[2],
    ]
}

pub fn vec3_mult_scal(vec: Vec3, s: f32) -> Vec3 {
    [vec[0] * s, vec[1] * s, vec[2] * s]
}

// Measures how much 2 vectors point in the same direction
pub fn vec3_dot_product(vec_one: Vec3, vec_two: Vec3) -> f32 {
    vec_one[0] * vec_two[0] + vec_one[1] * vec_two[1] + vec_one[2] * vec_two[2]
}

// Creates a vector perpendicular to both vectors
pub fn vec3_cross_product(vec_one: Vec3, vec_two: Vec3) -> Vec3 {
    [
        (vec_one[1] * vec_two[2]) - (vec_one[2] * vec_two[1]),
        (vec_one[2] * vec_two[0]) - (vec_one[0] * vec_two[2]),
        (vec_one[0] * vec_two[1]) - (vec_one[1] * vec_two[0]),
    ]
}

pub fn vec3_length(vec: Vec3) -> f32 {
    (vec[0] * vec[0] + vec[1] * vec[1] + vec[2] * vec[2]).sqrt()
}

pub fn vec3_normalize(vec: Vec3) -> Vec3 {
    let length = vec3_length(vec);
    if length == 0.0 {
        return [0.0, 0.0, 0.0];
    }

    [vec[0] / length, vec[1] / length, vec[2] / length]
}

pub fn vec3_translation_matrix(translate: Vec3) -> Mat4 {
    [
        [1.0, 0.0, 0.0, translate[0]],
        [0.0, 1.0, 0.0, translate[1]],
        [0.0, 0.0, 1.0, translate[2]],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn vec3_rotate_by_y(vector: Vec3, degree: f32) -> Vec3 {
    let radians: f32 = degree * PI / 180.0;

    [
        vector[0] * radians.cos() - vector[2] * radians.sin(),
        vector[1],
        vector[0] * radians.sin() + vector[2] * radians.sin(),
    ]
}

pub fn mat4_identity() -> Mat4 {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

pub fn mat4_mul(a: Mat4, b: Mat4) -> Mat4 {
    let mut result = [[0.0; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            result[i][j] =
                a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + a[i][3] * b[3][j];
        }
    }
    result
}

pub fn mat4_transpose(matrix: Mat4) -> Mat4 {
    [
        [matrix[0][0], matrix[1][0], matrix[2][0], matrix[3][0]],
        [matrix[0][1], matrix[1][1], matrix[2][1], matrix[3][1]],
        [matrix[0][2], matrix[1][2], matrix[2][2], matrix[3][2]],
        [matrix[0][3], matrix[1][3], matrix[2][3], matrix[3][3]],
    ]
}

// Everything below is AI-generated, I just want to get this thing done mehn
fn invert_mat3(m: Mat3) -> Option<Mat3> {
    let a = m[0][0];
    let b = m[0][1];
    let c = m[0][2];
    let d = m[1][0];
    let e = m[1][1];
    let f = m[1][2];
    let g = m[2][0];
    let h = m[2][1];
    let i = m[2][2];

    let det = a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g);

    if det.abs() < 1e-8 {
        return None;
    }

    let inv_det = 1.0 / det;

    Some([
        [
            (e * i - f * h) * inv_det,
            (c * h - b * i) * inv_det,
            (b * f - c * e) * inv_det,
        ],
        [
            (f * g - d * i) * inv_det,
            (a * i - c * g) * inv_det,
            (c * d - a * f) * inv_det,
        ],
        [
            (d * h - e * g) * inv_det,
            (b * g - a * h) * inv_det,
            (a * e - b * d) * inv_det,
        ],
    ])
}

fn mat3_transpose(m: Mat3) -> Mat3 {
    [
        [m[0][0], m[1][0], m[2][0]],
        [m[0][1], m[1][1], m[2][1]],
        [m[0][2], m[1][2], m[2][2]],
    ]
}

fn mat4_to_mat3(m: Mat4) -> Mat3 {
    [
        [m[0][0], m[0][1], m[0][2]],
        [m[1][0], m[1][1], m[1][2]],
        [m[2][0], m[2][1], m[2][2]],
    ]
}

fn compute_normal_matrix(model: Mat4) -> Mat3 {
    let m3 = mat4_to_mat3(model);

    match invert_mat3(m3) {
        Some(inv) => mat3_transpose(inv),
        None => [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    }
}
