//! Provides a map between fancy/flexible consoles and webgpu back-end.

use super::index_array_helper::IndexBuffer;
use super::vertex_array_helper::FloatBuffer;
use crate::hal::{Font, Shader, WgpuLink, scaler::{FontScaler, ScreenScaler}};
use crate::prelude::FlexiTile;
use crate::BResult;
use bracket_color::prelude::RGBA;
use bracket_geometry::prelude::PointF;
use wgpu::{BufferUsages, RenderPipeline};

/// Provides a mapping between fancy/flexible terminals and webgpu.
/// Maintains a vertex buffer of font positions, an index buffer,
/// and its own pipeline.
pub struct FancyConsoleBackend {
    /// The vertex buffer representing how to draw this console.
    vao: FloatBuffer<f32>,
    /// The index buffer representing how to draw this console.
    index: IndexBuffer,
    /// The WGPU render pipeline used to render fancy consoles.
    render_pipeline: RenderPipeline,
}

impl FancyConsoleBackend {
    /// Instantiate a new FancyConsoleBackend. This should be called from the
    /// "rebuild consoles" step of the main loop.
    pub fn new(wgpu: &WgpuLink, shader: &Shader, font: &Font) -> FancyConsoleBackend {
        let mut vao = FancyConsoleBackend::init_buffer_for_console(1000);
        let mut index = IndexBuffer::new(1000);
        vao.build(wgpu);
        index.build(wgpu);

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

        FancyConsoleBackend {
            vao,
            render_pipeline,
            index,
        }
    }

    /// Creates a vertex buffer matching the appropriate shader (fancy.wgsl)
    fn init_buffer_for_console(vertex_capacity: usize) -> FloatBuffer<f32> {
        FloatBuffer::<f32>::new(
            &[3, 4, 4, 2, 3, 2], // Pos, fg, col, tex, rot, scale
            vertex_capacity,
            BufferUsages::VERTEX,
        )
    }

    /// Helper to push a point to the shader.
    #[allow(clippy::too_many_arguments)]
    fn push_point(
        vertex_buffer: &mut Vec<f32>,
        x: f32,
        y: f32,
        fg: RGBA,
        bg: RGBA,
        ux: f32,
        uy: f32,
        rotation: f32,
        screen_x: f32,
        screen_y: f32,
        scale: PointF,
    ) {
        vertex_buffer.extend_from_slice(&[
            x, y, 0.0, fg.r, fg.g, fg.b, fg.a, bg.r, bg.g, bg.b, bg.a, ux, uy, rotation, screen_x,
            screen_y, scale.x, scale.y,
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
        tiles: &[FlexiTile],
        font_scaler: FontScaler,
        screen_scaler: &ScreenScaler,
    ) {
        if tiles.is_empty() {
            return;
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
            let x = t.position.x;
            let y = t.position.y;

            let screen_x = ((step_x * x) + left_x) + offset_x;
            let screen_y = ((step_y * y) + top_y) + offset_y;
            let fg = t.fg;
            let bg = t.bg;
            let glyph = t.glyph;
            let gp = font_scaler.glyph_position(glyph);

            let rot_center_x = screen_x + (step_x / 2.0);
            let rot_center_y = screen_y + (step_y / 2.0);

            FancyConsoleBackend::push_point(
                &mut self.vao.data,
                screen_x + step_x,
                screen_y + step_y,
                fg,
                bg,
                gp.glyph_right,
                gp.glyph_top,
                t.rotation,
                rot_center_x,
                rot_center_y,
                t.scale,
            );
            FancyConsoleBackend::push_point(
                &mut self.vao.data,
                screen_x + step_x,
                screen_y,
                fg,
                bg,
                gp.glyph_right,
                gp.glyph_bottom,
                t.rotation,
                rot_center_x,
                rot_center_y,
                t.scale,
            );
            FancyConsoleBackend::push_point(
                &mut self.vao.data,
                screen_x,
                screen_y,
                fg,
                bg,
                gp.glyph_left,
                gp.glyph_bottom,
                t.rotation,
                rot_center_x,
                rot_center_y,
                t.scale,
            );
            FancyConsoleBackend::push_point(
                &mut self.vao.data,
                screen_x,
                screen_y + step_y,
                fg,
                bg,
                gp.glyph_left,
                gp.glyph_top,
                t.rotation,
                rot_center_x,
                rot_center_y,
                t.scale,
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
    }

    /// Uses WGPU to render the console. Note that it grabs its own accessor
    /// to the backend mutex, so it doesn't need to be passed in - AND CANNOT
    /// BE LOCKED when you call this, or things will deadlock.
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
