use crate::prelude::{BTerm, InitHints};
use crate::Result;

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    _window_title: S,
    _platform_hints: InitHints,
) -> Result<BTerm> {
    use super::super::*;
    use super::*;
    use wasm_bindgen::JsCast;

    let canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    canvas.set_width(width_pixels);
    canvas.set_height(height_pixels);

    super::bind_wasm_events(&canvas);

    let webgl2_context = canvas
        .get_context("webgl2")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::WebGl2RenderingContext>()
        .unwrap();
    webgl2_context
        .get_extension("EXT_color_buffer_float")
        .expect("Unable to add extensions");

    let gl = glow::Context::from_webgl2_context(webgl2_context);

    // Load our basic shaders
    let mut shaders: Vec<Shader> = Vec::new();

    shaders.push(Shader::new(
        &gl,
        shader_strings::UBERSHADER_VS,
        shader_strings::UBERSHADER_FS,
    ));

    let quad_vao = quadrender::setup_quad(&gl);

    let mut be = BACKEND.lock().unwrap();
    be.gl = Some(gl);
    be.quad_vao = Some(quad_vao);

    Ok(BTerm {
        width_pixels,
        height_pixels,
        fonts: Vec::new(),
        consoles: Vec::new(),
        shaders,
        fps: 0.0,
        frame_time_ms: 0.0,
        active_console: 0,
        key: None,
        mouse_pos: (0, 0),
        left_click: false,
        shift: false,
        alt: false,
        control: false,
        web_button: None,
        quitting: false,
        post_scanlines: false,
        post_screenburn: false,
    })
}
