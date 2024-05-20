@group(0) @binding(0) var<uniform> mouse_pos: vec2<f32>;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

const blue = vec3<f32>(0.0, 0.0, 1.0);

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    let dist = sqrt((mouse_pos.x * mouse_pos.x) + (mouse_pos.y * mouse_pos.y));

    out.color = blue * (1.0 - dist);
    out.clip_position = vec4<f32>(model.position.x + mouse_pos.x, model.position.y + mouse_pos.y, model.position.z, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
