use super::GameState;
use super::{
    font, framebuffer::Framebuffer, gl, quadrender, rex::XpFile, rex::XpLayer, Console, Shader,
    SimpleConsole, VirtualKeyCode, RGB,
};
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use std::ffi::CString;
use std::time::Instant;

/// A display console, used internally to provide console render support.
/// Public in case you want to play with it, or access it directly.
pub struct DisplayConsole {
    pub console: Box<dyn Console>,
    pub shader_index: usize,
    pub font_index: usize,
}

/// A helper, to get around difficulties with moving the event loop
/// and window context types.
struct WrappedContext {
    el: glutin::event_loop::EventLoop<()>,
    wc: glutin::WindowedContext<glutin::PossiblyCurrent>,
}

/// An RLTK context.
pub struct Rltk {
    pub gl: gl::Gles2,
    pub width_pixels: u32,
    pub height_pixels: u32,
    pub fonts: Vec<font::Font>,
    pub shaders: Vec<Shader>,
    pub consoles: Vec<DisplayConsole>,
    pub fps: f32,
    pub frame_time_ms: f32,
    pub active_console: usize,
    pub key: Option<glutin::event::VirtualKeyCode>,
    mouse_pos: (i32, i32),
    pub left_click: bool,
    context_wrapper: Option<WrappedContext>,
    quitting: bool,
    backing_buffer: Framebuffer,
    quad_vao: u32,
    post_scanlines: bool,
    post_screenburn: bool,
}

impl Rltk {
    /// Initializes an OpenGL context and a window, stores the info in the Rltk structure.
    pub fn init_raw<S: ToString>(
        width_pixels: u32,
        height_pixels: u32,
        window_title: S,
        path_to_shaders: S,
    ) -> Rltk {
        let el = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title(window_title.to_string())
            .with_inner_size(LogicalSize::new(f64::from(width_pixels), f64::from(height_pixels)));
        let windowed_context = ContextBuilder::new().with_vsync(true).build_windowed(wb, &el).unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let gl = gl::Gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

        // Load our basic shaders
        let mut shaders: Vec<Shader> = Vec::new();

        let shader_path = path_to_shaders.to_string();
        shaders.push(Shader::new(
            &gl,
            "console_with_bg.vs",
            "console_with_bg.fs",
            &shader_path,
        ));
        shaders.push(Shader::new(
            &gl,
            "console_no_bg.vs",
            "console_no_bg.fs",
            &shader_path,
        ));
        shaders.push(Shader::new(&gl, "backing.vs", "backing.fs", &shader_path));
        shaders.push(Shader::new(
            &gl,
            "scanlines.vs",
            "scanlines.fs",
            &shader_path,
        ));

        // Build the backing frame-buffer
        let backing_fbo = Framebuffer::build_fbo(&gl, width_pixels as i32, height_pixels as i32);

        // Build a simple quad rendering vao
        let quad_vao = quadrender::setup_quad(&gl);

        Rltk {
            gl,
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
            context_wrapper: Some(WrappedContext {
                el,
                wc: windowed_context,
            }),
            quitting: false,
            backing_buffer: backing_fbo,
            quad_vao,
            post_scanlines: false,
            post_screenburn: false,
        }
    }

    /// Quick initialization for when you just want an 8x8 font terminal
    pub fn init_simple8x8<S: ToString>(
        width_chars: u32,
        height_chars: u32,
        window_title: S,
        path_to_shaders: S,
    ) -> Rltk {
        let font_path = format!("{}/terminal8x8.jpg", &path_to_shaders.to_string());
        let mut context = Rltk::init_raw(
            width_chars * 8,
            height_chars * 8,
            window_title,
            path_to_shaders,
        );
        let font = context.register_font(font::Font::load(&font_path.to_string(), (8, 8)));
        context.register_console(
            SimpleConsole::init(width_chars, height_chars, &context.gl),
            font,
        );
        context
    }

