extern crate glfw;
use self::glfw::{Context, Action};
extern crate gl;
use std::sync::mpsc::Receiver;
use super::GameState;
use std::time::{Instant};

#[allow(non_snake_case)]
pub struct Rltk {
    pub glfw : glfw::Glfw,
    pub window : glfw::Window,
    pub events: Receiver<(f64, glfw::WindowEvent)>,
    pub width_pixels : u32,
    pub height_pixels : u32
}

#[allow(dead_code)]
impl Rltk {
    // Initializes an OpenGL context and a window, stores the info in the Rltk structure.
    pub fn init_raw<S: ToString>(width_pixels:u32, height_pixels:u32, window_title: S) -> Rltk {        
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

        return Rltk{
            glfw: glfw, 
            window: window, 
            events: events,
            width_pixels : width_pixels,
            height_pixels: height_pixels
        };
    }

    // Message pump handler for RLTK applications
    fn process_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {

            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    // make sure the viewport matches the new window dimensions; note that width and
                    // height will be significantly larger than specified on retina displays.
                    unsafe { gl::Viewport(0, 0, width, height) }
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
                //self.fps = frames as f32 / (now_seconds - prev_seconds) as f32;
                frames = 0;
                prev_seconds = now_seconds;
            }

            let now_ms = now.elapsed().as_millis();
            if now_ms > prev_ms {
                //self.frame_time_ms = (now_ms - prev_ms) as f32;
                prev_ms = now_ms;
            }

            // events
            // -----
            self.process_events();
            gamestate.tick(self);

            // Console structure - doesn't really have to be every frame...
            //for cons in self.consoles.iter_mut() {
            //    cons.rebuild_if_dirty();
            //}         

            // Clear the screen
            unsafe {
                gl::ClearColor(0.2, 0.3, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            //for cons in self.consoles.iter_mut() {
            //    cons.gl_draw();
            //} 

            // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
            // -------------------------------------------------------------------------------
            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }
}
