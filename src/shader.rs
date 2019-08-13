use super::gl;
use cgmath::prelude::*;
use cgmath::{Matrix, Matrix4, Vector3};
use gl::types::*;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

#[allow(non_snake_case)]
pub struct Shader {
    pub ID: u32,
}

/// NOTE: mixture of `shader_s.h` and `shader_m.h` (the latter just contains
/// a few more setters for uniforms)
impl Shader {
    pub fn new<S: ToString>(
        gl: &gl::Gles2,
        vertex_path: S,
        fragment_path: S,
        path_to_shaders: S,
    ) -> Shader {
        let shader_path = path_to_shaders.to_string();
        let vertex_path = format!("{}/{}", &shader_path, vertex_path.to_string());
        let fragment_path = format!("{}/{}", &shader_path, fragment_path.to_string());

        let mut shader = Shader { ID: 0 };
        // 1. retrieve the vertex/fragment source code from filesystem
        let mut v_shader_file =
            File::open(&vertex_path).unwrap_or_else(|_| panic!("Failed to open {}", vertex_path));
        let mut f_shader_file = File::open(&fragment_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragment_path));
        let mut vertex_code = String::new();
        let mut fragment_code = String::new();
        v_shader_file
            .read_to_string(&mut vertex_code)
            .expect("Failed to read vertex shader");
        f_shader_file
            .read_to_string(&mut fragment_code)
            .expect("Failed to read fragment shader");

        let v_shader_code = CString::new(vertex_code.as_bytes()).unwrap();
        let f_shader_code = CString::new(fragment_code.as_bytes()).unwrap();

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl.CreateShader(gl::VERTEX_SHADER);
            gl.ShaderSource(vertex, 1, &v_shader_code.as_ptr(), ptr::null());
            gl.CompileShader(vertex);
            shader.checkCompileErrors(gl, vertex, "VERTEX");
            // fragment Shader
            let fragment = gl.CreateShader(gl::FRAGMENT_SHADER);
            gl.ShaderSource(fragment, 1, &f_shader_code.as_ptr(), ptr::null());
            gl.CompileShader(fragment);
            shader.checkCompileErrors(gl, fragment, "FRAGMENT");
            // shader Program
            let id = gl.CreateProgram();
            gl.AttachShader(id, vertex);
            gl.AttachShader(id, fragment);
            gl.LinkProgram(id);
            shader.checkCompileErrors(gl, id, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl.DeleteShader(vertex);
            gl.DeleteShader(fragment);
            shader.ID = id;
        }

        shader
    }

    #[allow(non_snake_case)]
    /// activate the shader
    /// ------------------------------------------------------------------------
    pub unsafe fn useProgram(&self, gl: &gl::Gles2) {
        gl.UseProgram(self.ID)
    }

    #[allow(non_snake_case)]
    /// utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn setBool(&self, gl: &gl::Gles2, name: &CStr, value: bool) {
        gl.Uniform1i(gl.GetUniformLocation(self.ID, name.as_ptr()), value as i32);
    }
    #[allow(non_snake_case)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setInt(&self, gl: &gl::Gles2, name: &CStr, value: i32) {
        gl.Uniform1i(gl.GetUniformLocation(self.ID, name.as_ptr()), value);
    }
    #[allow(non_snake_case)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setFloat(&self, gl: &gl::Gles2, name: &CStr, value: f32) {
        gl.Uniform1f(gl.GetUniformLocation(self.ID, name.as_ptr()), value);
    }
    #[allow(non_snake_case)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setVector3(&self, gl: &gl::Gles2, name: &CStr, value: &Vector3<f32>) {
        gl.Uniform3fv(
            gl.GetUniformLocation(self.ID, name.as_ptr()),
            1,
            value.as_ptr(),
        );
    }
    #[allow(non_snake_case)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setVec3(&self, gl: &gl::Gles2, name: &CStr, x: f32, y: f32, z: f32) {
        gl.Uniform3f(gl.GetUniformLocation(self.ID, name.as_ptr()), x, y, z);
    }
    #[allow(non_snake_case)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setMat4(&self, gl: &gl::Gles2, name: &CStr, mat: &Matrix4<f32>) {
        gl.UniformMatrix4fv(
            gl.GetUniformLocation(self.ID, name.as_ptr()),
            1,
            gl::FALSE,
            mat.as_ptr(),
        );
    }

    #[allow(non_snake_case)]
    /// utility function for checking shader compilation/linking errors.
    /// ------------------------------------------------------------------------
    unsafe fn checkCompileErrors(&self, gl: &gl::Gles2, shader: u32, type_: &str) {
        let mut success = i32::from(gl::FALSE);
        let mut infoLog = Vec::with_capacity(1024);
        infoLog.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                gl.GetShaderInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    infoLog.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                     -- --------------------------------------------------- -- ",
                    type_,
                    str::from_utf8(&infoLog).unwrap()
                );
            }
        } else {
            gl.GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                gl.GetProgramInfoLog(
                    shader,
                    1024,
                    ptr::null_mut(),
                    infoLog.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                     -- --------------------------------------------------- -- ",
                    type_,
                    str::from_utf8(&infoLog).unwrap()
                );
            }
        }
    }
}
