
@group(0) @binding(0) var<storage, read> input: array<f32>;
@group(0) @binding(1) var<storage, read_write> result: f32;

@compute
@workgroup_size(1)
fn sum() {
    for (var i = 0u; i < arrayLength(&input); i++) {
        result += input[i];
    }
}