    /// Quick initialization for when you just want an 8x16 VGA font terminal
    pub fn init_simple8x16<S: ToString>(
        width_chars: u32,
        height_chars: u32,
        window_title: S,
        path_to_shaders: S,
    ) -> Rltk {
        let font_path = format!("{}/vga8x16.jpg", &path_to_shaders.to_string());
        let mut context = Rltk::init_raw(
            width_chars * 8,
            height_chars * 16,
            window_title,
            path_to_shaders,
        );
        let font = context.register_font(font::Font::load(&font_path.to_string(), (8, 16)));
        context.register_console(
            SimpleConsole::init(width_chars, height_chars, &context.gl),
            font,
        );
        context
    }

    /// Registers a font, and returns its handle number. Also loads it into OpenGL.
    pub fn register_font(&mut self, mut font: font::Font) -> usize {
        font.setup_gl_texture(&self.gl);
        font.bind_texture(&self.gl);
        self.fonts.push(font);
        self.fonts.len() - 1
    }

    /// Registers a new console terminal for output, and returns its handle number.
    pub fn register_console(&mut self, new_console: Box<dyn Console>, font_index: usize) -> usize {
        self.consoles.push(DisplayConsole {
            console: new_console,
            font_index,
            shader_index: 0,
        });
        self.consoles.len() - 1
    }

    /// Registers a new console terminal for output, and returns its handle number. This variant requests
    /// that the new console not render background colors, so it can be layered on top of other consoles.
    pub fn register_console_no_bg(
        &mut self,
        new_console: Box<dyn Console>,
        font_index: usize,
    ) -> usize {
        self.consoles.push(DisplayConsole {
            console: new_console,
            font_index,
            shader_index: 1,
        });
        self.consoles.len() - 1
    }

    /// Sets the currently active console number.
    pub fn set_active_console(&mut self, id: usize) {
        self.active_console = id;
    }

    /// Applies the current physical mouse position to the active console, and translates
    /// the coordinates into that console's coordinate space.
    pub fn mouse_pos(&self) -> (i32, i32) {
        let font_size = self.fonts[self.consoles[self.active_console].font_index].tile_size;

        (
            (self.mouse_pos.0 as f32 / font_size.0 as f32) as i32,
            (self.mouse_pos.1 as f32 / font_size.1 as f32) as i32,
        )
    }

    /// Tells the game to quit
    pub fn quit(&mut self) {
        self.quitting = true;
    }

    /// Render a REX Paint (https://www.gridsagegames.com/rexpaint/) file as a sprite.
    /// The sprite will be offset by offset_x and offset_y.
    /// Transparent cells will not be rendered.
    pub fn render_xp_sprite(&mut self, xp: &super::rex::XpFile, x: i32, y: i32) {
        super::rex::xp_to_console(xp, &mut self.consoles[self.active_console].console, x, y);
    }

    /// Saves the entire console stack to a REX Paint xp file. If your consoles are of
    /// varying sizes, the file format supports it - but REX doesn't. So you may want to
    /// avoid that. You can also get individual layers with to_xp_layer.
    pub fn to_xp_file(&self, width: usize, height: usize) -> XpFile {
        let mut xp = XpFile::new(width, height);

        xp.layers
            .push(self.consoles[self.active_console].console.to_xp_layer());

        if self.consoles.len() > 1 {
            for layer in self.consoles.iter().skip(1) {
                xp.layers.push(layer.console.to_xp_layer());
            }
        }

        xp
    }

    /// Enable scanlines post-processing effect.
    pub fn with_post_scanlines(&mut self, with_burn: bool) {
        self.post_scanlines = true;
        self.post_screenburn = with_burn;
    }
}

impl Console for Rltk {
    // A couple of ones we'll never use
    fn rebuild_if_dirty(&mut self, _gl: &gl::Gles2) {}
    fn gl_draw(&mut self, _font: &font::Font, _shader: &Shader, _gl: &gl::Gles2) {}

    // Implement pass-through to active console

