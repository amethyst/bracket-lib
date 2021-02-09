use super::*;
use crate::prelude::{BEvent, BTerm, GameState, BACKEND_INTERNAL};
use crate::{clear_input_state, Result};

use amethyst::{
    core::math::{Point3, Vector3},
    core::transform::Transform,
    core::TransformBundle,
    //core::frame_limiter::{FrameLimiter, FrameRateLimitStrategy},
    ecs::prelude::*,
    input::{Bindings, InputBundle, InputHandler, StringBindings},
    prelude::*,
    renderer::{palette::Srgba, Camera},
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    tiles::{FlatEncoder, MapStorage, RenderTiles2D, Tile, TileMap},
    utils::application_root_dir,
    winit::MouseButton,
};

pub struct BTermGemBridge {
    bterm: BTerm,
    state: Box<dyn GameState>,
    input_reader: Option<amethyst::shrev::ReaderId<amethyst::input::InputEvent<StringBindings>>>,
}

impl SimpleState for BTermGemBridge {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<SimpleConsoleLink>();
        self.make_camera(world);
        super::font::initialize_fonts(world).unwrap();
        self.initialize_console_objects(world);

        // Frame rate limiter - does not override vsync?
        //world.insert(FrameLimiter::new(FrameRateLimitStrategy::Unlimited, 0));
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> amethyst::SimpleTrans {
        // Frame times
        let timer = data.world.fetch::<amethyst::core::Time>();
        self.bterm.frame_time_ms = timer.delta_time().as_millis() as f32;
        self.bterm.fps = 1.0 / timer.delta_seconds();
        std::mem::drop(timer);

        // Handle Input
        clear_input_state(&mut self.bterm);

        use amethyst::input::InputEvent;
        use amethyst::shrev::EventChannel;
        let mut channel = data
            .world
            .fetch_mut::<EventChannel<InputEvent<StringBindings>>>();
        if let Some(mut reader) = self.input_reader.as_mut() {
            for event in channel.read(&mut reader) {
                match event {
                    InputEvent::CursorMoved { .. } => {
                        // We don't want delta..
                        let inputs = data.world.fetch::<InputHandler<StringBindings>>();
                        if let Some(pos) = inputs.mouse_position() {
                            self.bterm.on_mouse_position(pos.0 as f64, pos.1 as f64);
                        }
                    }
                    InputEvent::MouseButtonPressed(button) => {
                        self.bterm.on_mouse_button(
                            match button {
                                MouseButton::Left => 0,
                                MouseButton::Right => 1,
                                MouseButton::Middle => 2,
                                MouseButton::Other(num) => 3 + *num as usize,
                            },
                            true,
                        );
                    }
                    InputEvent::MouseButtonReleased(button) => {
                        self.bterm.on_mouse_button(
                            match button {
                                MouseButton::Left => 0,
                                MouseButton::Right => 1,
                                MouseButton::Middle => 2,
                                MouseButton::Other(num) => 3 + *num as usize,
                            },
                            false,
                        );
                    }
                    InputEvent::KeyPressed { key_code, scancode } => {
                        self.bterm.on_key(*key_code, *scancode, true);
                    }
                    InputEvent::KeyReleased { key_code, scancode } => {
                        self.bterm.on_key(*key_code, *scancode, true);
                    }
                    InputEvent::KeyTyped(c) => self.bterm.on_event(BEvent::Character { c: *c }),
                    InputEvent::ButtonPressed { .. } => {}
                    InputEvent::ButtonReleased { .. } => {}
                    _ => {}
                }
            }
        } else {
            self.input_reader = Some(channel.register_reader());
        }

        // Tick the game's state
        self.state.tick(&mut self.bterm);

        // Quit if BTerm wants to (it's my party and I'll quit if I want to)
        if self.bterm.quitting {
            return Trans::Quit;
        }

        {
            let mut bi = BACKEND_INTERNAL.lock();
            let mut map_storage = data
                .world
                .write_storage::<TileMap<SimpleConsoleTile, FlatEncoder>>();
            let console_storage = data.world.read_storage::<SimpleConsoleLink>();
            for (map, conlink) in (&mut map_storage, &console_storage).join() {
                let cons = &mut bi.consoles[conlink.console_index];
                let size = cons.console.get_char_size();
                if let Some(concrete) = cons
                    .console
                    .as_any()
                    .downcast_ref::<crate::prelude::SimpleConsole>()
                {
                    amethyst::tiles::iters::Region::new(
                        Point3::new(0, 0, 1),
                        Point3::new(size.0, size.1, 1),
                    )
                    .iter()
                    .for_each(|coord| {
                        if let Some(fg) = map.get_mut(&coord) {
                            let flipped_y = (size.1 - 1) - coord.y;
                            let idx = ((flipped_y * size.0) + coord.x) as usize;
                            if idx < concrete.tiles.len() {
                                let tile = &concrete.tiles[idx];
                                fg.glyph = tile.glyph as usize;
                                fg.color.color.red = tile.fg.r;
                                fg.color.color.green = tile.fg.g;
                                fg.color.color.blue = tile.fg.b;
                                fg.color.alpha = tile.fg.a;
                            }
                        }
                    });

                    amethyst::tiles::iters::Region::new(
                        Point3::new(0, 0, 0),
                        Point3::new(size.0, size.1 - 1, 0),
                    )
                    .iter()
                    .for_each(|coord| {
                        if let Some(bg) = map.get_mut(&coord) {
                            let flipped_y = (size.1 - 1) - coord.y;
                            let idx = ((flipped_y * size.0) + coord.x) as usize;
                            if idx < concrete.tiles.len() {
                                let tile = &concrete.tiles[idx];
                                bg.glyph = 219;
                                bg.color.color.red = tile.bg.r;
                                bg.color.color.green = tile.bg.g;
                                bg.color.color.blue = tile.bg.b;
                                bg.color.alpha = tile.bg.a;
                            }
                        }
                    });
                }
            }

            let mut smap_storage = data
                .world
                .write_storage::<TileMap<SparseConsoleTile, FlatEncoder>>();
            for (map, conlink) in (&mut smap_storage, &console_storage).join() {
                let cons = &mut bi.consoles[conlink.console_index];
                let size = cons.console.get_char_size();
                if let Some(concrete) = cons
                    .console
                    .as_any()
                    .downcast_ref::<crate::prelude::SparseConsole>()
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
                            t.color.alpha = tile.fg.a;
                        }
                    }
                }
            }
        }

        Trans::None
    }
}

