use super::gl;
use gl::types::*;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

/// Sets up a simple VAO/VBO to render a single quad
/// Used for presenting the backing buffer and in post-process chains.
pub fn setup_quad(gl: &gl::Gles2) -> u32 {
    let quad_vertices: [f32; 24] = [
        // vertex attributes for a quad that fills the entire screen in Normalized Device Coordinates.
        // positions // texCoords
        -1.0, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0,
        -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
    ];
    let (mut quad_vao, mut quad_vbo) = (0, 0);
    unsafe {
        gl.GenVertexArrays(1, &mut quad_vao);
        gl.GenBuffers(1, &mut quad_vbo);
        gl.BindVertexArray(quad_vbo);
        gl.BindBuffer(gl::ARRAY_BUFFER, quad_vbo);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (quad_vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
            &quad_vertices[0] as *const f32 as *const c_void,
            gl::STATIC_DRAW,
        );
        gl.EnableVertexAttribArray(0);
        let stride = 4 * mem::size_of::<GLfloat>() as GLsizei;
        gl.VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl.EnableVertexAttribArray(1);
        gl.VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride,
            (2 * mem::size_of::<GLfloat>()) as *const c_void,
        );
    }

    quad_vao
}
