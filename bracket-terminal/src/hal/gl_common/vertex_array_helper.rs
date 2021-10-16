use super::{BufferId, Font, Shader, VertexArrayId};
use crate::gl_error_wrap;
use crate::hal::BACKEND;
use glow::HasContext;
use std::mem;

#[derive(Debug)]
pub(crate) struct VertexArray {
    vbo: Option<BufferId>,
    vao: Option<VertexArrayId>,
    ebo: Option<BufferId>,
    pub vertex_buffer: Vec<f32>,
    pub index_buffer: Vec<i32>,
}

pub(crate) struct VertexArrayEntry {
    pub index: u32,
    pub size: i32,
}

impl VertexArray {
    pub(crate) fn float_builder(
        gl: &glow::Context,
        entries: &[VertexArrayEntry],
        vertex_capacity: usize,
        index_capacity: usize,
    ) -> Self {
        let mut buffer = VertexArray {
            vbo: None,
            vao: None,
            ebo: None,
            vertex_buffer: Vec::with_capacity(vertex_capacity),
            index_buffer: Vec::with_capacity(index_capacity),
        };
        unsafe {
            // Generate buffers and arrays, as well as attributes.
            gl_error_wrap!(
                gl,
                buffer.vao = Some(gl.create_vertex_array().expect("Unable to create VAO"))
            );
            gl_error_wrap!(
                gl,
                buffer.vbo = Some(gl.create_buffer().expect("Unable to create VBO"))
            );
            gl_error_wrap!(
                gl,
                buffer.ebo = Some(gl.create_buffer().expect("Unable to create EBO"))
            );

            gl.bind_vertex_array(buffer.vao);
            gl.bind_buffer(glow::ARRAY_BUFFER, buffer.vbo);

            let stride: i32 =
                entries.iter().map(|e| e.size).sum::<i32>() * mem::size_of::<f32>() as i32;

            let mut cumulative_offset: i32 = 0;
            for entry in entries.iter() {
                gl_error_wrap!(
                    gl,
                    gl.vertex_attrib_pointer_f32(
                        entry.index,
                        entry.size,
                        glow::FLOAT,
                        false,
                        stride,
                        (cumulative_offset * mem::size_of::<f32>() as i32) as i32,
                    )
                );
                gl_error_wrap!(gl, gl.enable_vertex_attrib_array(entry.index));
                cumulative_offset += entry.size;
            }

            gl_error_wrap!(gl, gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, buffer.ebo));
        }

        buffer
    }

    fn bind(&self, gl: &glow::Context) {
        unsafe {
            gl_error_wrap!(gl, gl.bind_vertex_array(self.vao));
            gl_error_wrap!(gl, gl.bind_buffer(glow::ARRAY_BUFFER, self.vbo));
            gl_error_wrap!(gl, gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, self.ebo));
        }
    }

    pub(crate) fn upload_buffers(&self) {
        let be = BACKEND.lock();
        let gl = be.gl.as_ref().unwrap();
        unsafe {
            self.bind(gl);
            gl_error_wrap!(
                gl,
                gl.buffer_data_u8_slice(
                    glow::ARRAY_BUFFER,
                    self.vertex_buffer.align_to::<u8>().1,
                    glow::STATIC_DRAW,
                )
            );

            gl_error_wrap!(
                gl,
                gl.buffer_data_u8_slice(
                    glow::ELEMENT_ARRAY_BUFFER,
                    self.index_buffer.align_to::<u8>().1,
                    glow::STATIC_DRAW,
                )
            );

            gl_error_wrap!(gl, gl.bind_vertex_array(None));
        }
    }

    pub(crate) fn draw_elements(&self, shader: &Shader, font: &Font) {
        let be = BACKEND.lock();
        let gl = be.gl.as_ref().unwrap();
        unsafe {
            self.bind(gl);
            shader.useProgram(gl);
            font.bind_texture(gl);
            gl_error_wrap!(gl, gl.enable(glow::BLEND));
            gl_error_wrap!(
                gl,
                gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA)
            );
            gl_error_wrap!(
                gl,
                gl.draw_elements(
                    glow::TRIANGLES,
                    self.index_buffer.len() as i32,
                    glow::UNSIGNED_INT,
                    0,
                )
            );
            gl_error_wrap!(gl, gl.disable(glow::BLEND));

            gl_error_wrap!(gl, gl.bind_vertex_array(None));
        }
    }
}
