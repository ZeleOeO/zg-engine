struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_cords: vec2<f32>,
    @location(1) world_position: vec3<f32>,
    @location(2) normal: vec3<f32>,
}

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_cords: vec2<f32>,
    @location(2) normal: vec3<f32>,
}

struct MaterialUniform {
    color: vec3<f32>,
    has_texture: f32,
}

struct ItemUniform {
    transform: mat4x4<f32>,
    normal_matrix: mat3x3<f32>,
}

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}

struct CamaraUniform {
    projection: mat4x4<f32>,
    position: vec3<f32>,
}

@group(0) @binding(0) var<uniform> camera: CamaraUniform;

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var t_sampler: sampler;
@group(1) @binding(2) var<uniform> material_uniform: MaterialUniform;

@group(2) @binding(0) var<uniform> model_transform: ItemUniform;

@group(3) @binding(0) var<uniform> light_uniform: Light;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_cords = in.tex_cords;
    let world_position = model_transform.transform * vec4<f32>(in.position, 1.0);
    out.normal = model_transform.normal_matrix * in.normal;
    out.world_position = world_position.xyz;
    out.clip_position = camera.projection * world_position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = select(
        material_uniform.color, textureSample(texture, t_sampler, in.tex_cords).rgb,
        material_uniform.has_texture == 1.0
    );

    let light_dir = normalize(light_uniform.position - in.world_position);
    let view = normalize(camera.position - in.world_position);
    let normal = normalize(in.normal);

    // ambient lighting
    let ambient_light_intensity = 0.05;
    let ambient_color = ambient_light_intensity * light_uniform.color;

    // diffuse lighting
    let diffuse_strength = max(dot(normal, light_dir), 0.0);
    let diffuse_color = light_uniform.color * diffuse_strength;

    // specular lighting
    let half_dir = normalize(light_dir + view);
    let specular_strength = pow(max(dot(normal, half_dir), 0.0), 128.0);
    let specular_color = specular_strength * light_uniform.color;

    let light_final = ambient_color + diffuse_color + specular_color;
    let final_color = base_color * (ambient_color + diffuse_color) + specular_color;

    return vec4<f32>(final_color, 1.0);
}
