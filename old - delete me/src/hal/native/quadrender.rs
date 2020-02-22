use glow::HasContext;
use std::mem;

/// Sets up a simple VAO/VBO to render a single quad
/// Used for presenting the backing buffer and in post-process chains.
pub fn setup_quad(gl: &glow::Context) -> u32 {
    let quad_vertices: [f32; 24] = [
        // vertex attributes for a quad that fills the entire screen in Normalized Device Coordinates.
        // positions // texCoords
        -1.0, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0, 1.0,
        -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
    ];
    let (quad_vao, quad_vbo);
    unsafe {
        quad_vao = gl.create_vertex_array().unwrap();
        quad_vbo = gl.create_buffer().unwrap();
        gl.bind_vertex_array(Some(quad_vbo));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(quad_vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            &quad_vertices.align_to::<u8>().1,
            glow::STATIC_DRAW,
        );
        gl.enable_vertex_attrib_array(0);
        let stride = 4 * mem::size_of::<f32>() as i32;
        gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(1);
        gl.vertex_attrib_pointer_f32(
            1,
            2,
            glow::FLOAT,
            false,
            stride,
            2 * mem::size_of::<f32>() as i32,
        );
    }

    quad_vao
}
