//! WGPU Definition of a font
use bracket_embedding::prelude::EMBED;
use crate::BResult;
use bracket_color::prelude::RGB;
use image::GenericImageView;
use wgpu::{BindGroup, BindGroupLayout, Sampler, TextureView};

use super::WgpuLink;

/// BTerm's representation of a font or tileset file.
pub struct Font {
    pub bitmap_file: String,
    pub width: u32,
    pub height: u32,

    pub texture_id: Option<usize>,

    pub tile_size: (u32, u32),
    pub explicit_background: Option<RGB>,
    pub font_dimensions_glyphs: (u32, u32),
    pub font_dimensions_texture: (f32, f32),

    pub view: Option<TextureView>,
    pub sampler: Option<Sampler>,
    pub bind_group: Option<BindGroup>,
    pub bind_group_layout: Option<BindGroupLayout>,
}

#[allow(non_snake_case)]
impl Font {
    /// Creates an unloaded texture with filename and size parameters provided.
    pub fn new<S: ToString>(filename: S, width: u32, height: u32, tile_size: (u32, u32)) -> Font {
        Font {
            bitmap_file: filename.to_string(),
            width,
            height,
            texture_id: None,
            tile_size,
            explicit_background: None,
            font_dimensions_glyphs: (tile_size.0 / width, tile_size.1 / height),
            font_dimensions_texture: ( tile_size.0 as f32 / width as f32, tile_size.1 as f32 / height as f32 ),
            view: None,
            sampler: None,
            bind_group: None,
            bind_group_layout: None,
        }
    }

    fn load_image(filename: &str) -> image::DynamicImage {
        let resource = EMBED.lock().get_resource(filename.to_string());
        match resource {
            None => image::open(std::path::Path::new(&filename.to_string()))
                .expect("Failed to load texture"),
            Some(res) => image::load_from_memory(res).expect("Failed to load texture from memory"),
        }
    }

    /// Loads a font file (texture) to obtain the width and height for you
    pub fn load<S: ToString>(
        filename: S,
        tile_size: (u32, u32),
        explicit_background: Option<RGB>,
    ) -> Font {
        let img = Font::load_image(&filename.to_string());
        Font {
            bitmap_file: filename.to_string(),
            width: img.width(),
            height: img.height(),
            texture_id: None,
            tile_size,
            explicit_background,
            font_dimensions_glyphs: (img.width() / tile_size.0, img.height() / tile_size.1),
            font_dimensions_texture: ( tile_size.0 as f32 / img.width() as f32, tile_size.1 as f32 / img.height() as f32),
            view: None,
            sampler: None,
            bind_group: None,
            bind_group_layout: None,
        }
    }

    /// Load a font, and allocate it as an OpenGL resource. Returns the OpenGL binding number (which is also set in the structure).
    pub fn setup_wgpu_texture(&mut self, wgpu: &WgpuLink) -> BResult<usize> {
        let texture = 0;

        // Ensure image is in the correct orientation and handle explicit backgrounds
        let img_orig = Font::load_image(&self.bitmap_file);
        let w = img_orig.width() as i32;
        let h = img_orig.height() as i32;
        self.width = w as u32;
        self.height = h as u32;
        let img_flip = img_orig.flipv();
        let img = img_flip.to_rgba8();
        let mut data = img.into_raw();
        if let Some(bg_rgb) = self.explicit_background {
            let bg_r = (bg_rgb.r * 255.0) as u8;
            let bg_g = (bg_rgb.g * 255.0) as u8;
            let bg_b = (bg_rgb.b * 255.0) as u8;
            let len = data.len() / 4;
            for i in 0..len {
                let idx = i * 4;
                if data[idx] == bg_r && data[idx + 1] == bg_g && data[idx + 2] == bg_b {
                    data[idx] = 0;
                    data[idx + 1] = 0;
                    data[idx + 2] = 0;
                    data[idx + 3] = 0;
                }
            }
        }

        // Setup the WGPU texture
        let texture_size = wgpu::Extent3d {
            width: self.width,
            height: self.height,
            depth_or_array_layers: 1,
        };
        let tex = wgpu.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            label: None,
        });
        wgpu.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * self.width),
                rows_per_image: std::num::NonZeroU32::new(self.height),
            },
            texture_size,
        );

        // Build view and sampler
        let texture_view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = wgpu.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

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

        let bind_group = wgpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        self.view = Some(texture_view);
        self.sampler = Some(sampler);
        self.bind_group_layout = Some(texture_bind_group_layout);
        self.bind_group = Some(bind_group);

        Ok(texture)
    }

    // Sets this font file as the active texture
    /*pub fn bind_texture(&self, gl: &glow::Context) {
        unsafe {
            gl.bind_texture(glow::TEXTURE_2D, self.gl_id);
            gl_error(gl);
        }
    }*/
}

// Some unit testing for fonts

#[cfg(test)]
mod tests {
    use super::Font;

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    fn make_font_minimal() {
        let f = Font::new("test.png", 1, 2, (8, 8));
        assert_eq!(f.bitmap_file, "test.png");
        assert_eq!(f.width, 1);
        assert_eq!(f.height, 2);
    }

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    fn make_font_from_file() {
        let f = Font::load("resources/terminal8x8.png", (8, 8), None);
        assert_eq!(f.width, 128);
        assert_eq!(f.height, 128);
    }
}
