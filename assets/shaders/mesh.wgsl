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
    // normal_matrix: mat3x3<f32>,
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

fn lit(light: vec3f, normal: vec3f, view: vec3f) -> vec3f {
    let rflct = reflect(-light, normal);
    let highlight = vec3<f32>(2.0, 2.0, 2.0);
    let warm = vec3<f32>(0.3, 0.3, 0.0);
    let s = clamp(100.0 * dot(rflct, view) - 97, 0.0, 1.0);

    return mix(warm, highlight, s);
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_cords = in.tex_cords;
    let world_position = model_transform.transform * vec4<f32>(in.position, 1.0);
    out.normal = (model_transform.transform * vec4<f32>(in.normal, 1.0)).xyz;
    out.world_position = world_position.xyz;
    out.clip_position = camera.projection * world_position;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let base_color = select(
        vec4<f32>(material_uniform.color, 1.0).rgb,
        textureSample(texture, t_sampler, in.tex_cords).rgb,
        material_uniform.has_texture == 1.0
    );

    let light = normalize(light_uniform.position - in.world_position);
    let normal = normalize(in.normal);
    let view = normalize(camera.position - in.world_position);

    let shaded = clamp(dot(light, normal), 0.0, 1.0);
    let light_final = shaded * light_uniform.color * lit(light, normal, view);

    let final_color = base_color + light_final;

    return vec4<f32>(final_color, 1.0);
}
