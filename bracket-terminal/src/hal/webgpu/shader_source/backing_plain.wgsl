// Uniform
[[block]]
struct BackingUnform {
    enable_scan_lines: f32;
    enable_screen_burn: f32;
    padding: vec2<f32>;
    screen_burn_color: vec4<f32>;
};
[[group(1), binding(0)]]
var<uniform> post_process: BackingUnform;

// Vertex shader
struct VertexInput {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] tex_coords: vec2<f32>;
};

struct VertexOutput{
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[stage(vertex)]]
fn main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
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
    let base = textureSample(t_font, s_font, in.tex_coords);

    if (post_process.enable_scan_lines > 0.0) {
        let scan_line : f32 = (in.clip_position.y % 2.0) * 0.25;
        let scan_color : vec3<f32> = base.rgb - scan_line;

        if (base.r < 0.1 && base.g < 0.1 && base.b < 0.1) {
            if (post_process.enable_screen_burn > 0.0) {
                let dist : f32 = (1.0 - distance(
                    vec2<f32>(in.tex_coords.x, in.tex_coords.y),
                    vec2<f32>(0.5,0.5)
                )) * 0.05;
                return vec4<f32>(
                    dist * post_process.screen_burn_color.rgb,
                    1.0
                );
            } else {
                return vec4<f32>(0.0, 0.0, 0.0, 1.0);
            }
        }

        return vec4<f32>(scan_color, 1.0);
    }

    return base;

//    let col : vec4<f32> = textureSample(t_font, s_font, in.tex_coords);

    // If there are no post effects
//    if (post_process.enable_scan_lines < 1.0) {
//        return col;
//    }

//    let scan_line : f32 = (in.clip_position.y % 2.0) * 0.25;
//    let scan_color : vec3<f32> = col.rgb;

//    if (col.r < 0.1 && col.g < 0.1 && col.b < 0.1) {
        // Background pixel
//    } else {
        // Foreground pixel
//        return vec4<f32>(scan_color, 1.0);
//    }

//    return col;
}
