//! Provides a wgpu mapping to the sparse consoele
use super::index_array_helper::IndexBuffer;
use super::vertex_array_helper::FloatBuffer;
use crate::hal::{Font, Shader, WgpuLink, scaler::{FontScaler, ScreenScaler}};
use crate::prelude::SparseTile;
use crate::BResult;
use bracket_color::prelude::RGBA;
use wgpu::{BufferUsages, RenderPipeline};

/// Maps the Sparse Console type to a wgpu back-end.
pub struct SparseConsoleBackend {
    /// Vertex buffer to use
    vao: FloatBuffer<f32>,
    /// Index buffer to use
    index: IndexBuffer,
    /// WGPU Render Pipeline to use
    render_pipeline: RenderPipeline,
    /// No change optimization
    previous_console : Option<Vec<SparseTile>>,
}

impl SparseConsoleBackend {
    /// Creates a new sparse console back-end, called from mainloop's rebuild consoles.
    pub fn new(wgpu: &WgpuLink, shader: &Shader, font: &Font) -> SparseConsoleBackend {
        let vao = SparseConsoleBackend::init_buffer_for_console(1000);
        let index = IndexBuffer::new(1000);

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

        SparseConsoleBackend {
            vao,
            index,
            render_pipeline,
            previous_console: None,
        }
    }

    /// Creates a vertex buffer with the right mappings to use the sparse console shader.
    fn init_buffer_for_console(vertex_capacity: usize) -> FloatBuffer<f32> {
        FloatBuffer::<f32>::new(
            &[3, 4, 4, 2], // Pos, FG, BG, TexPos
            vertex_capacity,
            BufferUsages::VERTEX,
        )
    }

    /// Helper to push a point to the shader.
    fn push_point(
        vertex_buffer: &mut Vec<f32>,
        x: f32,
        y: f32,
        fg: RGBA,
        bg: RGBA,
        ux: f32,
        uy: f32,
    ) {
        vertex_buffer.extend_from_slice(&[
            x, y, 0.0, fg.r, fg.g, fg.b, fg.a, bg.r, bg.g, bg.b, bg.a, ux, uy,
        ]);
    }

    /// Helper to build vertices for the sparse grid.
    #[allow(clippy::too_many_arguments)]
    pub fn rebuild_vertices(
        &mut self,
        wgpu: &WgpuLink,
        height: u32,
        width: u32,
        offset_x: f32,
        offset_y: f32,
        scale: f32,
        scale_center: (i32, i32),
        tiles: &Vec<SparseTile>,
        font_scaler: FontScaler,
        screen_scaler: &ScreenScaler,
        must_resize: bool,
    ) {
        if !must_resize {
            if let Some(old) = &self.previous_console {
                if old.len() == tiles.len() {
                    let no_change = tiles.iter().zip(old.iter()).all(|(a, b)| *a==*b);
                    if no_change {
                        return;
                    }
                }
            }
        }

        self.vao.data.clear();
        self.index.data.clear();

        let (step_x, step_y, left_x, top_y) = {
            let (step_x, step_y) = screen_scaler.calc_step(width, height, scale);
            let (left_x, top_y) = screen_scaler.top_left_pixel();

            (step_x, step_y, left_x, top_y)
        };

        //let step_x: f32 = scale * 2.0 / width as f32;
        //let step_y: f32 = scale * 2.0 / height as f32;

        let mut index_count: u16 = 0;
        //let screen_x_start: f32 = -1.0 * scale
        //    - 2.0 * (scale_center.0 - width as i32 / 2) as f32 * (scale - 1.0) / width as f32;
        //let screen_y_start: f32 = -1.0 * scale
        //    + 2.0 * (scale_center.1 - height as i32 / 2) as f32 * (scale - 1.0) / height as f32;
        for t in tiles.iter() {
            let x = t.idx % width as usize;
            let y = t.idx / width as usize;

            let screen_x = ((step_x * x as f32) + left_x) + offset_x;
            let screen_y = ((step_y * y as f32) + top_y) + offset_y;
            let fg = t.fg;
            let bg = t.bg;
            let glyph = t.glyph;
            let gp = font_scaler.glyph_position(glyph);

            SparseConsoleBackend::push_point(
                &mut self.vao.data,
                screen_x + step_x,
                screen_y + step_y,
                fg,
                bg,
                gp.glyph_right,
                gp.glyph_top,
            );
            SparseConsoleBackend::push_point(
                &mut self.vao.data,
                screen_x + step_x,
                screen_y,
                fg,
                bg,
                gp.glyph_right,
                gp.glyph_bottom,
            );
            SparseConsoleBackend::push_point(
                &mut self.vao.data,
                screen_x,
                screen_y,
                fg,
                bg,
                gp.glyph_left,
                gp.glyph_bottom,
            );
            SparseConsoleBackend::push_point(
                &mut self.vao.data,
                screen_x,
                screen_y + step_y,
                fg,
                bg,
                gp.glyph_left,
                gp.glyph_top,
            );

            self.index.data.push(index_count);
            self.index.data.push(1 + index_count);
            self.index.data.push(3 + index_count);
            self.index.data.push(1 + index_count);
            self.index.data.push(2 + index_count);
            self.index.data.push(3 + index_count);

            index_count += 4;
        }

        self.vao.build(wgpu);
        self.index.build(wgpu);
        self.previous_console = Some(tiles.clone());
    }

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
