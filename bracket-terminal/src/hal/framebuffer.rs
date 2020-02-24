use crate::Result;
use glow::HasContext;

#[cfg(not(target_arch = "wasm32"))]
pub struct Framebuffer {
    fbo: u32,
    pub texture: u32,
}

#[cfg(target_arch = "wasm32")]
pub struct Framebuffer {
    fbo: glow::WebFramebufferKey,
    pub texture: glow::WebTextureKey,
}

impl Framebuffer {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn build_fbo(gl: &glow::Context, width: i32, height: i32) -> Result<Framebuffer> {
        let fbo;
        let buffer;

        unsafe {
            fbo = gl.create_framebuffer()?;
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
            buffer = gl.create_texture()?;

            gl.bind_texture(glow::TEXTURE_2D, Some(buffer));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width,
                height,
                0,
                glow::RGBA,
                glow::FLOAT,
                None,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::LINEAR as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
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

        let framebuffer = Framebuffer {
            fbo,
            texture: buffer,
        };
        Ok(framebuffer)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn build_fbo(gl: &glow::Context, width: i32, height: i32) -> Framebuffer {
        let fbo;
        let buffer;

        unsafe {
            fbo = gl.create_framebuffer().unwrap();
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
            buffer = gl.create_texture().unwrap();

            gl.bind_texture(glow::TEXTURE_2D, Some(buffer));
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RGBA as i32,
                width,
                height,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                None,
            );

            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );

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
