#![allow(dead_code)]
#[allow(unused_imports)]
use crate::{
    prelude::{
        init_raw, BEvent, CharacterTranslationMode, Console, FlexiConsole, Font, FontCharType,
        GameState, InitHints, Radians, RenderSprite, Shader, SimpleConsole, SpriteConsole,
        SpriteSheet, TextAlign, VirtualKeyCode, XpFile, XpLayer, BACKEND, INPUT,
    },
    Result,
};
use bracket_color::prelude::RGBA;
use bracket_geometry::prelude::{Point, PointF, Rect};
use parking_lot::Mutex;
use std::convert::*;

/// A display console, used internally to provide console render support.
/// Public in case you want to play with it, or access it directly.
pub struct DisplayConsole {
    pub console: Box<dyn Console>,
    pub shader_index: usize,
    pub font_index: usize,
}

pub struct BTermInternal {
    pub fonts: Vec<Font>,
    pub shaders: Vec<Shader>,
    pub consoles: Vec<DisplayConsole>,
    pub sprite_sheets: Vec<SpriteSheet>,
}

impl BTermInternal {
    pub fn new() -> Self {
        Self {
            fonts: Vec::new(),
            shaders: Vec::new(),
            consoles: Vec::new(),
            sprite_sheets: Vec::new(),
        }
    }
}

impl Default for BTermInternal {
    fn default() -> Self {
        Self {
            fonts: Vec::new(),
            shaders: Vec::new(),
            consoles: Vec::new(),
            sprite_sheets: Vec::new(),
        }
    }
}

unsafe impl Send for BTermInternal {}
unsafe impl Sync for BTermInternal {}

lazy_static! {
    pub static ref BACKEND_INTERNAL: Mutex<BTermInternal> = Mutex::new(BTermInternal::new());
}

/// A BTerm context.
#[derive(Clone, Debug)]
pub struct BTerm {
    pub width_pixels: u32,
    pub height_pixels: u32,
    pub original_height_pixels: u32,
    pub original_width_pixels: u32,
    pub fps: f32,
    pub frame_time_ms: f32,
    pub active_console: usize,
    pub key: Option<VirtualKeyCode>,
    pub mouse_pos: (i32, i32),
    pub left_click: bool,
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
    pub web_button: Option<String>,
    pub quitting: bool,
    pub post_scanlines: bool,
    pub post_screenburn: bool,
    pub screen_burn_color: bracket_color::prelude::RGB
}

impl BTerm {
    /// Initializes an OpenGL context and a window, stores the info in the BTerm structure.
    pub fn init_raw<S: ToString, T>(
        width_pixels: T,
        height_pixels: T,
        window_title: S,
        platform_hints: InitHints,
    ) -> Result<BTerm>
    where
        T: TryInto<u32>,
    {
        let w = width_pixels.try_into();
        let h = height_pixels.try_into();
        let (w, h) = if let (Ok(w), Ok(h)) = (w, h) {
            (w, h)
        } else {
            return Err("Couldn't convert to u32".into());
        };
        Ok(init_raw(w, h, window_title, platform_hints)?)
    }

