use super::{BufferId, VertexArrayId};
use glow::HasContext;
use std::mem;

pub(crate) struct VertexArrayEntry {
    pub index: u32,
    pub size: i32,
}

pub(crate) fn vao_float_builder(
    gl: &glow::Context,
    entries: &[VertexArrayEntry],
) -> (BufferId, VertexArrayId, BufferId) {
    let (vbo, vao, ebo);

    unsafe {
        // Generate buffers and arrays, as well as attributes.
        vao = gl.create_vertex_array().unwrap();
        vbo = gl.create_buffer().unwrap();
        ebo = gl.create_buffer().unwrap();

        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

        let stride: i32 =
            entries.iter().map(|e| e.size).sum::<i32>() * mem::size_of::<f32>() as i32;

        let mut cumulative_offset: i32 = 0;
        for entry in entries.iter() {
            gl.vertex_attrib_pointer_f32(
                entry.index,
                entry.size,
                glow::FLOAT,
                false,
                stride,
                (cumulative_offset * mem::size_of::<f32>() as i32) as i32,
            );
            gl.enable_vertex_attrib_array(entry.index);
            cumulative_offset += entry.size;
        }
    }

    (vbo, vao, ebo)
}
