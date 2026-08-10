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
    emmisive: f32,
}

struct ItemUniform {
    transform: mat4x4<f32>,
}

struct Light {
    position: vec3<f32>,
    color: vec3<f32>,
}

@group(0) @binding(0) var<uniform> camera: mat4x4<f32>;

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
    // temporary transform till i figure out how to do the normal_matrix
    out.normal = normalize((model_transform.transform * vec4<f32>(in.normal, 0.0)).xyz);
    out.world_position = world_position.xyz;
    out.clip_position = camera * world_position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let ambient_light_intensity = 0.2;
    let base_color = select(
        vec4<f32>(material_uniform.color, 1.0).rgb,
        textureSample(texture, t_sampler, in.tex_cords).rgb,
        material_uniform.has_texture == 1.0
    );

    let normal = normalize(in.normal);

    if material_uniform.emmisive >= 0.0 {
        return vec4<f32>(base_color * material_uniform.emmisive, 1.0);
    }

    // diffuse lighting
    let light_dir = normalize(light_uniform.position - in.clip_position.xyz);
    let diffuse_strength = max(dot(light_uniform.position, light_dir), 0.0);
    let diffuse_color = light_uniform.color * diffuse_strength;

    // specular lighting

    let lightFinal = ambient_light_intensity + diffuse_color;
    return vec4<f32>(base_color * lightFinal, 1.0);
}
