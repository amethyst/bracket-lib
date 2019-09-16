use cgmath::{Vector3};
use std::fs::File;
use std::io::Read;
use std::str;
use glow::HasContext;

#[allow(non_snake_case)]
pub struct Shader {
    pub ID: u32,
}

/// NOTE: mixture of `shader_s.h` and `shader_m.h` (the latter just contains
/// a few more setters for uniforms)
impl Shader {
    pub fn new<S: ToString>(
        gl: &glow::Context,
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

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl.create_shader(glow::VERTEX_SHADER).unwrap();
            gl.shader_source(vertex, &vertex_code);
            gl.compile_shader(vertex);

            // fragment Shader
            let fragment = gl.create_shader(glow::FRAGMENT_SHADER).unwrap();
            gl.shader_source(fragment, &fragment_code);
            gl.compile_shader(fragment);

            // shader Program
            let id = gl.create_program().unwrap();
            gl.attach_shader(id, vertex);
            gl.attach_shader(id, fragment);
            gl.link_program(id);

            // delete the shaders as they're linked into our program now and no longer necessary
            shader.ID = id;
        }

        shader
    }

    #[allow(non_snake_case)]
    /// activate the shader
    /// ------------------------------------------------------------------------
    pub unsafe fn useProgram(&self, gl: &glow::Context) {
        gl.use_program(Some(self.ID))
    }

    #[allow(non_snake_case)]
    /// utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn setBool(&self, gl: &glow::Context, name: &str, value: bool) {
        gl.uniform_1_i32(gl.get_uniform_location(self.ID, name), value as i32);
    }

    #[allow(non_snake_case)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setInt(&self, gl: &glow::Context, name: &str, value: i32) {
        gl.uniform_1_i32(gl.get_uniform_location(self.ID, name), value);
    }

    #[allow(non_snake_case)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setFloat(&self, gl: &glow::Context, name: &str, value: f32) {
        gl.uniform_1_f32(gl.get_uniform_location(self.ID, name), value);
    }

    #[allow(non_snake_case)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setVector3(&self, gl: &glow::Context, name: &str, value: &Vector3<f32>) {
        gl.uniform_3_f32(
            gl.get_uniform_location(self.ID, name),
            value.x,
            value.y,
            value.z
        );
    }

    #[allow(non_snake_case)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setVec3(&self, gl: &glow::Context, name: &str, x: f32, y: f32, z: f32) {
        gl.uniform_3_f32(
            gl.get_uniform_location(self.ID, name),
            x,
            y,
            z
        );
    }

    #[allow(non_snake_case)]
    /// ------------------------------------------------------------------------
    pub unsafe fn setMat4(&self, gl: &glow::Context, name: &str, mat: &[f32; 16]) {
        gl.uniform_matrix_4_f32_slice(
            gl.get_uniform_location(self.ID, name),
            false,
            mat
        );
    }
}
