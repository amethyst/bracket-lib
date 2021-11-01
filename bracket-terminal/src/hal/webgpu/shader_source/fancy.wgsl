// Vertex shader

struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] fg: vec4<f32>;
    [[location(2)]] bg: vec4<f32>;
    [[location(3)]] tex_coords: vec2<f32>;
    [[location(4)]] rotate: vec3<f32>;
    [[location(5)]] scale: vec2<f32>;
};

struct VertexOutput{
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] fg: vec4<f32>;
    [[location(1)]] bg: vec4<f32>;
    [[location(2)]] tex_coords: vec2<f32>;
};

fn r2d(a:f32) -> mat2x2<f32> {
	let c : f32 = cos(a);
    let s : f32 = sin(a);
    return mat2x2<f32>(
        vec2<f32>(c, s),
        vec2<f32>(-s, c)
    );
}

[[stage(vertex)]]
fn main(
    model: VertexInput,
) -> VertexOutput {
    let rot : f32 = model.rotate.x;
    let center_pos : vec2<f32> = model.rotate.yz;
    var base_pos : vec2<f32> = model.position.xy - center_pos;
    base_pos = base_pos * r2d(rot);
    base_pos = base_pos * model.scale;
    base_pos = base_pos + center_pos;

    var out: VertexOutput;
    out.clip_position = vec4<f32>(base_pos, 0.0, 1.0);
    out.fg = model.fg;
    out.bg = model.bg;
    out.tex_coords = vec2<f32>(model.tex_coords.x, model.tex_coords.y);

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
    if ((original.r > 0.1 || original.g > 0.1 || original.b > 0.1) && original.a > 0.1) {
        return original * in.fg;
    } else {
        return in.bg;
    }
}
