mod keycodes;
pub use keycodes::*;

pub struct PlatformGL {
    pub quad_vao: glow::WebVertexArrayKey,
    pub context_wrapper: Option<WrappedContext>,
    pub backing_buffer: super::Framebuffer,
}

pub struct WrappedContext {}