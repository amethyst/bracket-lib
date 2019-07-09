use super::GameState;
use std::time::{Instant};
use super::{ font, Console, Shader, RGB, SimpleConsole, gl };
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use glutin::dpi::LogicalSize;
extern crate winit;

pub struct DisplayConsole {
    pub console : Box<Console>,
    pub shader_index : usize,
    pub font_index : usize
}

struct WrappedContext {
    el : glutin::event_loop::EventLoop<()>, 
    wc : glutin::WindowedContext<glutin::PossiblyCurrent>
}

#[allow(non_snake_case)]
pub struct Rltk {
    pub gl : gl::Gles2,
    pub width_pixels : u32,
    pub height_pixels : u32,
    pub fonts : Vec<font::Font>,
    pub shaders : Vec<Shader>,
    pub consoles : Vec<DisplayConsole>,
    pub fps : f32,
    pub frame_time_ms : f32,
    pub active_console : usize,
    pub key : Option<glutin::event::VirtualKeyCode>,
    mouse_pos: (i32, i32),
    pub left_click: bool,
    context_wrapper : Option<WrappedContext>
}

#[allow(dead_code)]
#[allow(non_snake_case)]
impl Rltk {
    // Initializes an OpenGL context and a window, stores the info in the Rltk structure.
    pub fn init_raw<S: ToString>(width_pixels:u32, height_pixels:u32, window_title: S, path_to_shaders: S) -> Rltk {
        let el = EventLoop::new();
        let wb = WindowBuilder::new().with_title(window_title.to_string()).with_inner_size(LogicalSize::new(width_pixels as f64, height_pixels as f64));
        let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();
        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let gl = gl::Gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

        // Load our basic shaders
        let vertex_path = format!("{}/console_with_bg.vs", path_to_shaders.to_string());
        let fragment_path = format!("{}/console_with_bg.fs", path_to_shaders.to_string());
        let vs = Shader::new(&gl, &vertex_path, &fragment_path);

        Rltk{
            gl: gl,
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
            context_wrapper: Some(WrappedContext{ el: el, wc: windowed_context })
        }
    }

    // Quick initialization for when you just want an 8x8 font terminal
    pub fn init_simple8x8<S: ToString>(width_chars : u32, height_chars: u32, window_title: S, path_to_shaders: S) -> Rltk {
        let font_path = format!("{}/terminal8x8.jpg", &path_to_shaders.to_string());
        let mut context = Rltk::init_raw(width_chars * 8, height_chars * 8, window_title, path_to_shaders);
        let font = context.register_font(font::Font::load(&font_path.to_string(), (8,8)));
        context.register_console(SimpleConsole::init(width_chars, height_chars, &context.gl), font);
        context
    }

    // Quick initialization for when you just want an 8x16 VGA font terminal
    pub fn init_simple8x16<S: ToString>(width_chars : u32, height_chars: u32, window_title: S, path_to_shaders: S) -> Rltk {
        let font_path = format!("{}/vga8x16.jpg", &path_to_shaders.to_string());
        let mut context = Rltk::init_raw(width_chars * 8, height_chars * 16, window_title, path_to_shaders);
        let font = context.register_font(font::Font::load(&font_path.to_string(), (8,16)));
        context.register_console(SimpleConsole::init(width_chars, height_chars, &context.gl), font);
        context
    }    

    // Registers a font, and returns its handle number. Also loads it into OpenGL.
    pub fn register_font(&mut self, mut font : font::Font) -> usize {
        font.setup_gl_texture(&self.gl);
        font.bind_texture(&self.gl);
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
    fn rebuild_if_dirty(&mut self, _gl : &gl::Gles2) {}
    fn gl_draw(&mut self, _font : &font::Font, _shader : &Shader, _gl : &gl::Gles2) {}

    // Implement pass-through to active console

    fn at(&self, x:i32, y:i32) -> usize { self.consoles[self.active_console].console.at(x,y) }
    fn cls(&mut self) { self.consoles[self.active_console].console.cls(); }
    fn cls_bg(&mut self, background : RGB) { self.consoles[self.active_console].console.cls_bg(background); }
    fn print(&mut self, x:i32, y:i32, output:&str) { self.consoles[self.active_console].console.print(x, y, output); }
    fn print_color(&mut self, x:i32, y:i32, fg:RGB, bg:RGB, output:&str) { self.consoles[self.active_console].console.print_color(x,y,fg,bg,output); }
}

// Runs the RLTK application, calling into the provided gamestate handler every tick.
#[allow(non_snake_case)]
pub fn main_loop(mut rltk : Rltk, mut gamestate: Box<GameState>) {
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
        //println!("{:?}", event);
        *control_flow = ControlFlow::Poll;

        match event {
            Event::NewEvents(_) => {
                rltk.left_click = false;
                rltk.key = None;
            }
            Event::EventsCleared => {
                tock(&mut rltk, &mut gamestate, &mut frames, &mut prev_seconds, &mut prev_ms, &now);
                wc.swap_buffers().unwrap();
            }
            Event::LoopDestroyed => return,
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::Resized(logical_size) => {
                    let dpi_factor = wc.window().hidpi_factor();
                    wc.resize(logical_size.to_physical(dpi_factor));
                }
                WindowEvent::RedrawRequested => {
                    //tock(&mut rltk, &mut gamestate, &mut frames, &mut prev_seconds, &mut prev_ms, &now);
                    wc.swap_buffers().unwrap();
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit
                }

                WindowEvent::CursorMoved{device_id : _,  position: pos, modifiers: _} => {
                    rltk.mouse_pos = (pos.x as i32, pos.y as i32);
                }

                WindowEvent::MouseInput{ device_id : _, state: _, button: _, modifiers: _} => {
                    rltk.left_click = true;
                }

                WindowEvent::KeyboardInput {
                    input:
                        glutin::event::KeyboardInput { virtual_keycode: Some(virtual_keycode),
                        state : glutin::event::ElementState::Pressed,
                        ..
                    },
                    ..
                } => {
                    //println!("{:?}", event);
                    rltk.key = Some(*virtual_keycode);
                }                

                _ => (),
            },
            _ => (),
        }

        /*if event == Event::EventsCleared {
            //println!("tock");
            tock(&mut rltk, &mut gamestate, &mut frames, &mut prev_seconds, &mut prev_ms, &now);
            wc.swap_buffers().unwrap();
        }*/
    });
}

fn tock(rltk : &mut Rltk, gamestate: &mut Box<GameState>, frames: &mut i32, prev_seconds : &mut u64, prev_ms : &mut u128, now : &Instant) {    
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
    for cons in rltk.consoles.iter_mut() {
        cons.console.rebuild_if_dirty(&rltk.gl);
    }

    // Clear the screen
    unsafe {
        rltk.gl.ClearColor(0.2, 0.3, 0.3, 1.0);
        rltk.gl.Clear(gl::COLOR_BUFFER_BIT);
    }
    
    // Tell each console to draw itself
    for cons in rltk.consoles.iter_mut() {
        let font = &rltk.fonts[cons.font_index];
        let shader = &rltk.shaders[cons.shader_index];
        cons.console.gl_draw(font, shader, &rltk.gl);
    } 
}