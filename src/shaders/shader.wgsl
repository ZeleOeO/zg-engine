struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_cords: vec2<f32>,
}

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_cords: vec2<f32>,
}

@group(1) @binding(0) var texture: texture_2d<f32>;
@group(1) @binding(1) var t_sampler: sampler;

@group(0) @binding(0) var<uniform> frame: f32;

fn rotate3D(deg: f32) -> mat4x4<f32> {
    return mat4x4(vec4<f32>(cos(deg), 0.0, sin(deg), 0.0),
        vec4<f32>(0.0, 1.0, 0.0, 0.0),
        vec4<f32>(-sin(deg), 0.0, cos(deg), 0.0),
        vec4<f32>(0.0, 0.0, 0.0, 1.0));
}

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.tex_cords = in.tex_cords;
    out.clip_position = rotate3D(frame * 0.01) * vec4<f32>(in.position, 1.0);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, t_sampler, in.tex_cords);
}
