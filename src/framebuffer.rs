use super::gl;
use std::ptr;
use glow::HasContext;

pub struct Framebuffer {
    fbo: u32,
    pub texture: u32,
}

impl Framebuffer {
    pub fn build_fbo(gl: &glow::Context, width: i32, height: i32) -> Framebuffer {
        let mut fbo: u32 = 0;
        let mut buffer: u32 = 0;

        unsafe {
            fbo = gl.create_framebuffer().unwrap();
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
            buffer = gl.create_texture().unwrap();

            gl.bind_texture(glow::TEXTURE_2D, Some(buffer));
            gl.tex_image_2d(
                gl::TEXTURE_2D,
                0,
                gl::RGB16F as i32,
                width,
                height,
                0,
                gl::RGB,
                gl::FLOAT,
                None,
            );
            gl.tex_parameter_i32(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.tex_parameter_i32(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl.tex_parameter_i32(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32);
            // attach texture to framebuffer
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(buffer),
                0,
            );
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }

        Framebuffer {
            fbo,
            texture: buffer,
        }
    }

    pub fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.fbo));
        }
    }

    pub fn default(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
        }
    }
}
