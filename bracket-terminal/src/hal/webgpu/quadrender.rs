//! Helper to render a single full-screen quad

use super::{vertex_array_helper::FloatBuffer, Shader, WgpuLink};
use crate::BResult;
use bracket_color::prelude::RGB;
use wgpu::util::DeviceExt;
use wgpu::{Buffer, BufferUsages, TextureView};

/// Render helper to present the backing buffer to the active
/// surface, including any post-process effects.
pub struct QuadRender {
    quad_buffer: FloatBuffer<f32>,
    pipeline: wgpu::RenderPipeline,
    tex_layout: wgpu::BindGroupLayout,
    uniform_layout: wgpu::BindGroupLayout,
    uniform: QuadUniform,
    uniform_buffer: Buffer,
}

impl QuadRender {
    pub fn new(wgpu: &WgpuLink, shader: &Shader) -> Self {
        // Build the texture bind group
        let texture_bind_group_layout =
            wgpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT,
                            ty: wgpu::BindingType::Sampler {
                                // This is only for TextureSampleType::Depth
                                comparison: false,
                                // This should be true if the sample_type of the texture is:
                                //     TextureSampleType::Float { filterable: true }
                                // Otherwise you'll get an error.
                                filtering: true,
                            },
                            count: None,
                        },
                    ],
                    label: Some("texture_bind_group_layout"),
                });

        // Build the vertex buffer
        let mut quad_buffer = FloatBuffer::new(&[2, 2], 24, BufferUsages::VERTEX);
        quad_buffer.data = vec![
            -1.0, 1.0, 0.0, 0.0, -1.0, -1.0, 0.0, 1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 0.0,
        ];
        quad_buffer.build(wgpu);

        // Build the pipeline
        let uniform_layout =
            wgpu.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: None,
                });

        let render_pipeline_layout =
            wgpu.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&texture_bind_group_layout, &uniform_layout],
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
                    buffers: &[quad_buffer.descriptor()],
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

        let uniform = QuadUniform::empty();
        let uniform_buffer = wgpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Post Process Buffer"),
                contents: bytemuck::cast_slice(&[uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        // Build the result
        Self {
            quad_buffer,
            pipeline: render_pipeline,
            tex_layout: texture_bind_group_layout,
            uniform_layout,
            uniform,
            uniform_buffer,
        }
    }

    pub fn update_uniform(
        &mut self,
        wgpu: &WgpuLink,
        enable_scan_lines: bool,
        enable_screen_burn: bool,
        screen_burn_color: RGB,
    ) {
        // Update the render effects uniform
        self.uniform.enable_scan_lines = if enable_scan_lines { 1.0 } else { 0.0 };
        self.uniform.enable_screen_burn = if enable_screen_burn { 1.0 } else { 0.0 };
        self.uniform.screen_burn_color = [
            screen_burn_color.r,
            screen_burn_color.g,
            screen_burn_color.b,
            1.0,
        ];

        let uniform_buffer = wgpu
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Post Process Buffer"),
                contents: bytemuck::cast_slice(&[self.uniform]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });
        self.uniform_buffer = uniform_buffer;
    }

    pub fn render(&self, wgpu: &WgpuLink, target: &TextureView) -> BResult<()> {
        let mut encoder = wgpu
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            // Recreating because the FB gets rebuilt from time to time
            let bind_group = wgpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.tex_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(wgpu.backing_buffer.view()),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(wgpu.backing_buffer.sampler()),
                    },
                ],
                label: Some("diffuse_bind_group"),
            });

            let uniform_bind_group = wgpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &self.uniform_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(
                        self.uniform_buffer.as_entire_buffer_binding(),
                    ),
                }],
                label: None,
            });

            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &bind_group, &[]);
            render_pass.set_bind_group(1, &uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, self.quad_buffer.slice());
            render_pass.draw(0..6, 0..1);
        }

        // submit will accept anything that implements IntoIter
        wgpu.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct QuadUniform {
    enable_scan_lines: f32,
    enable_screen_burn: f32,
    padding: [f32; 2],
    screen_burn_color: [f32; 4],
}

unsafe impl bytemuck::Pod for QuadUniform {}
unsafe impl bytemuck::Zeroable for QuadUniform {}

impl QuadUniform {
    fn empty() -> Self {
        Self {
            enable_scan_lines: 0.0,
            enable_screen_burn: 0.0,
            padding: [0.0, 0.0],
            screen_burn_color: [0.0, 0.0, 0.0, 1.0],
        }
    }
}