impl BTermGemBridge {
    fn make_camera(&self, world: &mut World) {
        let mut transform = Transform::default();
        let width = self.bterm.width_pixels as f32;
        let height = self.bterm.height_pixels as f32;
        transform.set_translation_xyz(width * 0.5, height * 0.5, 1.0);

        world
            .create_entity()
            .with(Camera::orthographic(
                -width / 2.0,
                width / 2.0,
                -height / 2.0,
                height / 2.0,
                0.0,
                5.0,
            ))
            .with(transform)
            .build();
    }

    fn initialize_console_objects(&mut self, world: &mut World) {
        let bi = BACKEND_INTERNAL.lock();
        for (i, cons) in bi.consoles.iter().enumerate() {
            let size = cons.console.get_char_size();
            if let Some(_concrete) = cons
                .console
                .as_any()
                .downcast_ref::<crate::prelude::SimpleConsole>()
            {
                if let Some(ss) = &bi.fonts[cons.font_index].ss {
                    let font_size = &bi.fonts[cons.font_index].tile_size;

                    let mut transform = Transform::default();
                    transform.set_translation_xyz(
                        (self.bterm.width_pixels as f32 * 0.5) + (font_size.0 as f32 / 2.0),
                        (self.bterm.height_pixels as f32 * 0.5) - (font_size.1 as f32 / 2.0),
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

            if let Some(_concrete) = cons
                .console
                .as_any()
                .downcast_ref::<crate::prelude::SparseConsole>()
            {
                if let Some(ss) = &bi.fonts[cons.font_index].ss {
                    let font_size = &bi.fonts[cons.font_index].tile_size;

                    let mut transform = Transform::default();
                    transform.set_translation_xyz(
                        (self.bterm.width_pixels as f32 * 0.5) + (font_size.0 as f32 / 2.0),
                        (self.bterm.height_pixels as f32 * 0.5) - (font_size.1 as f32 / 2.0),
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

pub fn main_loop<GS: GameState>(bterm: BTerm, gamestate: GS) -> Result<()> {
    amethyst::start_logger(Default::default());

    let mut cfg = amethyst::window::DisplayConfig::default();
    cfg.dimensions = Some((bterm.width_pixels, bterm.height_pixels));
    cfg.title = BACKEND.lock().window_title.clone();

    let app_root = application_root_dir()?;

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
        BTermGemBridge {
            bterm,
            state: Box::new(gamestate),
            input_reader: None,
        },
        game_data,
    )
    .expect("Failed to make game data");
    game.run();
    Ok(())
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
