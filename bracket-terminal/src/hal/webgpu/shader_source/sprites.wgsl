// Vertex shader

struct VertexInput {
    [[location(0)]] relative_pos: vec2<f32>;
    [[location(1)]] transform: vec2<f32>;
    [[location(2)]] fg: vec4<f32>;
    [[location(3)]] tex_coords: vec2<f32>;
    [[location(4)]] scale: vec2<f32>;
};

struct VertexOutput{
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] fg: vec4<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

[[stage(vertex)]]
fn main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    let base_pos : vec2<f32> = model.relative_pos;
    let scaled : vec2<f32> = base_pos * model.scale;
    let translated : vec2<f32> = scaled + model.transform.xy;

    out.clip_position = vec4<f32>(translated, 1.0, 1.0);
    out.fg = model.fg;
	out.tex_coords = model.tex_coords;

    return out;
}

// Fragment shader

[[group(0), binding(0)]]
var t_font: texture_2d<f32>;
[[group(0), binding(1)]]
var s_font: sampler;

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var out: VertexOutput;
    let original : vec4<f32> = textureSample(t_font, s_font, in.tex_coords);
    let fg = original * in.fg;
    return fg;
}
