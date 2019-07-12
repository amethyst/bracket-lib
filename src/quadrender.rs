use gl::types::*;
use super::gl;
use std::os::raw::c_void;
use std::mem;
use std::ptr;

/// Sets up a simple VAO/VBO to render a single quad
/// Used for presenting the backing buffer and in post-process chains.
#[allow(non_snake_case)]
pub fn setup_quad(gl : &gl::Gles2) -> u32 {
    let quadVertices: [f32; 24] = [ // vertex attributes for a quad that fills the entire screen in Normalized Device Coordinates.
        // positions // texCoords
        -1.0,  1.0,  0.0, 1.0,
        -1.0, -1.0,  0.0, 0.0,
        1.0, -1.0,  1.0, 0.0,

        -1.0,  1.0,  0.0, 1.0,
        1.0, -1.0,  1.0, 0.0,
        1.0,  1.0,  1.0, 1.0
    ];
    let (mut quadVAO, mut quadVBO) = (0, 0);
    unsafe {
        gl.GenVertexArrays(1, &mut quadVAO);
        gl.GenBuffers(1, &mut quadVBO);
        gl.BindVertexArray(quadVBO);
        gl.BindBuffer(gl::ARRAY_BUFFER, quadVBO);
        gl.BufferData(gl::ARRAY_BUFFER,
                    (quadVertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                    &quadVertices[0] as *const f32 as *const c_void,
                    gl::STATIC_DRAW);
        gl.EnableVertexAttribArray(0);
        let stride = 4 * mem::size_of::<GLfloat>() as GLsizei;
        gl.VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl.EnableVertexAttribArray(1);
        gl.VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, (2 * mem::size_of::<GLfloat>()) as *const c_void);
    }

    quadVAO
}