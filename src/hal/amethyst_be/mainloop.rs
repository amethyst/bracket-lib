use crate::{GameState, Rltk};

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
    renderer::{Camera, ImageFormat, SpriteSheet, Texture, SpriteRender, palette::Srgba, resources::Tint, camera::Projection},
    input::{InputBundle, StringBindings, Bindings, InputHandler, Button},
    winit::MouseButton,
    ecs::prelude::*,
};

pub struct RltkGemBridge {
    rltk : Rltk,
    state : Box<dyn GameState>,
    key_delay : f32
}

impl SimpleState for RltkGemBridge {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<SimpleConsoleSprite>();
        world.register::<SimpleConsoleBackground>();
        world.register::<SparseConsoleSprite>();
        self.make_camera(world);
        super::font::initialize_fonts(&mut self.rltk, world);
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
        if self.key_delay > 75.0 {
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

        // Update the consoles
        {
            let simple_console_sprites = data.world.read_storage::<SimpleConsoleSprite>();
            let simple_console_bgs = data.world.read_storage::<SimpleConsoleBackground>();
            let mut sprites = data.world.write_storage::<SpriteRender>();
            let mut tints = data.world.write_storage::<Tint>();
            let mut sparse_consoles = data.world.write_storage::<SparseConsoleSprite>();
            let entities = data.world.entities();
            let mut transforms = data.world.write_storage::<Transform>();

            for (entity, _sc) in (&entities, &sparse_consoles).join() {
                entities.delete(entity).expect("Fail");
            }

            for cons in self.rltk.consoles.iter_mut() {
                let size = cons.console.get_char_size();

                if let Some(concrete) = cons.console.as_any().downcast_ref::<crate::SimpleConsole>() {
                    (&simple_console_sprites, &mut sprites, &mut tints).par_join().for_each(|(tile,sprite,tint)| {
                        let tile = &concrete.tiles[tile.idx];
                        sprite.sprite_number = tile.glyph as usize;
                        tint.0 = Srgba::new(tile.fg.r, tile.fg.g, tile.fg.b, 1.0);
                    });

                    (&simple_console_bgs, &mut tints).par_join().for_each(|(tile,tint)| {
                        let tile = &concrete.tiles[tile.idx];
                        tint.0 = Srgba::new(tile.bg.r, tile.bg.g, tile.bg.b, 1.0);
                    });
                } else if let Some(concrete) = cons.console.as_any().downcast_ref::<crate::SparseConsole>() {
                    // Sparse console detected
                    if let Some(ss) = &self.rltk.fonts[cons.font_index].ss {
                        let font_size = &self.rltk.fonts[cons.font_index].tile_size;
                        for tile in concrete.tiles.iter() {
                            let mut tile_transform = Transform::default();
                            tile_transform.set_translation_xyz(
                                (font_size.0 * (tile.idx as u32 % size.0)) as f32, 
                                (font_size.1 * (tile.idx as u32 / size.0)) as f32, 
                                0.5
                            );

                            let c = entities.create();
                            transforms.insert(c, tile_transform).expect("Fail");
                            sprites.insert(c, SpriteRender{ sprite_sheet: ss.clone(), sprite_number: tile.glyph as usize }).expect("Fail");
                            sparse_consoles.insert(c, SparseConsoleSprite{}).expect("Fail");
                            tints.insert(c, Tint(Srgba::new(tile.fg.r, tile.fg.g, tile.fg.b, 1.0))).expect("Fail");
                        }
                    }
                }
            }
        }
        data.world.maintain();

        Trans::None
    }
}

impl RltkGemBridge {
    fn make_camera(&self, world : &mut World) {
        let mut transform = Transform::default();
        let width = self.rltk.width_pixels as f32;
        let height = self.rltk.height_pixels as f32;
        transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

        world
            .create_entity()
            .with(Camera::from(Projection::orthographic(
                -width / 2.0,
                width / 2.0,
                -height / 2.0,
                height / 2.0,
                0.0,
                5.0
            )))
            .with(transform)
            .build();
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

                    let mut y = 0;
                    let mut x = 0;
                    for (idx, _chr) in concrete.tiles.iter().enumerate() {
                        let mut tile_transform = Transform::default();
                        tile_transform.set_translation_xyz(
                            (font_size.0 * x) as f32,
                            (font_size.1 * y) as f32,
                            0.0
                        );

                        let mut tile_bg_transform = Transform::default();
                        tile_bg_transform.set_translation_xyz(
                            (font_size.0 * x) as f32,
                            (font_size.1 * y) as f32,
                            -1.0
                        );

                        world
                            .create_entity()
                            .with(tile_transform)
                            .with(SpriteRender{ sprite_sheet: ss.clone(), sprite_number: 1 })
                            .with(SimpleConsoleSprite{ idx })
                            .with(Tint(Srgba::new(1.0, 1.0, 1.0, 1.0)))
                            .build();

                        world
                            .create_entity()
                            .with(tile_bg_transform)
                            .with(SpriteRender{ sprite_sheet: ss.clone(), sprite_number: 219 })
                            .with(SimpleConsoleBackground{ idx })
                            .with(Tint(Srgba::new(1.0, 0.0, 0.0, 1.0)))
                            .build();

                        x += 1;
                        if x >= size.0 {
                            x = 0;
                            y += 1;
                        }
                    }
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

struct SimpleConsoleSprite {
    idx : usize
}

impl Component for SimpleConsoleSprite {
    type Storage = DenseVecStorage<Self>;
}

struct SimpleConsoleBackground {
    idx : usize
}

impl Component for SimpleConsoleBackground {
    type Storage = DenseVecStorage<Self>;
}

struct SparseConsoleSprite {}

impl Component for SparseConsoleSprite {
    type Storage = DenseVecStorage<Self>;
}