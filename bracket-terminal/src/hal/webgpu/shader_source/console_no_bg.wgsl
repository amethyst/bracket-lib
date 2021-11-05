// Vertex shader

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] fg: vec4<f32>;
    [[location(2)]] bg: vec4<f32>;
    [[location(3)]] tex_coords: vec2<f32>;
};

struct VertexOutput{
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] fg: vec4<f32>;
    [[location(1)]] bg: vec4<f32>;
    [[location(2)]] tex_coords: vec2<f32>;
};

[[stage(vertex)]]
fn main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.fg = model.fg;
    out.bg = model.bg;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader

[[group(0), binding(0)]]
var t_font: texture_2d<f32>;
[[group(0), binding(1)]]
var s_font: sampler;

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let original : vec4<f32> = textureSample(t_font, s_font, in.tex_coords);
    if (original.r < 0.01 && original.g < 0.01 && original.b < 0.01) {
        discard;
    }
    if ((original.r > 0.1 || original.g > 0.1 || original.b > 0.1) || original.a > 0.1) {
        return original * in.fg;
    } else {
        return in.bg;
    }
}
