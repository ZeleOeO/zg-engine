struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>}

struct VertexIn {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>}

@group(0) @binding(0) var<uniform> frame: f32;

fn rotate2D(deg: f32) -> mat2x2<f32> {
    return mat2x2<f32>(cos(deg), sin(deg), -1.0 * sin(deg), cos(deg));
}

@vertex
fn vs_main(in: VertexIn) -> VertexOutput {
    var out: VertexOutput;
    out.color = in.color;
    out.clip_position = vec4<f32>(in.position, 1.0) * vec4<f32>(0.6, 0.6, 0.6, 1.0);
    out.clip_position = vec4<f32>(out.clip_position.xy * rotate2D(frame * 0.01), out.clip_position.zw);
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