    fn at(&self, x: i32, y: i32) -> usize {
        self.consoles[self.active_console].console.at(x, y)
    }
    fn cls(&mut self) {
        self.consoles[self.active_console].console.cls();
    }
    fn cls_bg(&mut self, background: RGB) {
        self.consoles[self.active_console]
            .console
            .cls_bg(background);
    }
    fn print(&mut self, x: i32, y: i32, output: &str) {
        self.consoles[self.active_console]
            .console
            .print(x, y, output);
    }
    fn print_color(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, output: &str) {
        self.consoles[self.active_console]
            .console
            .print_color(x, y, fg, bg, output);
    }
    fn set(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, glyph: u8) {
        self.consoles[self.active_console]
            .console
            .set(x, y, fg, bg, glyph);
    }
    fn set_bg(&mut self, x: i32, y: i32, bg: RGB) {
        self.consoles[self.active_console].console.set_bg(x, y, bg);
    }
    fn draw_box(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        self.consoles[self.active_console]
            .console
            .draw_box(x, y, width, height, fg, bg);
    }
    fn draw_box_double(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        self.consoles[self.active_console]
            .console
            .draw_box_double(x, y, width, height, fg, bg);
    }
    fn draw_bar_horizontal(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        n: i32,
        max: i32,
        fg: RGB,
        bg: RGB,
    ) {
        self.consoles[self.active_console]
            .console
            .draw_bar_horizontal(x, y, width, n, max, fg, bg);
    }
    fn draw_bar_vertical(
        &mut self,
        x: i32,
        y: i32,
        height: i32,
        n: i32,
        max: i32,
        fg: RGB,
        bg: RGB,
    ) {
        self.consoles[self.active_console]
            .console
            .draw_bar_vertical(x, y, height, n, max, fg, bg);
    }
    fn print_centered(&mut self, y: i32, text: &str) {
        self.consoles[self.active_console]
            .console
            .print_centered(y, text);
    }
    fn print_color_centered(&mut self, y: i32, fg: RGB, bg: RGB, text: &str) {
        self.consoles[self.active_console]
            .console
            .print_color_centered(y, fg, bg, text);
    }
    fn to_xp_layer(&self) -> XpLayer {
        self.consoles[self.active_console].console.to_xp_layer()
    }
    fn set_offset(&mut self, x: f32, y: f32) {
        self.consoles[self.active_console].console.set_offset(x, y);
    }
}

/// Runs the RLTK application, calling into the provided gamestate handler every tick.
pub fn main_loop<GS: GameState>(mut rltk: Rltk, mut gamestate: GS) {
    let now = Instant::now();
    let mut prev_seconds = now.elapsed().as_secs();
    let mut prev_ms = now.elapsed().as_millis();
    let mut frames = 0;

    // We're doing a little dance here to get around lifetime/borrow checking.
    // Removing the context data from RLTK in an atomic swap, so it isn't borrowed after move.
    let wrap = std::mem::replace(&mut rltk.context_wrapper, None);
    let unwrap = wrap.unwrap();

    let el = unwrap.el;
    let wc = unwrap.wc;

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        if rltk.quitting {
            *control_flow = ControlFlow::Exit;
        }

        match event {
            Event::NewEvents(_) => {
                rltk.left_click = false;
                rltk.key = None;
            }
            Event::EventsCleared => {
                tock(
                    &mut rltk,
                    &mut gamestate,
                    &mut frames,
                    &mut prev_seconds,
                    &mut prev_ms,
                    &now,
                );
                wc.swap_buffers().unwrap();
            }
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    // Commenting out to see if it helps the Linux world
                    //let dpi_factor = wc.window().hidpi_factor();
                    //wc.resize(logical_size.to_physical(dpi_factor));
                }
                WindowEvent::RedrawRequested => {
                    //tock(&mut rltk, &mut gamestate, &mut frames, &mut prev_seconds, &mut prev_ms, &now);
                    wc.swap_buffers().unwrap();
                }
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::CursorMoved { position: pos, .. } => {
                    rltk.mouse_pos = (pos.x as i32, pos.y as i32);
                }

                WindowEvent::MouseInput { .. } => {
                    rltk.left_click = true;
                }

                WindowEvent::KeyboardInput {
                    input:
                        glutin::event::KeyboardInput {
                            virtual_keycode: Some(virtual_keycode),
                            state: glutin::event::ElementState::Pressed,
                            ..
                        },
                    ..
                } => {
                    rltk.key = Some(*virtual_keycode);
                }

                _ => (),
            },
            _ => (),
        }
    });
}

