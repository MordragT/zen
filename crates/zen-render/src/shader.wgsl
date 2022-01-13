// Vertex shader

//[[block]] // 1.
struct Uniforms {
    projection: mat4x4<f32>;
    view: mat4x4<f32>;
};
[[group(1), binding(0)]] // 2.
var<uniform> uniforms: Uniforms;

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
    [[location(2)]] normals: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = uniforms.projection * uniforms.view * vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    return out;
}


// Fragment shader

[[group(0), binding(0)]]
var t_diffuse: texture_2d<f32>;
[[group(0), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
// returned before vec4<f32>
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
    // var tex: vec4<u32> = textureLoad(t_diffuse, vec2<i32>(in.tex_coords), 0);
    // return vec4<f32>(f32(tex.x)/255.0, f32(tex.y)/255.0, f32(tex.z)/255.0, f32(tex.w)/255.0);
    //return vec4<f32>(0.9, 0.5, 0.2, 1.0);
}