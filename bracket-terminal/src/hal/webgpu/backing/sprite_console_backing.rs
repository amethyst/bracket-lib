//! Sprite console mapping for wgpu
use super::index_array_helper::IndexBuffer;
use super::vertex_array_helper::FloatBuffer;
use crate::hal::{Font, Shader, WgpuLink};
use crate::prelude::{RenderSprite, SpriteSheet};
use crate::BResult;
use bracket_color::prelude::RGBA;
use wgpu::{BufferUsages, RenderPipeline};

/// Mapping between a sparse console and wgpu rendering
pub struct SpriteConsoleBackend {
    /// Vertex buffer to use
    vao: FloatBuffer<f32>,
    /// Index buffer to use
    index: IndexBuffer,
    /// WGPU render pipeline
    render_pipeline: RenderPipeline,
}

impl SpriteConsoleBackend {
    /// Create a new sprite console back-end. Called from the main loop's console rebuild.
    pub fn new(wgpu: &WgpuLink, shader: &Shader, font: &Font) -> SpriteConsoleBackend {
        let mut vao = SpriteConsoleBackend::init_buffer_for_console(0);
        let mut index = IndexBuffer::new(0);
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

        SpriteConsoleBackend {
            vao,
            index,
            render_pipeline,
        }
    }

    /// Maps a vertex buffer to the appropriate shader.
    fn init_buffer_for_console(vertex_capacity: usize) -> FloatBuffer<f32> {
        FloatBuffer::<f32>::new(
            &[2, 2, 4, 2, 2], // Pos, XY Transform, Tint, TexPos, Scale
            vertex_capacity,
            BufferUsages::VERTEX,
        )
    }

    /// Helper to push a point to the shader.
    #[allow(clippy::too_many_arguments)]
    fn push_point(
        vertex_buffer: &mut Vec<f32>,
        rel_x: f32,
        rel_y: f32,
        trans_x: f32,
        trans_y: f32,
        fg: RGBA,
        ux: f32,
        uy: f32,
        scale: (f32, f32),
    ) {
        vertex_buffer.extend_from_slice(&[
            rel_x, rel_y, trans_x, trans_y, fg.r, fg.g, fg.b, fg.a, ux, uy, scale.0, scale.1,
        ]);
    }

    /// Helper to build vertices for the sparse grid.
    #[allow(clippy::too_many_arguments)]
    pub fn rebuild_vertices(
        &mut self,
        wgpu: &WgpuLink,
        height: u32,
        width: u32,
        sprites: &[RenderSprite],
        sprite_sheet: &SpriteSheet,
    ) {
        let scale_x = 1.0 / (width as f32 * 0.5);
        let scale_y = 1.0 / (height as f32 * 0.5);

        let offset_x = (width as f32 / 2.0) * scale_x;
        let offset_y = (height as f32 / 2.0) * scale_y;

        self.vao.data.clear();
        self.index.data.clear();

        let mut index_count: u16 = 0;
        for s in sprites.iter() {
            let sprite_sheet = &sprite_sheet;
            let ss_x = 1.0 / sprite_sheet.backing.as_ref().unwrap().width as f32;
            let ss_y = 1.0 / sprite_sheet.backing.as_ref().unwrap().height as f32;
            let sprite_pos = sprite_sheet.sprites[s.index].sheet_location;
            let sprite_left = sprite_pos.x1 as f32 * ss_x;
            let sprite_bottom = sprite_pos.y1 as f32 * ss_y;
            let sprite_right = sprite_pos.x2 as f32 * ss_x;
            let sprite_top = sprite_pos.y2 as f32 * ss_y;

            let render_width = s.destination.width() as f32;
            let sprite_width = sprite_pos.width() as f32;

            let render_height = s.destination.height() as f32;
            let sprite_height = sprite_pos.height() as f32;
            let scale = (
                (render_width / sprite_width) * scale_x,
                (render_height / sprite_height) * scale_y,
            );

            let mut sd = s.destination;
            sd.y2 = height as i32 - s.destination.y1;
            sd.y1 = height as i32 - s.destination.y2;

            SpriteConsoleBackend::push_point(
                &mut self.vao.data,
                0.5,
                0.5,
                (sd.x2 as f32 * scale_x) - offset_x,
                (sd.y2 as f32 * scale_y) - offset_y,
                s.tint,
                sprite_right,
                sprite_top,
                scale,
            );
            SpriteConsoleBackend::push_point(
                &mut self.vao.data,
                0.5,
                -0.5,
                (sd.x2 as f32 * scale_x) - offset_x,
                (sd.y1 as f32 * scale_y) - offset_y,
                s.tint,
                sprite_right,
                sprite_bottom,
                scale,
            );
            SpriteConsoleBackend::push_point(
                &mut self.vao.data,
                -0.5,
                -0.5,
                (sd.x1 as f32 * scale_x) - offset_x,
                (sd.y1 as f32 * scale_y) - offset_y,
                s.tint,
                sprite_left,
                sprite_bottom,
                scale,
            );
            SpriteConsoleBackend::push_point(
                &mut self.vao.data,
                -0.5,
                0.5,
                (sd.x1 as f32 * scale_x) - offset_x,
                (sd.y2 as f32 * scale_y) - offset_y,
                s.tint,
                sprite_left,
                sprite_top,
                scale,
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
