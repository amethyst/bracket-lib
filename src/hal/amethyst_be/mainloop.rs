use crate::{GameState, Rltk};

use amethyst::{
    core::math::{Point3, Vector2, Vector3},
    core::transform::Transform,
    core::TransformBundle,
    ecs::prelude::*,
    input::{Bindings, Button, InputBundle, InputHandler, StringBindings},
    prelude::*,
    renderer::{camera::Projection, palette::Srgba, resources::Tint, Camera},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    tiles::{FlatEncoder, MapStorage, MortonEncoder2D, RenderTiles2D, Tile, TileMap},
    utils::application_root_dir,
    winit::MouseButton,
};

pub struct RltkGemBridge {
    rltk: Rltk,
    state: Box<dyn GameState>,
    key_delay: f32,
}

impl SimpleState for RltkGemBridge {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<SimpleConsoleLink>();
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

        {
            let mut map_storage = data
                .world
                .write_storage::<TileMap<SimpleConsoleTile, FlatEncoder>>();
            let console_storage = data.world.read_storage::<SimpleConsoleLink>();
            for (map, conlink) in (&mut map_storage, &console_storage).join() {
                let cons = &mut self.rltk.consoles[conlink.console_index];
                let size = cons.console.get_char_size();
                if let Some(concrete) = cons.console.as_any().downcast_ref::<crate::SimpleConsole>()
                {
                    amethyst::tiles::iters::Region::new(
                        Point3::new(0, 0, 1),
                        Point3::new(size.0, size.1, 1),
                    )
                    .iter()
                    .for_each(|coord| {
                        if let Some(fg) = map.get_mut(&coord) {
                            let idx = ((coord.y * size.0) + coord.x) as usize;
                            if idx < concrete.tiles.len() {
                                let tile = &concrete.tiles[idx];
                                fg.glyph = tile.glyph as usize;
                                fg.color.color.red = tile.fg.r;
                                fg.color.color.green = tile.fg.g;
                                fg.color.color.blue = tile.fg.b;
                            }
                        }
                    });

                    amethyst::tiles::iters::Region::new(
                        Point3::new(0, 0, 0),
                        Point3::new(size.0, size.1, 0),
                    )
                    .iter()
                    .for_each(|coord| {
                        if let Some(bg) = map.get_mut(&coord) {
                            let idx = ((coord.y * size.0) + coord.x) as usize;
                            if idx < concrete.tiles.len() {
                                let tile = &concrete.tiles[idx];
                                bg.glyph = 219;
                                bg.color.color.red = tile.bg.r;
                                bg.color.color.green = tile.bg.g;
                                bg.color.color.blue = tile.bg.b;
                            }
                        }
                    });
                }
            }

            let mut smap_storage = data
                .world
                .write_storage::<TileMap<SparseConsoleTile, FlatEncoder>>();
            for (map, conlink) in (&mut smap_storage, &console_storage).join() {
                let cons = &mut self.rltk.consoles[conlink.console_index];
                let size = cons.console.get_char_size();
                if let Some(concrete) = cons.console.as_any().downcast_ref::<crate::SparseConsole>()
                {
                    amethyst::tiles::iters::Region::new(
                        Point3::new(0, 0, 0),
                        Point3::new(size.0, size.1, 0),
                    )
                    .iter()
                    .for_each(|coord| {
                        if let Some(t) = map.get_mut(&coord) {
                            t.glyph = None;
                        }
                    });

                    for tile in concrete.tiles.iter() {
                        let x = tile.idx as u32 % size.0;
                        let y = size.1 - (tile.idx as u32 / size.0);
                        let point = Point3::new(x, y - 1, 0);
                        let t = map.get_mut(&point);
                        if let Some(t) = t {
                            t.glyph = Some(tile.glyph as usize);
                            t.color.color.red = tile.fg.r;
                            t.color.color.green = tile.fg.g;
                            t.color.color.blue = tile.fg.b;
                        }
                    }
                }
            }
        }

        Trans::None
    }
}