    /// Quick initialization for when you just want an 8x8 font terminal
    #[deprecated(
        since = "0.6.2",
        note = "Please migrate to the BTermBuilder system instead."
    )]
    pub fn init_simple8x8<S: ToString, T>(
        width_chars: T,
        height_chars: T,
        window_title: S,
        path_to_shaders: S,
    ) -> BTerm
    where
        T: TryInto<u32>,
    {
        let w: u32 = width_chars.try_into().ok().unwrap();
        let h: u32 = height_chars.try_into().ok().unwrap();
        let font_path = format!("{}/terminal8x8.png", &path_to_shaders.to_string());
        let mut context = BTerm::init_raw(w * 8, h * 8, window_title, InitHints::new()).unwrap();
        let font = context.register_font(Font::load(font_path, (8, 8), None));
        context.register_console(SimpleConsole::init(w, h), font.unwrap());
        context
    }

    /// Quick initialization for when you just want an 8x16 VGA font terminal
    #[deprecated(
        since = "0.6.2",
        note = "Please migrate to the BTermBuilder system instead."
    )]
    pub fn init_simple8x16<S: ToString, T>(
        width_chars: T,
        height_chars: T,
        window_title: S,
        path_to_shaders: S,
    ) -> BTerm
    where
        T: TryInto<u32>,
    {
        let w: u32 = width_chars.try_into().ok().unwrap();
        let h: u32 = height_chars.try_into().ok().unwrap();
        let font_path = format!("{}/vga8x16.png", &path_to_shaders.to_string());
        let mut context = BTerm::init_raw(w * 8, h * 16, window_title, InitHints::new()).unwrap();
        let font = context.register_font(Font::load(font_path, (8, 16), None));
        context.register_console(SimpleConsole::init(w, h), font.unwrap());
        context
    }

    /// Registers a font, and returns its handle number. Do not use after initialization!
    pub(crate) fn register_font(&mut self, font: Font) -> Result<usize> {
        let mut bi = BACKEND_INTERNAL.lock();
        bi.fonts.push(font);
        Ok(bi.fonts.len() - 1)
    }

    /// Registers a new console terminal for output, and returns its handle number.
    pub fn register_console(&mut self, new_console: Box<dyn Console>, font_index: usize) -> usize {
        let mut bi = BACKEND_INTERNAL.lock();
        bi.consoles.push(DisplayConsole {
            console: new_console,
            font_index,
            shader_index: 0,
        });
        bi.consoles.len() - 1
    }

    /// Registers a new console terminal for output, and returns its handle number. This variant requests
    /// that the new console not render background colors, so it can be layered on top of other consoles.
    pub fn register_console_no_bg(
        &mut self,
        new_console: Box<dyn Console>,
        font_index: usize,
    ) -> usize {
        let mut bi = BACKEND_INTERNAL.lock();
        bi.consoles.push(DisplayConsole {
            console: new_console,
            font_index,
            shader_index: 1,
        });
        bi.consoles.len() - 1
    }

    /// Registers a new console terminal for output, and returns its handle number. This variant requests
    /// that the new console not render background colors, so it can be layered on top of other consoles.
    pub fn register_fancy_console(
        &mut self,
        new_console: Box<dyn Console>,
        font_index: usize,
    ) -> usize {
        let mut bi = BACKEND_INTERNAL.lock();
        bi.consoles.push(DisplayConsole {
            console: new_console,
            font_index,
            shader_index: 4,
        });
        bi.consoles.len() - 1
    }

    /// Registers a new Sprite-based console
    pub fn register_sprite_console(&mut self, new_console: Box<dyn Console>) -> usize {
        let mut bi = BACKEND_INTERNAL.lock();
        bi.consoles.push(DisplayConsole {
            console: new_console,
            font_index: 0,
            shader_index: 5,
        });
        bi.consoles.len() - 1
    }

    /// Sets the currently active console number.
    pub fn set_active_console(&mut self, id: usize) {
        let length = BACKEND_INTERNAL.lock().consoles.len();
        if id < length {
            self.active_console = id;
        } else {
            panic!(
                "Invalid console id: {}. Valid consoles are 0..{}",
                id, length
            );
        }
    }

    /// Applies the current physical mouse position to the active console, and translates the coordinates into that console's coordinate space.
    #[cfg(feature = "curses")]
    pub fn mouse_pos(&self) -> (i32, i32) {
        (self.mouse_pos.0, self.mouse_pos.1)
    }

    ///
    #[cfg(feature = "curses")]
    fn pixel_to_char_pos(&self, pos: (i32, i32), _console: &Box<dyn Console>) -> (i32, i32) {
        pos
    }

    #[cfg(not(feature = "curses"))]
    fn pixel_to_char_pos(&self, pos: (i32, i32), console: &Box<dyn Console>) -> (i32, i32) {
        let max_sizes = console.get_char_size();
        let (scale, center_x, center_y) = console.get_scale();
        let font_size = (
            self.width_pixels as f32 / max_sizes.0 as f32,
            self.height_pixels as f32 / max_sizes.1 as f32,
        );
        let offsets = (
            center_x as f32 * font_size.0 * (scale - 1.0),
            center_y as f32 * font_size.1 * (scale - 1.0),
        );
        (
            iclamp(
                ((pos.0 as f32 + offsets.0) * max_sizes.0 as f32
                    / f32::max(1.0, scale * self.width_pixels as f32)) as i32,
                0,
                max_sizes.0 as i32 - 1,
            ),
            iclamp(
                ((pos.1 as f32 + offsets.1) * max_sizes.1 as f32
                    / f32::max(1.0, scale * self.height_pixels as f32)) as i32,
                0,
                max_sizes.1 as i32 - 1,
            ),
        )
    }

    /// Applies the current physical mouse position to the active console, and translates the coordinates into that console's coordinate space.
    #[cfg(not(feature = "curses"))]
    pub fn mouse_pos(&self) -> (i32, i32) {
        let bi = BACKEND_INTERNAL.lock();
        let active_console = &bi.consoles[self.active_console].console;

        self.pixel_to_char_pos(self.mouse_pos, active_console)
    }

    /// Applies the current physical mouse position to the active console, and translates the coordinates into that console's coordinate space.
    pub fn mouse_point(&self) -> Point {
        let bi = BACKEND_INTERNAL.lock();
        let active_console = &bi.consoles[self.active_console].console;
        let char_pos = self.pixel_to_char_pos(self.mouse_pos, active_console);

        Point::new(char_pos.0, char_pos.1)
    }

    /// Tells the game to quit
    pub fn quit(&mut self) {
        self.quitting = true;
    }

    /// Render a REX Paint (https://www.gridsagegames.com/rexpaint/) file as a sprite.
    /// The sprite will be offset by offset_x and offset_y.
    /// Transparent cells will not be rendered.
    pub fn render_xp_sprite(&mut self, xp: &super::rex::XpFile, x: i32, y: i32) {
        let mut bi = BACKEND_INTERNAL.lock();
        super::rex::xp_to_console(xp, &mut bi.consoles[self.active_console].console, x, y);
    }

    /// Saves the entire console stack to a REX Paint XP file. If your consoles are of
    /// varying sizes, the file format supports it - but REX doesn't. So you may want to
    /// avoid that. You can also get individual layers with `to_xp_layer`.
    pub fn to_xp_file(&self, width: usize, height: usize) -> XpFile {
        let bi = BACKEND_INTERNAL.lock();
        let mut xp = XpFile::new(width, height);

        xp.layers
            .push(bi.consoles[self.active_console].console.to_xp_layer());

        if bi.consoles.len() > 1 {
            for layer in bi.consoles.iter().skip(1) {
                xp.layers.push(layer.console.to_xp_layer());
            }
        }

        xp
    }

    /// Enable scanlines post-processing effect.
    pub fn with_post_scanlines(&mut self, with_burn: bool) {
        self.post_scanlines = true;
        self.post_screenburn = with_burn;
    }

    // Change the screen-burn color
    pub fn screen_burn_color(&mut self, color: bracket_color::prelude::RGB) {
        self.screen_burn_color = color;
    }

    /// Internal: mark a key press
    pub(crate) fn on_key(&mut self, key: VirtualKeyCode, scan_code: u32, pressed: bool) {
        let mut input = INPUT.lock();
        if pressed {
            self.key = Some(key);
            input.on_key_down(key, scan_code);
        } else {
            self.key = None;
            input.on_key_up(key, scan_code);
        }
        input.push_event(BEvent::KeyboardInput {
            key,
            scan_code,
            pressed,
        });
    }

    /// Internal: mark a mouse press
    pub(crate) fn on_mouse_button(&mut self, button_num: usize, pressed: bool) {
        if button_num == 0 {
            self.left_click = true;
        }
        let mut input = INPUT.lock();
        if pressed {
            input.on_mouse_button_down(button_num);
        } else {
            input.on_mouse_button_up(button_num);
        }
        input.push_event(BEvent::MouseClick {
            button: button_num,
            pressed,
        });
    }

    /// Internal: mark mouse position changes
    pub(crate) fn on_mouse_position(&mut self, x: f64, y: f64) {
        let bi = BACKEND_INTERNAL.lock();
        self.mouse_pos = (x as i32, y as i32);
        let mut input = INPUT.lock();
        input.on_mouse_pixel_position(x, y);
        // TODO: Console cascade!
        for (i, cons) in bi.consoles.iter().enumerate() {
            let char_pos = self.pixel_to_char_pos(self.mouse_pos, &cons.console);

            input.on_mouse_tile_position(i, char_pos.0, char_pos.1);
        }
    }

    /// Internal: record an event from the HAL back-end
    #[allow(dead_code)]
    pub(crate) fn on_event(&mut self, event: BEvent) {
        INPUT.lock().push_event(event);
    }
}

