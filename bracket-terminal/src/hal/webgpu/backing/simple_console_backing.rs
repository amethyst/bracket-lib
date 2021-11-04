//! Maps a Simple Console to WGPU backing

use super::index_array_helper::IndexBuffer;
use super::vertex_array_helper::FloatBuffer;
use crate::hal::{Font, Shader, WgpuLink, scaler::{FontScaler, ScreenScaler}};
use crate::prelude::Tile;
use crate::BResult;
use bracket_color::prelude::RGBA;
use wgpu::{BufferUsages, RenderPipeline};

/// Provide WGPU rendering services for a Simple Console
pub struct SimpleConsoleBackend {
    /// The vertex buffer
    vao: FloatBuffer<f32>,
    /// The index buffer
    index: IndexBuffer,
    /// # elements in the index
    index_counter: usize,
    /// # elements in the vertex list
    vertex_counter: usize,
    /// The WGPU render pipeline
    render_pipeline: RenderPipeline,
    /// For dirty optimization
    previous_console: Option<Vec<Tile>>,
}

impl SimpleConsoleBackend {
    /// Create a new simple console backend. Call this from the main loop's
    /// console rebuild.
    pub fn new(
        width: usize,
        height: usize,
        wgpu: &WgpuLink,
        shader: &Shader,
        font: &Font,
    ) -> SimpleConsoleBackend {
        let vertex_capacity: usize = (13 * width as usize * height as usize) * 4;
        let index_capacity: usize = 6 * width as usize * height as usize;
        let mut vao = SimpleConsoleBackend::init_buffer_for_console(vertex_capacity);
        vao.data = vec![0.0f32; vertex_capacity];
        let index = IndexBuffer::new(index_capacity);

        // Setup the pipeline
        let render_pipeline_layout =
            wgpu.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[font.bind_group_layout.as_ref().unwrap()],
                    push_constant_ranges: &[],
                });
        let render_pipeline = wgpu
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader.0,
                    entry_point: "main",
                    buffers: &[vao.descriptor()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader.0,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: wgpu.config.format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    //cull_mode: Some(wgpu::Face::Back),
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    clamp_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
            });

        // Build the result
        let result = SimpleConsoleBackend {
            vao,
            index,
            vertex_counter: 0,
            index_counter: 0,
            render_pipeline,
            previous_console: None,
        };
        result
    }

    /// Creates a buffer definition mapped to the simple console shader.
    fn init_buffer_for_console(vertex_capacity: usize) -> FloatBuffer<f32> {
        FloatBuffer::<f32>::new(
            &[3, 4, 4, 2], // Pos, FG, BG, TexPos
            vertex_capacity,
            BufferUsages::VERTEX,
        )
    }

    /// Helper function to add all the elements required by the shader for a given point.
    #[allow(clippy::too_many_arguments)]
    fn push_point(
        &mut self,
        x: f32,
        y: f32,
        fg: RGBA,
        bg: RGBA,
        ux: f32,
        uy: f32,
        offset_x: f32,
        offset_y: f32,
    ) {
        self.vao.data[self.vertex_counter] = x + offset_x;
        self.vao.data[self.vertex_counter + 1] = y + offset_y;
        self.vao.data[self.vertex_counter + 2] = 0.0f32;
        self.vao.data[self.vertex_counter + 3] = fg.r;
        self.vao.data[self.vertex_counter + 4] = fg.g;
        self.vao.data[self.vertex_counter + 5] = fg.b;
        self.vao.data[self.vertex_counter + 6] = fg.a;
        self.vao.data[self.vertex_counter + 7] = bg.r;
        self.vao.data[self.vertex_counter + 8] = bg.g;
        self.vao.data[self.vertex_counter + 9] = bg.b;
        self.vao.data[self.vertex_counter + 10] = bg.a;
        self.vao.data[self.vertex_counter + 11] = ux;
        self.vao.data[self.vertex_counter + 12] = uy;
        self.vertex_counter += 13;
    }

    /// Rebuilds the OpenGL backing buffer.
    #[allow(clippy::too_many_arguments)]
    pub fn rebuild_vertices(
        &mut self,
        wgpu: &WgpuLink,
        height: u32,
        width: u32,
        tiles: &Vec<Tile>,
        offset_x: f32,
        offset_y: f32,
        scale: f32,
        scale_center: (i32, i32),
        needs_resize: bool,
        font_scaler: FontScaler,
        screen_scaler: &ScreenScaler,
    ) {
        /*if !needs_resize {
            if let Some(old) = &self.previous_console {
                if old.len() == tiles.len() {
                    let no_change = tiles.iter().zip(old.iter()).all(|(a, b)| *a==*b);
                    if no_change {
                        return;
                    }
                }
            }
        }*/

        if needs_resize {
            let vertex_capacity: usize = (13 * width as usize * height as usize) * 4;
            let index_capacity: usize = 6 * width as usize * height as usize;
            self.vao.data.clear();
            self.vao.data.resize(vertex_capacity, 0.0);
            self.index.data.clear();
            self.index.data.resize(index_capacity, 0);
        }

        self.vertex_counter = 0;
        self.index_counter = 0;

        let (step_x, step_y, left_x, top_y) = {
            let (step_x, step_y) = screen_scaler.calc_step(width, height, scale);
            let (left_x, top_y) = screen_scaler.top_left_pixel();

            (step_x, step_y, left_x, top_y)
        };

        //let step_x: f32 = scale * 2.0f32 / width as f32;
        //let step_y: f32 = scale * 2.0f32 / height as f32;

        //let mut screen_y: f32 = -1.0 * scale
        //    + 2.0 * (scale_center.1 - height as i32 / 2) as f32 * (scale - 1.0) / height as f32;
        let mut index_count: u16 = 0;
        let mut screen_y = top_y;
        for y in 0..height {
            let mut screen_x = left_x;
            for x in 0..width {
                let fg = tiles[((y * width) + x) as usize].fg;
                let bg = tiles[((y * width) + x) as usize].bg;
                let glyph = tiles[((y * width) + x) as usize].glyph;
                let gp = font_scaler.glyph_position(glyph);

                self.push_point(
                    screen_x + step_x,
                    screen_y + step_y,
                    fg,
                    bg,
                    gp.glyph_right,
                    gp.glyph_top,
                    offset_x,
                    offset_y,
                );
                self.push_point(
                    screen_x + step_x,
                    screen_y,
                    fg,
                    bg,
                    gp.glyph_right,
                    gp.glyph_bottom,
                    offset_x,
                    offset_y,
                );
                self.push_point(
                    screen_x,
                    screen_y,
                    fg,
                    bg,
                    gp.glyph_left,
                    gp.glyph_bottom,
                    offset_x,
                    offset_y,
                );
                self.push_point(
                    screen_x,
                    screen_y + step_y,
                    fg,
                    bg,
                    gp.glyph_left,
                    gp.glyph_top,
                    offset_x,
                    offset_y,
                );

                self.index.data[self.index_counter] = index_count;
                self.index.data[self.index_counter + 1] = 1 + index_count;
                self.index.data[self.index_counter + 2] = 3 + index_count;
                self.index.data[self.index_counter + 3] = 1 + index_count;
                self.index.data[self.index_counter + 4] = 2 + index_count;
                self.index.data[self.index_counter + 5] = 3 + index_count;
                self.index_counter += 6;

                index_count += 4;

                screen_x += step_x;
            }
            screen_y += step_y;
        }

        self.vao.build(wgpu);
        self.index.build(wgpu);
        self.previous_console = Some(tiles.clone());
    }

    /// Renders the console via wgpu.
    pub fn wgpu_draw(&mut self, font: &Font) -> BResult<()> {
        use crate::hal::BACKEND;
        let mut be = BACKEND.lock();
        if let Some(wgpu) = be.wgpu.as_mut() {
            let mut encoder = wgpu
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: wgpu.backing_buffer.view(),
                        resolve_target: None,
                        ops: wgpu::Operations {
                            /*load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 0.0,
                                g: 0.0,
                                b: 0.0,
                                a: 1.0,
                            }),*/
                            load: wgpu::LoadOp::Load,
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
                render_pass.set_pipeline(&self.render_pipeline);
                render_pass.set_bind_group(0, font.bind_group.as_ref().unwrap(), &[]);
                render_pass.set_vertex_buffer(0, self.vao.slice());
                render_pass.set_index_buffer(self.index.slice(), wgpu::IndexFormat::Uint16);
                render_pass.draw_indexed(0..self.index.len(), 0, 0..1);
            }

            // submit will accept anything that implements IntoIter
            wgpu.queue.submit(std::iter::once(encoder.finish()));
            //output.present();
        }
        Ok(())
    }
}
