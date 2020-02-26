use crate::prelude::BTerm;
use crate::Result;
use amethyst::{
    assets::Handle,
    assets::{AssetStorage, Loader},
    ecs::prelude::*,
    renderer::{ImageFormat, SpriteSheet, Texture},
    utils::application_root_dir,
};

pub struct Font {
    pub tile_size: (u32, u32),
    pub filename: String,
    pub ss: Option<Handle<SpriteSheet>>,
}

impl Font {
    pub fn load<S: ToString>(filename: S, tile_size: (u32, u32)) -> Font {
        Font {
            tile_size,
            filename: filename.to_string(),
            ss: None,
        }
    }

    pub fn setup_gl_texture(&mut self, _gl: &crate::hal::BTermPlatform) -> Result<()> {
        Ok(())
    }

    pub fn bind_texture(&self, _gl: &crate::hal::BTermPlatform) {}
}

pub fn initialize_fonts(bterm: &mut BTerm, world: &mut World) -> Result<()> {
    use crate::embedding;
    use amethyst::renderer::rendy::texture::TextureBuilder;
    use amethyst::renderer::types::TextureData;
    use amethyst::renderer::Format;
    use amethyst::renderer::Sprite;

    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let ss_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    let app_root = application_root_dir().expect("Fail");
    use amethyst::renderer::rendy::*;
    use image::GenericImageView;

    for font in bterm.fonts.iter_mut() {
        let resource = embedding::EMBED
            .lock()?
            .get_resource(font.filename.to_string());

        let handle;
        if let Some(data) = resource {
            let png = image::load_from_memory(data).expect("Failed to load texture from memory");

            // This sets black pixels to be transparent
            const MIN_VAL: u8 = 10;
            let mut raw_pixels = png.raw_pixels().clone();
            for i in 0..raw_pixels.len() / 4 {
                if raw_pixels[(i * 4)] < MIN_VAL
                    && raw_pixels[(i * 4) + 1] < MIN_VAL
                    && raw_pixels[(i * 4) + 2] < MIN_VAL
                {
                    raw_pixels[(i * 4) + 3] = 0; // Make it transparent
                }
            }

            let texture_builder = TextureBuilder::new()
                .with_data_width(png.width())
                .with_data_height(png.height())
                .with_kind(hal::image::Kind::D2(png.width(), png.height(), 1, 1))
                .with_view_kind(hal::image::ViewKind::D2)
                //.with_sampler_info(hal::image::SamplerInfo::new(hal::image::Filter::Nearest, hal::image::WrapMode::Clamp))
                .with_sampler_info(hal::image::SamplerInfo {
                    min_filter: hal::image::Filter::Nearest,
                    mag_filter: hal::image::Filter::Nearest,
                    mip_filter: hal::image::Filter::Nearest,
                    wrap_mode: (
                        hal::image::WrapMode::Clamp,
                        hal::image::WrapMode::Clamp,
                        hal::image::WrapMode::Clamp,
                    ),
                    lod_bias: 0.0.into(),
                    lod_range: std::ops::Range {
                        start: 0.0.into(),
                        end: 1000.0.into(),
                    },
                    comparison: None,
                    border: hal::image::PackedColor(0),
                    anisotropic: hal::image::Anisotropic::Off,
                    normalized: true,
                })
                .with_raw_data(raw_pixels, Format::Rgba8Srgb);
            handle = loader.load_from_data(TextureData(texture_builder), (), &texture_storage);
        } else {
            let filename = app_root.join(font.filename.clone());
            handle = loader.load(
                filename
                    .to_str()
                    .ok_or("Couldn't convert filename to string")?,
                ImageFormat::default(),
                (),
                &texture_storage,
            );
        }

        // Make a font-specific sprite sheet
        let offsets = [
            0.0 - (font.tile_size.0 as f32 / 2.0),
            0.0 - (font.tile_size.1 as f32 / 2.0),
        ];
        let mut sprites = Vec::with_capacity(256);

        for y in 0..16 {
            for x in 0..16 {
                let sprite = Sprite::from_pixel_values(
                    font.tile_size.0 * 16,
                    font.tile_size.1 * 16,
                    font.tile_size.0,
                    font.tile_size.1,
                    x * font.tile_size.0,
                    y * font.tile_size.1,
                    offsets,
                    false,
                    false,
                );
                sprites.push(sprite);
            }
        }

        let ss_handle = loader.load_from_data(
            SpriteSheet {
                texture: handle.clone(),
                sprites,
            },
            (),
            &ss_storage,
        );
        font.ss = Some(ss_handle);
    }
    Ok(())
}
