use super::{gl_error, ShaderId};
use crate::console::log;
use glow::HasContext;
use std::str;
use ultraviolet::Vec3;

#[allow(non_snake_case)]
pub struct Shader {
    pub ID: ShaderId,
}

/// NOTE: mixture of `shader_s.h` and `shader_m.h` (the latter just contains
/// a few more setters for uniforms)
impl Shader {
    pub fn new(gl: &glow::Context, vertex_code: &str, fragment_code: &str) -> Shader {
        // 1. compile shaders from strings
        let shader;
        unsafe {
            // vertex shader
            let vertex = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            gl.shader_source(vertex, vertex_code);
            gl.compile_shader(vertex);
            if !gl.get_shader_compile_status(vertex) {
                log(&vertex_code);
                log(&gl.get_shader_info_log(vertex));
                panic!();
            }

            // fragment Shader
            let fragment = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            gl.shader_source(fragment, fragment_code);
            gl.compile_shader(fragment);
            if !gl.get_shader_compile_status(fragment) {
                log(&fragment_code);
                log(&gl.get_shader_info_log(fragment));
                panic!();
            }

            // shader Program
            let id = gl.create_program().unwrap();
            gl.attach_shader(id, vertex);
            gl.attach_shader(id, fragment);
            gl.link_program(id);
            if !gl.get_program_link_status(id) {
                log(&gl.get_program_info_log(id));
                panic!();
            }

            // delete the shaders as they're linked into our program now and no longer necessary
            shader = Shader { ID: id }
        }

        //log("Shaders Compiled");

        shader
    }

    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    /// activate the shader
    /// ------------------------------------------------------------------------
    pub unsafe fn useProgram(&self, gl: &glow::Context) {
        gl.use_program(Some(self.ID));
        gl_error(gl);
    }

    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    /// utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn setBool(&self, gl: &glow::Context, name: &str, value: bool) {
        gl.uniform_1_i32(
            gl.get_uniform_location(self.ID, name).as_ref(),
            value as i32,
        );
    }

    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setInt(&self, gl: &glow::Context, name: &str, value: i32) {
        gl.uniform_1_i32(gl.get_uniform_location(self.ID, name).as_ref(), value);
    }

    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setFloat(&self, gl: &glow::Context, name: &str, value: f32) {
        gl.uniform_1_f32(gl.get_uniform_location(self.ID, name).as_ref(), value);
    }

    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setVector3(&self, gl: &glow::Context, name: &str, value: &Vec3) {
        gl.uniform_3_f32(
            gl.get_uniform_location(self.ID, name).as_ref(),
            value.x,
            value.y,
            value.z,
        );
    }

    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setVec3(&self, gl: &glow::Context, name: &str, x: f32, y: f32, z: f32) {
        gl.uniform_3_f32(gl.get_uniform_location(self.ID, name).as_ref(), x, y, z);
    }

    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setVec2(&self, gl: &glow::Context, name: &str, x: f32, y: f32) {
        gl.uniform_2_f32(gl.get_uniform_location(self.ID, name).as_ref(), x, y);
    }

    #[allow(non_snake_case)]
    #[allow(clippy::missing_safety_doc)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setMat4(&self, gl: &glow::Context, name: &str, mat: &[f32; 16]) {
        gl.uniform_matrix_4_f32_slice(gl.get_uniform_location(self.ID, name).as_ref(), false, mat);
    }
}