/// Implements console-like BTerm. Note that this *isn't* a Console trait anymore,
/// due to the need for helper generics.
impl BTerm {
    /// Gets the active console's size, in characters.
    pub fn get_char_size(&self) -> (u32, u32) {
        let bi = BACKEND_INTERNAL.lock();
        bi.consoles[self.active_console].console.get_char_size()
    }

    /// Internal - do not use.
    /// Passes a resize message down to all registered consoles.
    pub(crate) fn resize_pixels<T>(&mut self, width: T, height: T, scaling_enabled: bool)
    where
        T: Into<u32>,
    {
        self.width_pixels = width.into();
        self.height_pixels = height.into();

        if scaling_enabled {
            self.original_width_pixels = self.width_pixels;
            self.original_height_pixels = self.height_pixels;
        }

        let mut bi = BACKEND_INTERNAL.lock();
        for c in bi.consoles.iter_mut() {
            c.console
                .resize_pixels(self.width_pixels, self.height_pixels);
        }
    }

    /// Request that the active console clear itself to default values.
    pub fn cls(&mut self) {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .cls();
    }

    /// Request that the active console clear itself to a specified background color.
    /// Has no effect on consoles that don't have a background color.
    pub fn cls_bg<COLOR>(&mut self, background: COLOR)
    where
        COLOR: Into<RGBA>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .cls_bg(background.into());
    }

    /// Print a string to the active console.
    pub fn print<S, X, Y>(&mut self, x: X, y: Y, output: S)
    where
        S: ToString,
        X: TryInto<i32>,
        Y: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .print(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                &output.to_string(),
            );
    }

    /// Print a string to the active console, in color.
    pub fn print_color<S, COLOR, COLOR2, X, Y>(
        &mut self,
        x: X,
        y: Y,
        fg: COLOR,
        bg: COLOR2,
        output: S,
    ) where
        S: ToString,
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        X: TryInto<i32>,
        Y: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .print_color(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                fg.into(),
                bg.into(),
                &output.to_string(),
            );
    }

    /// Set a single tile located at x/y to the specified foreground/background colors, and glyph.
    pub fn set<COLOR, COLOR2, GLYPH, X, Y>(
        &mut self,
        x: X,
        y: Y,
        fg: COLOR,
        bg: COLOR2,
        glyph: GLYPH,
    ) where
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        GLYPH: TryInto<FontCharType>,
        X: TryInto<i32>,
        Y: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .set(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                fg.into(),
                bg.into(),
                glyph.try_into().ok().expect("Must be u16 convertible"),
            );
    }

    /// Set a tile with "fancy" additional attributes
    #[cfg(feature = "opengl")]
    #[allow(clippy::too_many_arguments)]
    pub fn set_fancy<COLOR, COLOR2, GLYPH, ANGLE>(
        &mut self,
        position: PointF,
        z_order: i32,
        rotation: ANGLE,
        scale: PointF,
        fg: COLOR,
        bg: COLOR2,
        glyph: GLYPH,
    ) where
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        GLYPH: TryInto<FontCharType>,
        ANGLE: Into<Radians>,
    {
        let mut be = BACKEND_INTERNAL.lock();
        let cons_any = be.consoles[self.active_console].console.as_any_mut();
        if let Some(fc) = cons_any.downcast_mut::<FlexiConsole>() {
            fc.set_fancy(
                position,
                z_order,
                rotation.into().0,
                scale,
                fg.into(),
                bg.into(),
                glyph.try_into().ok().expect("Must be u16 convertible"),
            );
        }
    }

    /// Set a tile with "fancy" additional attributes
    #[cfg(not(feature = "opengl"))]
    pub fn set_fancy<COLOR, COLOR2, GLYPH, ANGLE>(
        &mut self,
        _position: PointF,
        _z_order: i32,
        _rotation: ANGLE,
        _scale: PointF,
        _fg: COLOR,
        _bg: COLOR2,
        _glyph: GLYPH,
    ) where
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        GLYPH: TryInto<FontCharType>,
        ANGLE: Into<Radians>,
    {
        // Does nothing
    }

    /// Sets the background color only of a specified tile.
    pub fn set_bg<COLOR, X, Y>(&mut self, x: X, y: Y, bg: COLOR)
    where
        COLOR: Into<RGBA>,
        X: TryInto<i32>,
        Y: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .set_bg(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                bg.into(),
            );
    }

    /// Draws a filled box, with single line characters.
    pub fn draw_box<COLOR, COLOR2, X, Y, W, H>(
        &mut self,
        x: X,
        y: Y,
        width: W,
        height: H,
        fg: COLOR,
        bg: COLOR2,
    ) where
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        X: TryInto<i32>,
        Y: TryInto<i32>,
        W: TryInto<i32>,
        H: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .draw_box(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                width.try_into().ok().expect("Must be i32 convertible"),
                height.try_into().ok().expect("Must be i32 convertible"),
                fg.into(),
                bg.into(),
            );
    }

    /// Draws a filled box, with double line characters.
    pub fn draw_box_double<COLOR, COLOR2, X, Y, W, H>(
        &mut self,
        x: X,
        y: Y,
        width: W,
        height: H,
        fg: COLOR,
        bg: COLOR2,
    ) where
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        X: TryInto<i32>,
        Y: TryInto<i32>,
        W: TryInto<i32>,
        H: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .draw_box_double(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                width.try_into().ok().expect("Must be i32 convertible"),
                height.try_into().ok().expect("Must be i32 convertible"),
                fg.into(),
                bg.into(),
            );
    }

    /// Draws a single-line box, without filling in the center.
    pub fn draw_hollow_box<COLOR, COLOR2, X, Y, W, H>(
        &mut self,
        x: X,
        y: Y,
        width: W,
        height: H,
        fg: COLOR,
        bg: COLOR2,
    ) where
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        X: TryInto<i32>,
        Y: TryInto<i32>,
        W: TryInto<i32>,
        H: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .draw_hollow_box(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                width.try_into().ok().expect("Must be i32 convertible"),
                height.try_into().ok().expect("Must be i32 convertible"),
                fg.into(),
                bg.into(),
            );
    }

    /// Draws a double-line box, without filling in the contents.
    pub fn draw_hollow_box_double<COLOR, COLOR2, X, Y, W, H>(
        &mut self,
        x: X,
        y: Y,
        width: W,
        height: H,
        fg: COLOR,
        bg: COLOR2,
    ) where
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        X: TryInto<i32>,
        Y: TryInto<i32>,
        W: TryInto<i32>,
        H: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .draw_hollow_box_double(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                width.try_into().ok().expect("Must be i32 convertible"),
                height.try_into().ok().expect("Must be i32 convertible"),
                fg.into(),
                bg.into(),
            );
    }

    /// Draws a horizontal bar, suitable for health-bars or progress bars.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_bar_horizontal<COLOR, COLOR2, X, Y, W, N, MAX>(
        &mut self,
        x: X,
        y: Y,
        width: W,
        n: N,
        max: MAX,
        fg: COLOR,
        bg: COLOR2,
    ) where
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        X: TryInto<i32>,
        Y: TryInto<i32>,
        W: TryInto<i32>,
        N: TryInto<i32>,
        MAX: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .draw_bar_horizontal(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                width.try_into().ok().expect("Must be i32 convertible"),
                n.try_into().ok().expect("Must be i32 convertible"),
                max.try_into().ok().expect("Must be i32 convertible"),
                fg.into(),
                bg.into(),
            );
    }

    /// Draws a vertical bar, suitable for health-bars or progress bars.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_bar_vertical<COLOR, COLOR2, X, Y, H, N, MAX>(
        &mut self,
        x: X,
        y: Y,
        height: H,
        n: N,
        max: MAX,
        fg: COLOR,
        bg: COLOR2,
    ) where
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        X: TryInto<i32>,
        Y: TryInto<i32>,
        H: TryInto<i32>,
        N: TryInto<i32>,
        MAX: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .draw_bar_vertical(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                height.try_into().ok().expect("Must be i32 convertible"),
                n.try_into().ok().expect("Must be i32 convertible"),
                max.try_into().ok().expect("Must be i32 convertible"),
                fg.into(),
                bg.into(),
            );
    }

    /// Fills a target region with the specified color/glyph combo.
    pub fn fill_region<COLOR, COLOR2, GLYPH>(
        &mut self,
        target: Rect,
        glyph: GLYPH,
        fg: COLOR,
        bg: COLOR2,
    ) where
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        GLYPH: TryInto<FontCharType>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .fill_region(target, glyph.try_into().ok().unwrap(), fg.into(), bg.into());
    }

    /// Prints centered text, centered across the whole line
    pub fn print_centered<S, Y>(&mut self, y: Y, text: S)
    where
        S: ToString,
        Y: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .print_centered(
                y.try_into().ok().expect("Must be i32 convertible"),
                &text.to_string(),
            );
    }

    /// Prints centered text, centered across the whole line - in color
    pub fn print_color_centered<S, COLOR, COLOR2, Y>(
        &mut self,
        y: Y,
        fg: COLOR,
        bg: COLOR2,
        text: S,
    ) where
        S: ToString,
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        Y: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .print_color_centered(
                y.try_into().ok().expect("Must be i32 convertible"),
                fg.into(),
                bg.into(),
                &text.to_string(),
            );
    }

    /// Prints text, centered on an arbitrary point
    pub fn print_centered_at<S, X, Y>(&mut self, x: X, y: Y, text: S)
    where
        S: ToString,
        X: TryInto<i32>,
        Y: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .print_centered_at(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                &text.to_string(),
            );
    }

    /// Prints colored text, centered on an arbitrary point
    pub fn print_color_centered_at<S, COLOR, COLOR2, X, Y>(
        &mut self,
        x: X,
        y: Y,
        fg: COLOR,
        bg: COLOR2,
        text: S,
    ) where
        S: ToString,
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        X: TryInto<i32>,
        Y: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .print_color_centered_at(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                fg.into(),
                bg.into(),
                &text.to_string(),
            );
    }

    /// Prints right-aligned text
    pub fn print_right<S, X, Y>(&mut self, x: X, y: Y, text: S)
    where
        S: ToString,
        X: TryInto<i32>,
        Y: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .print_right(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                &text.to_string(),
            );
    }

    /// Prints right-aligned text, in color
    pub fn print_color_right<S, COLOR, COLOR2, X, Y>(
        &mut self,
        x: X,
        y: Y,
        fg: COLOR,
        bg: COLOR2,
        text: S,
    ) where
        S: ToString,
        COLOR: Into<RGBA>,
        COLOR2: Into<RGBA>,
        X: TryInto<i32>,
        Y: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .print_color_right(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                fg.into(),
                bg.into(),
                &text.to_string(),
            );
    }

    /// Print a colorized string with the color encoding defined inline.
    /// For example: printer(1, 1, "#[blue]This blue text contains a #[pink]pink#[] word")
    /// You can get the same effect with a TextBlock, but this can be easier.
    /// Thanks to doryen_rs for the idea.
    pub fn printer<S, X, Y>(
        &mut self,
        x: X,
        y: Y,
        output: S,
        align: TextAlign,
        background: Option<RGBA>,
    ) where
        S: ToString,
        X: TryInto<i32>,
        Y: TryInto<i32>,
    {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .printer(
                x.try_into().ok().expect("Must be i32 convertible"),
                y.try_into().ok().expect("Must be i32 convertible"),
                &output.to_string(),
                align,
                background,
            );
    }

    /// Exports the current layer to a REX Paint file
    pub fn to_xp_layer(&self) -> XpLayer {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .to_xp_layer()
    }

    /// Sets the active offset for the current layer
    pub fn set_offset(&mut self, x: f32, y: f32) {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .set_offset(x, y);
    }

    /// Sets the active scale for the current layer
    pub fn set_scale(&mut self, scale: f32, center_x: i32, center_y: i32) {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .set_scale(scale, center_x, center_y);
    }

    /// Gets the active scale for the current layer
    pub fn get_scale(&self) -> (f32, i32, i32) {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .get_scale()
    }

    /// Permits the creation of an arbitrary clipping rectangle. It's a really good idea
    /// to make sure that this rectangle is entirely valid.
    pub fn set_clipping(&mut self, clipping: Option<Rect>) {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .set_clipping(clipping);
    }

    /// Returns the current arbitrary clipping rectangle, None if there isn't one.
    pub fn get_clipping(&self) -> Option<Rect> {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .get_clipping()
    }

    /// Sets ALL tiles foreground alpha (only tiles that exist, in sparse consoles).
    pub fn set_all_fg_alpha(&mut self, alpha: f32) {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .set_all_fg_alpha(alpha);
    }

    /// Sets ALL tiles background alpha (only tiles that exist, in sparse consoles).
    pub fn set_all_bg_alpha(&mut self, alpha: f32) {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .set_all_bg_alpha(alpha);
    }

    /// Sets ALL tiles foreground alpha (only tiles that exist, in sparse consoles).
    pub fn set_all_alpha(&mut self, fg: f32, bg: f32) {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .set_all_alpha(fg, bg);
    }

    /// Sets the character translation mode on a console
    pub fn set_translation_mode(&mut self, console: usize, translation: CharacterTranslationMode) {
        BACKEND_INTERNAL.lock().consoles[console]
            .console
            .set_translation_mode(translation)
    }

    #[cfg(feature = "opengl")]
    /// Change the active font for the layer. DO NOT USE WITH AMETHYST YET.
    pub fn set_active_font(&mut self, font_index: usize, resize_to_natural_dimensions: bool) {
        let mut be = BACKEND_INTERNAL.lock();
        if font_index > be.fonts.len() {
            panic!("Font index out of bounds.");
        }
        let old_font_size = be.fonts[be.consoles[self.active_console].font_index].tile_size;
        let new_font_size = be.fonts[font_index].tile_size;
        be.consoles[self.active_console].font_index = font_index;

        if old_font_size != new_font_size && resize_to_natural_dimensions {
            let x_size = self.original_width_pixels / new_font_size.0;
            let y_size = self.original_height_pixels / new_font_size.1;

            be.consoles[self.active_console]
                .console
                .set_char_size(x_size, y_size);
        }
    }

    #[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
    /// Manually override the character size for the current terminal. Use with caution!
    pub fn set_char_size(&mut self, width: u32, height: u32) {
        BACKEND_INTERNAL.lock().consoles[self.active_console]
            .console
            .set_char_size(width, height);
    }

    #[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
    /// Manually override the character size for the current terminal. Use with caution!
    pub fn set_char_size_and_resize_window(&mut self, _width: u32, _height: u32) {
        /*
        let be = BACKEND_INTERNAL.lock();
        let font_size = be.fonts[be.consoles[0].font_index].tile_size;
        let w = font_size.0 * width;
        let h = font_size.1 * height;
        crate::prelude::BACKEND.lock().resize_request = Some((w, h));
        */
        //panic!("This will be supported when `winit` stops crashing on resize request.");
    }

    /// Take a screenshot - Native only
    #[cfg(all(feature = "opengl", not(target_arch = "wasm32")))]
    pub fn screenshot<S: ToString>(&mut self, filename: S) {
        BACKEND.lock().request_screenshot = Some(filename.to_string());
    }

    /// Take a screenshot - Native only
    #[cfg(not(all(feature = "opengl", not(target_arch = "wasm32"))))]
    pub fn screenshot<S: ToString>(&mut self, _filename: S) {
        // Do nothing
    }

    /// Register a sprite sheet (OpenGL - native or WASM - only)
    #[cfg(feature = "opengl")]
    pub fn register_spritesheet(&mut self, ss: SpriteSheet) -> usize {
        let mut bi = BACKEND_INTERNAL.lock();
        let id = bi.sprite_sheets.len();
        bi.sprite_sheets.push(ss);
        id
    }

    /// Add a sprite to the current console
    #[cfg(feature = "opengl")]
    pub fn add_sprite(&mut self, destination: Rect, z_order: i32, tint: RGBA, index: usize) {
        let mut bi = BACKEND_INTERNAL.lock();
        let as_any = bi.consoles[self.active_console].console.as_any_mut();
        if let Some(cons) = as_any.downcast_mut::<SpriteConsole>() {
            cons.render_sprite(RenderSprite {
                destination,
                z_order,
                tint,
                index,
            });
        }
    }
}