impl RltkGemBridge {
    fn make_camera(&self, world: &mut World) {
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
                5.0,
            )))
            .with(transform)
            .build();
    }

    fn initialize_console_objects(&mut self, world: &mut World) {
        for (i, cons) in self.rltk.consoles.iter_mut().enumerate() {
            let size = cons.console.get_char_size();
            if let Some(_concrete) = cons.console.as_any().downcast_ref::<crate::SimpleConsole>() {
                if let Some(ss) = &self.rltk.fonts[cons.font_index].ss {
                    let font_size = &self.rltk.fonts[cons.font_index].tile_size;

                    let mut transform = Transform::default();
                    transform.set_translation_xyz(
                        (self.rltk.width_pixels as f32 * 0.5) + (font_size.0 as f32 / 2.0),
                        (self.rltk.height_pixels as f32 * 0.5) - (font_size.1 as f32 / 2.0),
                        0.0,
                    );

                    let map = TileMap::<SimpleConsoleTile, FlatEncoder>::new(
                        Vector3::new(size.0, size.1, 2),
                        Vector3::new(font_size.0, font_size.1, 1),
                        Some(ss.clone()),
                    );

                    world
                        .create_entity()
                        .with(transform.clone())
                        .with(map)
                        .with(SimpleConsoleLink { console_index: i })
                        .build();
                }
            }

            if let Some(_concrete) = cons.console.as_any().downcast_ref::<crate::SparseConsole>() {
                if let Some(ss) = &self.rltk.fonts[cons.font_index].ss {
                    let font_size = &self.rltk.fonts[cons.font_index].tile_size;

                    let mut transform = Transform::default();
                    transform.set_translation_xyz(
                        (self.rltk.width_pixels as f32 * 0.5) + (font_size.0 as f32 / 2.0),
                        (self.rltk.height_pixels as f32 * 0.5) - (font_size.1 as f32 / 2.0),
                        1.0,
                    );

                    let map = TileMap::<SparseConsoleTile, FlatEncoder>::new(
                        Vector3::new(size.0, size.1, 1),
                        Vector3::new(font_size.0, font_size.1, 1),
                        Some(ss.clone()),
                    );

                    world
                        .create_entity()
                        .with(transform.clone())
                        .with(map)
                        .with(SimpleConsoleLink { console_index: i })
                        .build();
                }
            }
        }
    }
}

pub fn main_loop<GS: GameState>(rltk: Rltk, gamestate: GS) {
    amethyst::start_logger(Default::default());

    let mut cfg = amethyst::window::DisplayConfig::default();
    cfg.dimensions = Some((rltk.width_pixels, rltk.height_pixels));
    cfg.title = rltk.backend.platform.window_title.clone();

    let app_root = application_root_dir().unwrap();

    let input_bundle = InputBundle::<StringBindings>::new().with_bindings(Bindings::new());

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)
        .expect("Input bundle fail")
        .with_bundle(TransformBundle::new())
        .expect("Transform bundle fail")
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config(cfg).with_clear([0.00196, 0.23726, 0.21765, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<SimpleConsoleTile, FlatEncoder>::default())
                .with_plugin(RenderTiles2D::<SparseConsoleTile, FlatEncoder>::default()),
        )
        .expect("Game data fail");
    let assets_dir = app_root;
    let mut game = Application::new(
        assets_dir,
        RltkGemBridge {
            rltk,
            state: Box::new(gamestate),
            key_delay: 0.0,
        },
        game_data,
    )
    .expect("Failed to make game data");
    game.run();
}

#[derive(Clone, Debug)]
struct SimpleConsoleLink {
    console_index: usize,
}

impl Component for SimpleConsoleLink {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Debug)]
struct SimpleConsoleTile {
    glyph: usize,
    color: Srgba,
}

impl Default for SimpleConsoleTile {
    fn default() -> Self {
        SimpleConsoleTile {
            glyph: 65,
            color: Srgba::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

impl Tile for SimpleConsoleTile {
    fn sprite(&self, _pt: Point3<u32>, _world: &World) -> Option<usize> {
        Some(self.glyph)
    }

    fn tint(&self, _pt: Point3<u32>, _world: &World) -> Srgba {
        self.color
    }
}

#[derive(Clone, Debug)]
struct SparseConsoleTile {
    glyph: Option<usize>,
    color: Srgba,
}

impl Default for SparseConsoleTile {
    fn default() -> Self {
        SparseConsoleTile {
            glyph: None,
            color: Srgba::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

impl Tile for SparseConsoleTile {
    fn sprite(&self, _pt: Point3<u32>, _world: &World) -> Option<usize> {
        self.glyph
    }

    fn tint(&self, _pt: Point3<u32>, _world: &World) -> Srgba {
        self.color
    }
}
