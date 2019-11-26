use crate::{GameState, Rltk};
use super::tiles::SimpleConsoleTile;

use amethyst::{
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    core::TransformBundle,
    renderer::{Camera, ImageFormat, SpriteSheet, Texture},
    tiles::{MortonEncoder2D, RenderTiles2D, TileMap},
    core::math::{Vector3},
    input::{InputBundle, StringBindings, Bindings, InputHandler, Button},
    winit::MouseButton
};

pub struct RltkGemBridge {
    rltk : Rltk,
    state : Box<dyn GameState>,
    key_delay : f32
}

impl SimpleState for RltkGemBridge {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.make_camera(world);
        self.initialize_fonts(world);
        self.initialize_console_objects(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> amethyst::SimpleTrans {
        // Frame times
        let timer = data.world.fetch::<amethyst::core::Time>();
        self.rltk.frame_time_ms = timer.delta_time().as_millis() as f32;
        self.rltk.fps = 1.0 / timer.delta_seconds();
        self.key_delay += self.rltk.frame_time_ms;
        std::mem::drop(timer);

        // Handle Input
        self.rltk.left_click = false;
        self.rltk.key = None;
        self.rltk.shift = false;
        self.rltk.control = false;
        self.rltk.alt = false;
        let inputs = data.world.fetch::<InputHandler<StringBindings>>();
        if self.key_delay > 50.0 {
            self.key_delay = 0.0;
            for key in inputs.keys_that_are_down() {
                use crate::VirtualKeyCode;
                match key {
                    VirtualKeyCode::LShift => self.rltk.shift = true,
                    VirtualKeyCode::RShift => self.rltk.shift = true,
                    VirtualKeyCode::LAlt => self.rltk.alt = true,
                    VirtualKeyCode::RAlt => self.rltk.alt = true,
                    VirtualKeyCode::LControl => self.rltk.control = true,
                    VirtualKeyCode::RControl => self.rltk.control = true,
                    _ => {
                        self.rltk.key = Some(key);
                    }
                }
            }
        }
        if let Some(pos) = inputs.mouse_position() {
            self.rltk.mouse_pos = (pos.0 as i32, pos.1 as i32);
        }
        if inputs.button_is_down(Button::Mouse(MouseButton::Left)) {
            self.rltk.left_click = true;
        }
        std::mem::drop(inputs);

        // Tick the game's state
        self.state.tick(&mut self.rltk);

        // Quit if RLTK wants to (it's my party and I'll quit if I want to)
        if self.rltk.quitting {
            return Trans::Quit;
        }

        // Update the simple consoles
        for cons in self.rltk.consoles.iter_mut() {
            let size = cons.console.get_char_size();
            if let Some(concrete) = cons.console.as_any().downcast_ref::<crate::SimpleConsole>() {
                data.world.insert(SimpleConsoleResource{
                    size,
                    tiles : concrete.tiles.clone()
                });
            }
        }

        Trans::None
    }
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

    fn initialize_console_objects(&mut self, world : &mut World) {
        let mut count = 0;
        for cons in self.rltk.consoles.iter_mut() {
            let size = cons.console.get_char_size();
            if let Some(concrete) = cons.console.as_any().downcast_ref::<crate::SimpleConsole>() {
                if let Some(ss) = &self.rltk.fonts[cons.font_index].ss {
                    assert!(count == 0, "Amethyst back-end only supports one simple console.");
                    count += 1;
                    let font_size = &self.rltk.fonts[cons.font_index].tile_size;
    
                    let mut transform = Transform::default();
                    transform.set_translation_xyz(
                        (self.rltk.width_pixels as f32 * 0.5) + (font_size.0 as f32 / 2.0), 
                        (self.rltk.height_pixels as f32 * 0.5) - (font_size.1 as f32 / 2.0), 
                        0.0
                    );
            
                    let map = TileMap::<SimpleConsoleTile, MortonEncoder2D>::new(
                        Vector3::new(size.0, size.1, 1),
                        Vector3::new(font_size.0, font_size.1, 1),
                        Some(ss.clone()),
                    );
                    /*let bgmap = TileMap::<SimpleConsoleBackgroundTile, MortonEncoder2D>::new(
                        Vector3::new(size.0, size.1, 1),
                        Vector3::new(font_size.0, font_size.1, 1),
                        Some(ss.clone()),
                    );*/

                    world.insert(SimpleConsoleResource{
                        size,
                        tiles : concrete.tiles.clone()
                    });
    
                    world
                        .create_entity()
                        .with(transform.clone())
                        .with(map)                    
                        .build();

                    /*world
                        .create_entity()
                        .with(transform.clone())
                        .with(bgmap)                    
                        .build();
                    */
                }
            };            
        }
    }
}

pub struct SimpleConsoleResource {
    pub size : (u32, u32),
    pub tiles : Vec<crate::Tile>
}

pub fn main_loop<GS: GameState>(rltk: Rltk, gamestate: GS) {
    amethyst::start_logger(Default::default());

    let mut cfg = amethyst::window::DisplayConfig::default();
    cfg.dimensions = Some((rltk.width_pixels, rltk.height_pixels));
    cfg.title = rltk.backend.platform.window_title.clone();

    let app_root = application_root_dir().unwrap();

    let input_bundle = InputBundle::<StringBindings>::new()
        .with_bindings(Bindings::new());

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle).expect("Input bundle fail")
        .with_bundle(TransformBundle::new()).expect("Transform bundle fail")
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
            .with_plugin(
                RenderToWindow::from_config(cfg)
                    .with_clear([0.00196, 0.23726, 0.21765, 1.0]),
            )
            .with_plugin(RenderFlat2D::default())
            .with_plugin(RenderTiles2D::<SimpleConsoleTile, MortonEncoder2D>::default())
        ).expect("Game data fail");
    let assets_dir = app_root;
    //let mut world = World::new(); // Why is this even here?
    let mut game = Application::new(
        assets_dir, 
        RltkGemBridge{rltk, state: Box::new(gamestate), key_delay : 0.0}, 
        game_data)
    .expect("Failed to make game data");
    game.run();
}
