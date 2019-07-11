use super::gl;
use std::ptr;

pub struct Framebuffer {
    fbo : u32,
    pub texture : u32
}

impl Framebuffer {
    pub fn build_fbo(gl : &gl::Gles2, width: i32, height: i32) -> Framebuffer {
        let mut fbo : u32 = 0;
        let mut buffer : u32 = 0;

        unsafe {
            gl.GenFramebuffers(1, &mut fbo);
            gl.BindFramebuffer(gl::FRAMEBUFFER, fbo);
            gl.GenTextures(1, &mut buffer);

            gl.BindTexture(gl::TEXTURE_2D, buffer);
            gl.TexImage2D(gl::TEXTURE_2D, 0, gl::RGB16F as i32, width, height, 0, gl::RGB, gl::FLOAT, ptr::null());
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            // attach texture to framebuffer
            gl.FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, buffer, 0);
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Framebuffer {
            fbo : fbo,
            texture : buffer
        }
    }

    pub fn bind(&self, gl : &gl::Gles2) {
        unsafe {
            gl.BindFramebuffer(gl::FRAMEBUFFER, self.fbo);
        }
    }

    pub fn default(&self, gl : &gl::Gles2) {
        unsafe {
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}