use crate::{
    prelude::{
        font::Font, init_raw, Console, GameState, InitHints, Shader, SimpleConsole, VirtualKeyCode,
        XpFile, XpLayer,
    },
    Result,
};
use bracket_color::prelude::RGB;
use bracket_geometry::prelude::{Point, Rect};
use std::any::Any;
use std::convert::TryInto;

/// A display console, used internally to provide console render support.
/// Public in case you want to play with it, or access it directly.
pub struct DisplayConsole {
    pub console: Box<dyn Console>,
    pub shader_index: usize,
    pub font_index: usize,
}

/// A BTerm context.
pub struct BTerm {
    pub width_pixels: u32,
    pub height_pixels: u32,
    pub fonts: Vec<Font>,
    pub shaders: Vec<Shader>,
    pub consoles: Vec<DisplayConsole>,
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
        let font = context.register_font(Font::load(&font_path.to_string(), (8, 8)));
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
        let font = context.register_font(Font::load(&font_path.to_string(), (8, 16)));
        context.register_console(SimpleConsole::init(w, h), font.unwrap());
        context
    }

    /// Registers a font, and returns its handle number. Also loads it into OpenGL.
    pub fn register_font(&mut self, font: Font) -> Result<usize> {
        self.fonts.push(font);
        Ok(self.fonts.len() - 1)
    }

    /// Registers a new console terminal for output, and returns its handle number.
    pub fn register_console(&mut self, new_console: Box<dyn Console>, font_index: usize) -> usize {
        self.consoles.push(DisplayConsole {
            console: new_console,
            font_index,
            shader_index: 0,
        });
        self.consoles.len() - 1
    }

    /// Registers a new console terminal for output, and returns its handle number. This variant requests
    /// that the new console not render background colors, so it can be layered on top of other consoles.
    pub fn register_console_no_bg(
        &mut self,
        new_console: Box<dyn Console>,
        font_index: usize,
    ) -> usize {
        self.consoles.push(DisplayConsole {
            console: new_console,
            font_index,
            shader_index: 1,
        });
        self.consoles.len() - 1
    }

    /// Sets the currently active console number.
    pub fn set_active_console(&mut self, id: usize) {
        self.active_console = id;
    }

    /// Applies the current physical mouse position to the active console, and translates the coordinates into that console's coordinate space.
    #[cfg(feature = "curses")]
    pub fn mouse_pos(&self) -> (i32, i32) {
        (self.mouse_pos.0, self.mouse_pos.1)
    }

    /// Applies the current physical mouse position to the active console, and translates the coordinates into that console's coordinate space.
    #[cfg(not(feature = "curses"))]
    pub fn mouse_pos(&self) -> (i32, i32) {
        let max_sizes = self.consoles[self.active_console].console.get_char_size();

        (
            iclamp(
                self.mouse_pos.0 * max_sizes.0 as i32 / i32::max(1, self.width_pixels as i32),
                0,
                max_sizes.0 as i32 - 1,
            ),
            iclamp(
                self.mouse_pos.1 * max_sizes.1 as i32 / i32::max(1, self.height_pixels as i32),
                0,
                max_sizes.1 as i32 - 1,
            ),
        )
    }

    /// Applies the current physical mouse position to the active console, and translates the coordinates into that console's coordinate space.
    pub fn mouse_point(&self) -> Point {
        let max_sizes = self.consoles[self.active_console].console.get_char_size();
        Point::new(
            iclamp(
                self.mouse_pos.0 * max_sizes.0 as i32 / self.width_pixels as i32,
                0,
                max_sizes.0 as i32 - 1,
            ),
            iclamp(
                self.mouse_pos.1 * max_sizes.1 as i32 / self.height_pixels as i32,
                0,
                max_sizes.1 as i32 - 1,
            ),
        )
    }

    /// Tells the game to quit
    pub fn quit(&mut self) {
        self.quitting = true;
    }

    /// Render a REX Paint (https://www.gridsagegames.com/rexpaint/) file as a sprite.
    /// The sprite will be offset by offset_x and offset_y.
    /// Transparent cells will not be rendered.
    pub fn render_xp_sprite(&mut self, xp: &super::rex::XpFile, x: i32, y: i32) {
        super::rex::xp_to_console(xp, &mut self.consoles[self.active_console].console, x, y);
    }

    /// Saves the entire console stack to a REX Paint xp file. If your consoles are of
    /// varying sizes, the file format supports it - but REX doesn't. So you may want to
    /// avoid that. You can also get individual layers with to_xp_layer.
    pub fn to_xp_file(&self, width: usize, height: usize) -> XpFile {
        let mut xp = XpFile::new(width, height);

        xp.layers
            .push(self.consoles[self.active_console].console.to_xp_layer());

        if self.consoles.len() > 1 {
            for layer in self.consoles.iter().skip(1) {
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
}

impl Console for BTerm {
    fn get_char_size(&self) -> (u32, u32) {
        self.consoles[self.active_console].console.get_char_size()
    }

    fn resize_pixels(&mut self, width: u32, height: u32) {
        self.width_pixels = width;
        self.height_pixels = height;

        for c in self.consoles.iter_mut() {
            c.console.resize_pixels(width, height);
        }
    }

    // Implement pass-through to active console

    fn at(&self, x: i32, y: i32) -> usize {
        self.consoles[self.active_console].console.at(x, y)
    }
    fn cls(&mut self) {
        self.consoles[self.active_console].console.cls();
    }
    fn cls_bg(&mut self, background: RGB) {
        self.consoles[self.active_console]
            .console
            .cls_bg(background);
    }
    fn print(&mut self, x: i32, y: i32, output: &str) {
        self.consoles[self.active_console]
            .console
            .print(x, y, output);
    }
    fn print_color(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, output: &str) {
        self.consoles[self.active_console]
            .console
            .print_color(x, y, fg, bg, output);
    }
    fn set(&mut self, x: i32, y: i32, fg: RGB, bg: RGB, glyph: u8) {
        self.consoles[self.active_console]
            .console
            .set(x, y, fg, bg, glyph);
    }
    fn set_bg(&mut self, x: i32, y: i32, bg: RGB) {
        self.consoles[self.active_console].console.set_bg(x, y, bg);
    }
    fn draw_box(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        self.consoles[self.active_console]
            .console
            .draw_box(x, y, width, height, fg, bg);
    }
    fn draw_box_double(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        self.consoles[self.active_console]
            .console
            .draw_box_double(x, y, width, height, fg, bg);
    }
    fn draw_hollow_box(&mut self, x: i32, y: i32, width: i32, height: i32, fg: RGB, bg: RGB) {
        self.consoles[self.active_console]
            .console
            .draw_hollow_box(x, y, width, height, fg, bg);
    }
    fn draw_hollow_box_double(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        fg: RGB,
        bg: RGB,
    ) {
        self.consoles[self.active_console]
            .console
            .draw_hollow_box_double(x, y, width, height, fg, bg);
    }
    fn draw_bar_horizontal(
        &mut self,
        x: i32,
        y: i32,
        width: i32,
        n: i32,
        max: i32,
        fg: RGB,
        bg: RGB,
    ) {
        self.consoles[self.active_console]
            .console
            .draw_bar_horizontal(x, y, width, n, max, fg, bg);
    }
    fn draw_bar_vertical(
        &mut self,
        x: i32,
        y: i32,
        height: i32,
        n: i32,
        max: i32,
        fg: RGB,
        bg: RGB,
    ) {
        self.consoles[self.active_console]
            .console
            .draw_bar_vertical(x, y, height, n, max, fg, bg);
    }
    fn fill_region(&mut self, target: Rect, glyph: u8, fg: RGB, bg: RGB) {
        self.consoles[self.active_console]
            .console
            .fill_region(target, glyph, fg, bg);
    }
    fn get(&self, x: i32, y: i32) -> Option<(&u8, &RGB, &RGB)> {
        self.consoles[self.active_console].console.get(x, y)
    }
    fn print_centered(&mut self, y: i32, text: &str) {
        self.consoles[self.active_console]
            .console
            .print_centered(y, text);
    }
    fn print_color_centered(&mut self, y: i32, fg: RGB, bg: RGB, text: &str) {
        self.consoles[self.active_console]
            .console
            .print_color_centered(y, fg, bg, text);
    }
    fn to_xp_layer(&self) -> XpLayer {
        self.consoles[self.active_console].console.to_xp_layer()
    }
    fn set_offset(&mut self, x: f32, y: f32) {
        self.consoles[self.active_console].console.set_offset(x, y);
    }
    fn set_scale(&mut self, scale: f32, center_x: i32, center_y: i32) {
        self.consoles[self.active_console]
            .console
            .set_scale(scale, center_x, center_y);
    }

    fn as_any(&self) -> &dyn Any {
        self
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
