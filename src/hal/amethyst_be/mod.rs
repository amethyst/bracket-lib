// Platform to integrate into Amethyst
use crate::{GameState, Rltk};

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle
    },
    utils::application_root_dir,
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    core::TransformBundle,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

mod keycodes;
pub use keycodes::VirtualKeyCode;

pub struct PlatformGL {}

pub mod shader {
    pub struct Shader{}
}

pub mod font {
    use amethyst::{
        renderer::SpriteSheet,
        renderer::Texture,
        assets::Handle
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
}

pub fn init_raw<S: ToString>(
    width_pixels: u32,
    height_pixels: u32,
    _window_title: S,
) -> crate::Rltk {
    crate::Rltk {
        backend: super::RltkPlatform { platform: PlatformGL{} },
        width_pixels,
        height_pixels,
        fonts: Vec::new(),
        consoles: Vec::new(),
        shaders : Vec::new(),
        fps: 0.0,
        frame_time_ms: 0.0,
        active_console: 0,
        key: None,
        mouse_pos: (0, 0),
        left_click: false,
        shift: false,
        control: false,
        alt: false,
        web_button: None,
        quitting: false,
        post_scanlines: false,
        post_screenburn: false,
    }
}

pub struct RltkGemBridge {
    rltk : Rltk,
    state : Box<dyn GameState>
}

impl SimpleState for RltkGemBridge {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<SimpleConsoleComponent>();
        self.make_camera(world);
        self.initialize_fonts(world);
        self.initialize_console_objects(world);
    }

    /*fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> amethyst::SimpleTrans {
        // Handle Input Somehow

        // Tick the game's state
        self.state.tick(&mut self.rltk);

        // Quit if RLTK wants to (it's my party and I'll quit if I want to)
        if self.rltk.quitting {
            return Trans::Quit;
        }

        // Update the console state objects

        Trans::None
    }*/
}

impl RltkGemBridge {
    fn make_camera(&self, world : &mut World) {
        let mut transform = Transform::default();
        let width = self.rltk.width_pixels as f32;
        let height = self.rltk.height_pixels as f32;
        println!("{} x {}", width, height);
        transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

        world
            .create_entity()
            .with(Camera::standard_2d(width, height))
            .with(transform)
            .build();
    }

    fn initialize_fonts(&mut self, world : &mut World) {
        use amethyst::renderer::Sprite;

        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        let ss_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

        for font in self.rltk.fonts.iter_mut() {
            let handle = loader.load(
                &font.filename,
                ImageFormat::default(),
                (),
                &texture_storage
            );
            
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
                    println!("{:?}", sprite);
                    sprites.push(sprite);
                }
            }

            let ss_handle = loader.load_from_data(
                SpriteSheet{ texture: handle.clone(), sprites },
                (),
                &ss_storage
            );
            font.ss = Some(ss_handle);
            println!("{:?}", font.ss);
        }
    }

    fn initialize_console_objects(&self, world : &mut World) {
        let transform = Transform::default();

        for (i,cons) in self.rltk.consoles.iter().enumerate() {
            if let Some(ss) = &self.rltk.fonts[cons.font_index].ss {
                let sprites = SpriteRender{
                    sprite_sheet: ss.clone(),
                    sprite_number: 1
                };

                world
                    .create_entity()
                    .with(SimpleConsoleComponent{ idx : i})
                    .with(transform.clone())
                    .with(sprites.clone())
                    .build();
                
                    println!("Made console");
            }
        }
    }
}

struct SimpleConsoleComponent {
    idx : usize
}

impl Component for SimpleConsoleComponent {
    type Storage = DenseVecStorage<Self>;
}

pub fn main_loop<GS: GameState>(rltk: Rltk, gamestate: GS) {
    amethyst::start_logger(Default::default());

    let mut cfg = amethyst::window::DisplayConfig::default();
    cfg.dimensions = Some((rltk.width_pixels, rltk.height_pixels));
    cfg.title = "Hello RLTK".to_string();

    let app_root = application_root_dir().unwrap();
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new()).expect("Transform bundle fail")
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config(cfg)
                    .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
            )
            .with_plugin(RenderFlat2D::default()),
        ).expect("Game data fail");
    let assets_dir = app_root;
    //let mut world = World::new(); // Why is this even here?
    let mut game = Application::new(
        assets_dir, 
        RltkGemBridge{rltk, state: Box::new(gamestate)}, 
        game_data)
    .expect("Failed to make game data");
    game.run();
}

pub struct SimpleConsoleBackend {
}

impl SimpleConsoleBackend {
    pub fn new(_gl: &super::RltkPlatform, _width: usize, _height: usize) -> SimpleConsoleBackend {
        SimpleConsoleBackend{}
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &super::RltkPlatform,
        _height: u32,
        _width: u32,
        _tiles: &[crate::Tile],
        _offset_x: f32,
        _offset_y: f32,
    ) {
    }

    pub fn gl_draw(
        &mut self,
        _font: &font::Font,
        _shader: &shader::Shader,
        _platform: &super::RltkPlatform,
        _width: u32,
        _height: u32,
    ) {
    }
}

pub struct SparseConsoleBackend {
}

impl SparseConsoleBackend {
    pub fn new(_gl: &super::RltkPlatform, _width: usize, _height: usize) -> SparseConsoleBackend {
        SparseConsoleBackend{}
    }

    pub fn rebuild_vertices(
        &mut self,
        _platform: &super::RltkPlatform,
        _height: u32,
        _width: u32,
        _offset_x: f32,
        _offset_y: f32,
        _tiles: &[crate::sparse_console::SparseTile],
    ) {
    }

    pub fn gl_draw(
        &mut self,
        _font: &font::Font,
        _shader: &shader::Shader,
        _platform: &super::RltkPlatform,
        _tiles: &[crate::sparse_console::SparseTile],
    ) {
    }
}