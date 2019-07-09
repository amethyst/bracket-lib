use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::Read;
use std::ptr;
use cgmath::{Matrix, Matrix4, Vector3};
use cgmath::prelude::*;
use gl::types::*;
use std::str;
use super::gl;

#[allow(non_snake_case)]
pub struct Shader {
    pub ID: u32,
}

/// NOTE: mixture of `shader_s.h` and `shader_m.h` (the latter just contains
/// a few more setters for uniforms)
#[allow(dead_code)]
#[allow(non_snake_case)]
impl Shader {
    pub fn new(gl : &gl::Gles2, vertexPath: &str, fragmentPath: &str) -> Shader {
        let mut shader = Shader { ID: 0 };
        // 1. retrieve the vertex/fragment source code from filesystem
        let mut vShaderFile = File::open(vertexPath)
            .unwrap_or_else(|_| panic!("Failed to open {}", vertexPath));
        let mut fShaderFile = File::open(fragmentPath)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragmentPath));
        let mut vertexCode = String::new();
        let mut fragmentCode = String::new();
        vShaderFile
            .read_to_string(&mut vertexCode)
            .expect("Failed to read vertex shader");
        fShaderFile
            .read_to_string(&mut fragmentCode)
            .expect("Failed to read fragment shader");

        let vShaderCode = CString::new(vertexCode.as_bytes()).unwrap();
        let fShaderCode = CString::new(fragmentCode.as_bytes()).unwrap();

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl.CreateShader(gl::VERTEX_SHADER);
            gl.ShaderSource(vertex, 1, &vShaderCode.as_ptr(), ptr::null());
            gl.CompileShader(vertex);
            shader.checkCompileErrors(gl, vertex, "VERTEX");
            // fragment Shader
            let fragment = gl.CreateShader(gl::FRAGMENT_SHADER);
            gl.ShaderSource(fragment, 1, &fShaderCode.as_ptr(), ptr::null());
            gl.CompileShader(fragment);
            shader.checkCompileErrors(gl, fragment, "FRAGMENT");
            // shader Program
            let ID = gl.CreateProgram();
            gl.AttachShader(ID, vertex);
            gl.AttachShader(ID, fragment);
            gl.LinkProgram(ID);
            shader.checkCompileErrors(gl, ID, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl.DeleteShader(vertex);
            gl.DeleteShader(fragment);
            shader.ID = ID;
        }

        shader
    }

    /// activate the shader
    /// ------------------------------------------------------------------------
    pub unsafe fn useProgram(&self, gl : &gl::Gles2) {
        gl.UseProgram(self.ID)
    }

    /// utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn setBool(&self, gl : &gl::Gles2, name: &CStr, value: bool) {
        gl.Uniform1i(gl.GetUniformLocation(self.ID, name.as_ptr()), value as i32);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setInt(&self, gl : &gl::Gles2, name: &CStr, value: i32) {
        gl.Uniform1i(gl.GetUniformLocation(self.ID, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setFloat(&self, gl : &gl::Gles2, name: &CStr, value: f32) {
        gl.Uniform1f(gl.GetUniformLocation(self.ID, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setVector3(&self, gl : &gl::Gles2, name: &CStr, value: &Vector3<f32>) {
        gl.Uniform3fv(gl.GetUniformLocation(self.ID, name.as_ptr()), 1, value.as_ptr());
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setVec3(&self, gl : &gl::Gles2, name: &CStr, x: f32, y: f32, z: f32) {
        gl.Uniform3f(gl.GetUniformLocation(self.ID, name.as_ptr()), x, y, z);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn setMat4(&self, gl : &gl::Gles2, name: &CStr, mat: &Matrix4<f32>) {
        gl.UniformMatrix4fv(gl.GetUniformLocation(self.ID, name.as_ptr()), 1, gl::FALSE, mat.as_ptr());
    }

    /// utility function for checking shader compilation/linking errors.
    /// ------------------------------------------------------------------------
    unsafe fn checkCompileErrors(&self, gl : &gl::Gles2, shader: u32, type_: &str) {
        let mut success = gl::FALSE as GLint;
        let mut infoLog = Vec::with_capacity(1024);
        infoLog.set_len(1024 - 1); // subtract 1 to skip the trailing null character
        if type_ != "PROGRAM" {
            gl.GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl.GetShaderInfoLog(shader, 1024, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                println!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&infoLog).unwrap());
            }

        } else {
            gl.GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl.GetProgramInfoLog(shader, 1024, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
                println!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         str::from_utf8(&infoLog).unwrap());
            }
        }
    }
}
