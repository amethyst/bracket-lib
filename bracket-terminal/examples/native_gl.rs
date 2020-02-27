use bracket_terminal::prelude::*;
use glow::HasContext;
use std::mem;

bracket_terminal::add_wasm_support!();

#[cfg(not(target_arch = "wasm32"))]
const VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

#[cfg(not(target_arch = "wasm32"))]
const FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

#[cfg(target_arch = "wasm32")]
const VERTEX_SHADER_SOURCE: &str = r#"#version 300 es
    precision mediump float;
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

#[cfg(target_arch = "wasm32")]
const FRAGMENT_SHADER_SOURCE: &str = r#"#version 300 es
    precision mediump float;
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

fn gl_setup(gl : &glow::Context, state : &mut State) {
    state.my_shader = Some(
        Shader::new(gl, VERTEX_SHADER_SOURCE, FRAGMENT_SHADER_SOURCE)
    );

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // left
         0.5, -0.5, 0.0, // right
         0.0,  0.5, 0.0  // top
    ];
    unsafe {
        let vao = gl.create_vertex_array().unwrap();
        let vbo = gl.create_buffer().unwrap();

        gl.bind_vertex_array(Some(vao));
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            &vertices.align_to::<u8>().1,
            glow::STATIC_DRAW,
        );

        let stride = 3 * mem::size_of::<f32>() as i32;
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, stride, 0);
        gl.enable_vertex_attrib_array(0);

        state.vao = Some(vao);
        state.vbo = Some(vbo);

        gl.bind_buffer(glow::ARRAY_BUFFER, None);
        gl.bind_vertex_array(None);
    }
}

fn gl_render(gs : &mut dyn std::any::Any, gl : &glow::Context) {
    let state = gs.downcast_ref::<State>().unwrap();
    unsafe {
        state.my_shader.as_ref().unwrap().useProgram(gl);
        gl.bind_vertex_array(state.vao);
        gl.draw_arrays(glow::TRIANGLES, 0, 3);
        gl.bind_vertex_array(None);
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct State {
    setup_gl : bool,
    my_shader : Option<Shader>,
    vao : Option<u32>,
    vbo : Option<u32>
}

#[cfg(target_arch = "wasm32")]
struct State {
    setup_gl : bool,
    my_shader : Option<Shader>,
    vao : Option<glow::WebVertexArrayKey>,
    vbo : Option<glow::WebBufferKey>
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        if !self.setup_gl {
            let mut be = BACKEND.lock().unwrap();
            let gl = be.gl.as_ref().unwrap();
                self.setup_gl = true;

            gl_setup(gl, self);
            be.gl_callback = Some(gl_render);
        }

        ctx.print(1,1,"Hello, the triangle is a native OpenGL call.");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Native GL!")
        .build()?;

    let gs: State = State {
        setup_gl : false,
        my_shader : None,
        vao : None,
        vbo : None
    };

    main_loop(context, gs)
}
