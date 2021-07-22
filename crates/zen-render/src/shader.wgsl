// Vertex shader

strict VertexInput {
    [[location(0)]] x: f32;
    [[location(1)]] y: f32;
    [[location(2)]] z: f32;
}

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
};

[[stage(vertex)]]
fn main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.x, model.y, model.z, 1.0);
    return out;
}


// Fragment shader

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}

 

 