/// Runs the BTerm application, calling into the provided gamestate handler every tick.
pub fn main_loop<GS: GameState>(bterm: BTerm, gamestate: GS) -> Result<()> {
    super::hal::main_loop(bterm, gamestate)?;
    Ok(())
}

/// For A-Z menus, translates the keys A through Z into 0..25
pub fn letter_to_option(key: VirtualKeyCode) -> i32 {
    match key {
        VirtualKeyCode::A => 0,
        VirtualKeyCode::B => 1,
        VirtualKeyCode::C => 2,
        VirtualKeyCode::D => 3,
        VirtualKeyCode::E => 4,
        VirtualKeyCode::F => 5,
        VirtualKeyCode::G => 6,
        VirtualKeyCode::H => 7,
        VirtualKeyCode::I => 8,
        VirtualKeyCode::J => 9,
        VirtualKeyCode::K => 10,
        VirtualKeyCode::L => 11,
        VirtualKeyCode::M => 12,
        VirtualKeyCode::N => 13,
        VirtualKeyCode::O => 14,
        VirtualKeyCode::P => 15,
        VirtualKeyCode::Q => 16,
        VirtualKeyCode::R => 17,
        VirtualKeyCode::S => 18,
        VirtualKeyCode::T => 19,
        VirtualKeyCode::U => 20,
        VirtualKeyCode::V => 21,
        VirtualKeyCode::W => 22,
        VirtualKeyCode::X => 23,
        VirtualKeyCode::Y => 24,
        VirtualKeyCode::Z => 25,
        _ => -1,
    }
}

// Since num::clamp is still experimental, this is a simple integer clamper.
fn iclamp(val: i32, min: i32, max: i32) -> i32 {
    i32::max(min, i32::min(val, max))
}

#[cfg(test)]
mod tests {
    use super::iclamp;

    #[test]
    // Tests that we make an RGB triplet at defaults and it is black.
    fn test_iclamp() {
        assert!(iclamp(1, 0, 2) == 1);
        assert!(iclamp(5, 0, 2) == 2);
        assert!(iclamp(-5, 0, 2) == 0);
    }
}
