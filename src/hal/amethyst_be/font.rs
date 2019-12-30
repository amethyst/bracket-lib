use crate::Rltk;
use amethyst::{
    assets::Handle,
    ecs::prelude::*,
    assets::{AssetStorage, Loader},
    utils::application_root_dir,
    renderer::{ImageFormat, SpriteSheet, Texture},
};

pub struct Font{
    pub tile_size: (u32, u32),
    pub filename : String,
    pub ss : Option<Handle<SpriteSheet>>
}

impl Font {
    pub fn load<S: ToString>(filename: S, tile_size: (u32, u32)) -> Font {
        Font{
            tile_size,
            filename : filename.to_string(),
            ss : None
        }
    }

    pub fn setup_gl_texture(&mut self, _gl: &crate::hal::RltkPlatform) {

    }

    pub fn bind_texture(&self, _gl: &crate::hal::RltkPlatform) {

    }
}

pub fn initialize_fonts(rltk: &mut Rltk, world : &mut World) {
    use amethyst::renderer::Sprite;
    use amethyst::renderer::types::{TextureData};
    use amethyst::renderer::rendy::texture::TextureBuilder;
    use crate::embedding;
    use amethyst::renderer::Format;

    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    let ss_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    let app_root = application_root_dir().expect("Fail");
    use image::GenericImageView;
    use amethyst::renderer::rendy::*;

    for font in rltk.fonts.iter_mut() {
        let resource = embedding::EMBED
            .lock()
            .unwrap()
            .get_resource(font.filename.to_string());

            let handle;
        if let Some(data) = resource {
            let png = image::load_from_memory(data).expect("Failed to load texture from memory");
            let texture_builder = TextureBuilder::new()
                .with_data_width(png.width())
                .with_data_height(png.height())
                .with_kind(hal::image::Kind::D2(png.width(), png.height(), 1, 1))
                .with_view_kind(hal::image::ViewKind::D2)
                .with_sampler_info(hal::image::SamplerInfo::new(hal::image::Filter::Nearest, hal::image::WrapMode::Clamp))
                .with_raw_data(png.raw_pixels(), Format::Rgba8Unorm);
            handle = loader.load_from_data(TextureData(texture_builder), (), &texture_storage);
        } else {
            let filename = app_root.join(font.filename.clone());
            handle = loader.load(
                filename.to_str().unwrap(),
                ImageFormat::default(),
                (),
                &texture_storage
            );
        }

        // Make a font-specific sprite sheet
        let offsets = [0.0 - (font.tile_size.0 as f32 / 2.0), 0.0 - (font.tile_size.1 as f32 / 2.0)];
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
                    false
                );
                sprites.push(sprite);
            }
        }

        let ss_handle = loader.load_from_data(
            SpriteSheet{ texture: handle.clone(), sprites },
            (),
            &ss_storage
        );
        font.ss = Some(ss_handle);
    }
}