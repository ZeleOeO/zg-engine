pub type Vec3 = [f32; 3];
pub type Mat4 = [[f32; 4]; 4];

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

pub fn transpose(matrix: Mat4) -> Mat4 {
    [
        [matrix[0][0], matrix[1][0], matrix[2][0], matrix[3][0]],
        [matrix[0][1], matrix[1][1], matrix[2][1], matrix[3][1]],
        [matrix[0][2], matrix[1][2], matrix[2][2], matrix[3][2]],
        [matrix[0][3], matrix[1][3], matrix[2][3], matrix[3][3]],
    ]
}
