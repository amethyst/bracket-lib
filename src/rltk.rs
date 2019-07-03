extern crate glfw;
use self::glfw::{Context, Action};
extern crate gl;
use std::sync::mpsc::Receiver;
use super::GameState;
use std::time::{Instant};
use super::{ font, Console, Shader, RGB, SimpleConsole };
pub use glfw::Key;

pub struct DisplayConsole {
    pub console : Box<Console>,
    pub shader_index : usize,
    pub font_index : usize
}

#[allow(non_snake_case)]
pub struct Rltk {
    pub glfw : glfw::Glfw,
    pub window : glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    pub width_pixels : u32,
    pub height_pixels : u32,
    pub fonts : Vec<font::Font>,
    pub shaders : Vec<Shader>,
    pub consoles : Vec<DisplayConsole>,
    pub fps : f32,
    pub frame_time_ms : f32,
    pub active_console : usize,
    pub key : Option<Key>,
    mouse_pos: (i32, i32),
    pub left_click: bool,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Rltk {
    // Initializes an OpenGL context and a window, stores the info in the Rltk structure.
    pub fn init_raw<S: ToString>(width_pixels:u32, height_pixels:u32, window_title: S, path_to_shaders: S) -> Rltk {        
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
        #[cfg(target_os = "macos")]
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw.create_window(width_pixels, height_pixels, &window_title.to_string(), glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

        window.make_current();
        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);

        // gl: load all OpenGL function pointers
        // ---------------------------------------
        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);        

        // Load our basic shaders
        let vertex_path = format!("{}/console_with_bg.vs", path_to_shaders.to_string());
        let fragment_path = format!("{}/console_with_bg.fs", path_to_shaders.to_string());
        let vs = Shader::new(&vertex_path, &fragment_path);

        return Rltk{
            glfw: glfw, 
            window: window, 
            events: events,
            width_pixels : width_pixels,
            height_pixels: height_pixels,
            fonts : Vec::new(),
            consoles: Vec::new(),
            shaders: vec![vs],
            fps: 0.0,
            frame_time_ms: 0.0,
            active_console : 0,
            key: None,
            mouse_pos: (0,0),
            left_click: false,
        };
    }

    // Quick initialization for when you just want an 8x8 font terminal
    pub fn init_simple8x8<S: ToString>(width_chars : u32, height_chars: u32, window_title: S, path_to_shaders: S) -> Rltk {
        let font_path = format!("{}/terminal8x8.jpg", &path_to_shaders.to_string());
        let mut context = Rltk::init_raw(width_chars * 8, height_chars * 8, window_title, path_to_shaders);
        let font = context.register_font(font::Font::load(&font_path.to_string(), (8,8)));
        context.register_console(SimpleConsole::init(width_chars, height_chars), font);
        context
    }

    // Quick initialization for when you just want an 8x16 VGA font terminal
    pub fn init_simple8x16<S: ToString>(width_chars : u32, height_chars: u32, window_title: S, path_to_shaders: S) -> Rltk {
        let font_path = format!("{}/vga8x16.jpg", &path_to_shaders.to_string());
        let mut context = Rltk::init_raw(width_chars * 8, height_chars * 16, window_title, path_to_shaders);
        let font = context.register_font(font::Font::load(&font_path.to_string(), (8,16)));
        context.register_console(SimpleConsole::init(width_chars, height_chars), font);
        context
    }

    // Message pump handler for RLTK applications
    fn process_events(&mut self) {
        self.key = None; // To avoid infinite repetition
        self.left_click = false;

        for (_, event) in glfw::flush_messages(&self.events) {

            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe { gl::Viewport(0, 0, width, height) }
                }

                glfw::WindowEvent::Key(KEY, _, Action::Press, _) => {
                    self.key = Some(KEY);
                }

                glfw::WindowEvent::Key(KEY, _, Action::Repeat, _) => {
                    self.key = Some(KEY);
                }

                glfw::WindowEvent::CursorPos(x, y) => {
                    self.mouse_pos.0 = x as i32;
                    self.mouse_pos.1 = y as i32;
                }

                glfw::WindowEvent::MouseButton(glfw::MouseButton::Button1, Action::Press, _) => {
                    self.left_click = true;
                }

                _ => { }
            }
        }
    }

    // Runs the RLTK application, calling into the provided gamestate handler every tick.
    pub fn main_loop(&mut self, gamestate: &mut GameState) {
        let now = Instant::now();
        let mut prev_seconds = now.elapsed().as_secs();
        let mut prev_ms = now.elapsed().as_millis();
        let mut frames = 0;

        while !self.window.should_close() {
            let now_seconds = now.elapsed().as_secs();
            frames += 1;

            if now_seconds > prev_seconds {
                self.fps = frames as f32 / (now_seconds - prev_seconds) as f32;
                frames = 0;
                prev_seconds = now_seconds;
            }

            let now_ms = now.elapsed().as_millis();
            if now_ms > prev_ms {
                self.frame_time_ms = (now_ms - prev_ms) as f32;
                prev_ms = now_ms;
            }

            // events
            // -----
            self.process_events();
            gamestate.tick(self);

            // Console structure - doesn't really have to be every frame...
            for cons in self.consoles.iter_mut() {
                cons.console.rebuild_if_dirty();
            }

            // Clear the screen
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            
            // Tell each console to draw itself
            for cons in self.consoles.iter_mut() {
                let font = &self.fonts[cons.font_index];
                let shader = &self.shaders[cons.shader_index];
                cons.console.gl_draw(font, shader);
            } 

            // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
            // -------------------------------------------------------------------------------
            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    // Registers a font, and returns its handle number. Also loads it into OpenGL.
    pub fn register_font(&mut self, mut font : font::Font) -> usize {
        font.setup_gl_texture();
        font.bind_texture();
        self.fonts.push(font);
        self.fonts.len()-1
    }

    // Registers a new console terminal for output, and returns its handle number.
    pub fn register_console(&mut self, new_console : Box<Console>, font_index : usize) -> usize {
        self.consoles.push(DisplayConsole{ console:new_console, font_index: font_index, shader_index: 0 });
        self.consoles.len()-1
    }

    pub fn set_active_console(&mut self, id : usize) {
        self.active_console = id;
    }

    pub fn mouse_pos(&self) -> (i32, i32) {
        let font_size = self.fonts[self.consoles[self.active_console].font_index].tile_size;

        (
            (self.mouse_pos.0 as f32 / font_size.0 as f32) as i32,
            (self.mouse_pos.1 as f32 / font_size.1 as f32) as i32,
        )
    }
}

impl Console for Rltk {
    // A couple of ones we'll never use
    fn rebuild_if_dirty(&mut self) {}
    fn gl_draw(&mut self, _font : &font::Font, _shader : &Shader) {}

    // Implement pass-through to active console

    fn at(&self, x:i32, y:i32) -> usize { self.consoles[self.active_console].console.at(x,y) }
    fn cls(&mut self) { self.consoles[self.active_console].console.cls(); }
    fn cls_bg(&mut self, background : RGB) { self.consoles[self.active_console].console.cls_bg(background); }
    fn print(&mut self, x:i32, y:i32, output:&str) { self.consoles[self.active_console].console.print(x, y, output); }
    fn print_color(&mut self, x:i32, y:i32, fg:RGB, bg:RGB, output:&str) { self.consoles[self.active_console].console.print_color(x,y,fg,bg,output); }
}