/// Internal handling of the main loop.
fn tock<GS: GameState>(
    rltk: &mut Rltk,
    gamestate: &mut GS,
    frames: &mut i32,
    prev_seconds: &mut u64,
    prev_ms: &mut u128,
    now: &Instant,
) {
    let now_seconds = now.elapsed().as_secs();
    *frames += 1;

    if now_seconds > *prev_seconds {
        rltk.fps = *frames as f32 / (now_seconds - *prev_seconds) as f32;
        *frames = 0;
        *prev_seconds = now_seconds;
    }

    let now_ms = now.elapsed().as_millis();
    if now_ms > *prev_ms {
        rltk.frame_time_ms = (now_ms - *prev_ms) as f32;
        *prev_ms = now_ms;
    }

    gamestate.tick(rltk);

    // Console structure - doesn't really have to be every frame...
    for cons in &mut rltk.consoles {
        cons.console.rebuild_if_dirty(&rltk.gl);
    }

    // Bind to the backing buffer
    if rltk.post_scanlines {
        rltk.backing_buffer.bind(&rltk.gl);
    }

    // Clear the screen
    unsafe {
        rltk.gl.ClearColor(0.2, 0.3, 0.3, 1.0);
        rltk.gl.Clear(gl::COLOR_BUFFER_BIT);
    }

    // Tell each console to draw itself
    for cons in &mut rltk.consoles {
        let font = &rltk.fonts[cons.font_index];
        let shader = &rltk.shaders[cons.shader_index];
        cons.console.gl_draw(font, shader, &rltk.gl);
    }

    if rltk.post_scanlines {
        // Now we return to the primary screen
        rltk.backing_buffer.default(&rltk.gl);
        unsafe {
            if rltk.post_scanlines {
                rltk.shaders[3].useProgram(&rltk.gl);
                rltk.shaders[3].setVec3(
                    &rltk.gl,
                    &CString::new("screenSize").unwrap(),
                    rltk.width_pixels as f32,
                    rltk.height_pixels as f32,
                    0.0,
                );
                rltk.shaders[3].setBool(
                    &rltk.gl,
                    &CString::new("screenBurn").unwrap(),
                    rltk.post_screenburn,
                );
            } else {
                rltk.shaders[2].useProgram(&rltk.gl);
            }
            rltk.gl.BindVertexArray(rltk.quad_vao);
            rltk.gl
                .BindTexture(gl::TEXTURE_2D, rltk.backing_buffer.texture);
            rltk.gl.DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}

/// For A-Z menus, translates the keys A through Z into 0..25
pub fn letter_to_option(key: VirtualKeyCode) -> i32 {
    match key {
        VirtualKeyCode::A => 0,
        VirtualKeyCode::B => 1,
        VirtualKeyCode::C => 2,
        VirtualKeyCode::D => 3,
        VirtualKeyCode::E => 4,
        VirtualKeyCode::F => 5,
        VirtualKeyCode::G => 6,
        VirtualKeyCode::H => 7,
        VirtualKeyCode::I => 8,
        VirtualKeyCode::J => 9,
        VirtualKeyCode::K => 10,
        VirtualKeyCode::L => 11,
        VirtualKeyCode::M => 12,
        VirtualKeyCode::N => 13,
        VirtualKeyCode::O => 14,
        VirtualKeyCode::P => 15,
        VirtualKeyCode::Q => 16,
        VirtualKeyCode::R => 17,
        VirtualKeyCode::S => 18,
        VirtualKeyCode::T => 19,
        VirtualKeyCode::U => 20,
        VirtualKeyCode::V => 21,
        VirtualKeyCode::W => 22,
        VirtualKeyCode::X => 23,
        VirtualKeyCode::Y => 24,
        VirtualKeyCode::Z => 25,
        _ => -1,
    }
}
