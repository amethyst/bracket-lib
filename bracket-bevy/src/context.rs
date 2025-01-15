use crate::{
    consoles::{ConsoleFrontEnd, DrawBatch, DrawCommand, ScreenScaler},
    fonts::FontStore,
    FontCharType, TerminalScalingMode,
};
use bevy::{
    prelude::{Mesh2d, Resource},
    utils::HashMap,
};
use bracket_color::prelude::RGBA;
use bracket_geometry::prelude::{Point, Rect};
use parking_lot::Mutex;

#[derive(Resource)]
pub struct BracketContext {
    pub(crate) fonts: Vec<FontStore>,
    pub(crate) terminals: Mutex<Vec<Box<dyn ConsoleFrontEnd>>>,
    pub(crate) current_layer: Mutex<usize>,
    pub(crate) color_palette: HashMap<String, RGBA>,
    pub fps: f64,
    pub frame_time_ms: f64,
    pub(crate) mesh_replacement: Vec<(Mesh2d, Mesh2d, bool)>,
    pub(crate) scaling_mode: TerminalScalingMode,
    command_buffers: Mutex<Vec<(usize, DrawBatch)>>,
    mouse_pixels: (f32, f32),
}

impl BracketContext {
    pub(crate) fn new(color_palette: HashMap<String, RGBA>) -> Self {
        Self {
            fonts: Vec::new(),
            terminals: Mutex::new(Vec::new()),
            current_layer: Mutex::new(0),
            color_palette,
            fps: 0.0,
            frame_time_ms: 0.0,
            mesh_replacement: Vec::new(),
            scaling_mode: TerminalScalingMode::Stretch,
            command_buffers: Mutex::new(Vec::new()),
            mouse_pixels: (0.0, 0.0),
        }
    }

    #[inline(always)]
    fn current_layer(&self) -> usize {
        *self.current_layer.lock()
    }

    /// Retrieve the current console size, in characters.
    /// Applies to the currently active layer.
    pub fn get_char_size(&self) -> (i32, i32) {
        self.terminals.lock()[self.current_layer()].get_char_size()
    }

    /// Retrieve the largest natural pixel size from all layers.
    /// This is useful when scaling.
    pub fn get_pixel_size(&self) -> (f32, f32) {
        let mut pixel_size = (0.0, 0.0);
        self.terminals.lock().iter().for_each(|t| {
            let ts = t.get_pixel_size();
            pixel_size.0 = f32::max(pixel_size.0, ts.0);
            pixel_size.1 = f32::max(pixel_size.1, ts.1);
        });
        pixel_size
    }

    /// Retrieve the pixel size of the largest active font, across all layers.
    pub fn largest_font(&self) -> (f32, f32) {
        let mut result = (1.0, 1.0);
        self.fonts.iter().for_each(|fs| {
            result.0 = f32::max(result.0, fs.font_height_pixels.0);
            result.1 = f32::max(result.1, fs.font_height_pixels.1);
        });
        result
    }

    /// Retrieve the index of a character in the backing array at x/y.
    /// WARNING: this may return an invalid/non-existent index.
    pub fn at<POS: Into<i32>>(&self, x: POS, y: POS) -> usize {
        self.terminals.lock()[self.current_layer()].at(x.into(), y.into())
    }

    /// Try to obtain the index of a character in the terminal backing array at x/y.
    /// If the character is out-of-bounds or clipped out of usefulness, returns None.
    /// Otherwise, it returns Some(index)
    pub fn try_at<POS: Into<i32>>(&self, x: POS, y: POS) -> Option<usize> {
        self.terminals.lock()[self.current_layer()].try_at(x.into(), y.into())
    }

    /// Get the current clipping rectangle, or None if there isn't one.
    pub fn get_clipping(&self) -> Option<Rect> {
        self.terminals.lock()[self.current_layer()].get_clipping()
    }

    /// Set the current clipping rectange. Set to None if you don't want one.
    pub fn set_clipping(&self, clipping: Option<Rect>) {
        self.terminals.lock()[self.current_layer()].set_clipping(clipping);
    }

    /// Set the current layer index.
    pub fn set_active_console(&self, layer: usize) {
        *self.current_layer.lock() = layer;
    }

    /// Remove all entries from the current layer.
    pub fn cls(&self) {
        self.terminals.lock()[self.current_layer()].cls();
    }

    /// Set the background on the current layer to a uniform color.
    /// Only useful for Simple consoles.
    pub fn cls_bg<C: Into<RGBA>>(&self, color: C) {
        self.terminals.lock()[self.current_layer()].cls_bg(color.into());
    }

    /// Set a character at (x,y) to a specified foreground, background and glyph.
    pub fn set<POS: Into<i32>, C: Into<RGBA>>(
        &self,
        x: POS,
        y: POS,
        fg: C,
        bg: C,
        glyph: FontCharType,
    ) {
        self.terminals.lock()[self.current_layer()].set(
            x.into(),
            y.into(),
            fg.into(),
            bg.into(),
            glyph,
        );
    }

    /// Set just the background color of a terminal cell.
    pub fn set_bg<POS: Into<i32>, C: Into<RGBA>>(&self, x: POS, y: POS, bg: C) {
        self.terminals.lock()[self.current_layer()].set_bg(x.into(), y.into(), bg.into());
    }

    /// Print some text to the currently active terminal.
    pub fn print<POS: Into<i32>, S: ToString>(&self, x: POS, y: POS, text: S) {
        self.terminals.lock()[self.current_layer()].print(x.into(), y.into(), &text.to_string());
    }

    /// Print some text, centered along the `y` line, to the current terminal.
    pub fn print_centered<POS: Into<i32>, S: ToString>(&self, y: POS, text: S) {
        self.terminals.lock()[self.current_layer()].print_centered(y.into(), &text.to_string());
    }

    /// Print some text, centered along the `y` line, to the current terminal in the specified color.
    pub fn print_color_centered<POS: Into<i32>, S: ToString, C: Into<RGBA>>(
        &self,
        y: POS,
        fg: C,
        bg: C,
        text: S,
    ) {
        self.terminals.lock()[self.current_layer()].print_color_centered(
            y.into(),
            fg.into(),
            bg.into(),
            &text.to_string(),
        );
    }

    /// Print some text, centered around (x, y) to the current terminal.
    pub fn print_centered_at<POS: Into<i32>, S: ToString>(&self, x: POS, y: POS, text: S) {
        self.terminals.lock()[self.current_layer()].print_centered_at(
            x.into(),
            y.into(),
            &text.to_string(),
        );
    }

    /// Print some text, cenetered around (x,y) to the current terminal in the specified colors.
    pub fn print_color_centered_at<POS: Into<i32>, S: ToString, C: Into<RGBA>>(
        &self,
        x: POS,
        y: POS,
        fg: C,
        bg: C,
        text: S,
    ) {
        self.terminals.lock()[self.current_layer()].print_color_centered_at(
            x.into(),
            y.into(),
            fg.into(),
            bg.into(),
            &text.to_string(),
        )
    }

    /// Print some text, right justified around (x,y), to the current terminal layer.
    pub fn print_right<POS: Into<i32>, S: ToString>(&self, x: POS, y: POS, text: S) {
        self.terminals.lock()[self.current_layer()].print_right(
            x.into(),
            y.into(),
            &text.to_string(),
        );
    }

    /// Print some text, right justified at (x,y), to the current terminal layer in the specified colors.
    pub fn print_color_right<POS: Into<i32>, S: ToString, C: Into<RGBA>>(
        &self,
        x: POS,
        y: POS,
        fg: C,
        bg: C,
        text: S,
    ) {
        self.terminals.lock()[self.current_layer()].print_color_right(
            x.into(),
            y.into(),
            fg.into(),
            bg.into(),
            &text.to_string(),
        );
    }

    /// Print some text in color at the specified (x,y) coordinates.
    pub fn print_color<POS: Into<i32>, S: ToString, C: Into<RGBA>>(
        &self,
        x: POS,
        y: POS,
        text: S,
        foreground: C,
        background: C,
    ) {
        self.terminals.lock()[self.current_layer()].print_color(
            x.into(),
            y.into(),
            &text.to_string(),
            foreground.into(),
            background.into(),
        )
    }

    /// Use the pretty-printer to format text for the screen.
    pub fn printer<POS: Into<i32>, S: ToString>(
        &self,
        x: POS,
        y: POS,
        output: S,
        align: crate::consoles::TextAlign,
        background: Option<RGBA>,
    ) {
        self.terminals.lock()[self.current_layer()].printer(
            self,
            x.into(),
            y.into(),
            &output.to_string(),
            align,
            background,
        );
    }

    /// Draws a filled box, with single line characters.
    pub fn draw_box<POS: Into<i32>, C: Into<RGBA>>(
        &self,
        x: POS,
        y: POS,
        width: POS,
        height: POS,
        fg: C,
        bg: C,
    ) {
        self.terminals.lock()[self.current_layer()].draw_box(
            x.into(),
            y.into(),
            width.into(),
            height.into(),
            fg.into(),
            bg.into(),
        );
    }

    /// Draw a hollow box with single line characters.
    pub fn draw_hollow_box<POS: Into<i32>, C: Into<RGBA>>(
        &self,
        x: POS,
        y: POS,
        width: POS,
        height: POS,
        fg: C,
        bg: C,
    ) {
        self.terminals.lock()[self.current_layer()].draw_hollow_box(
            x.into(),
            y.into(),
            width.into(),
            height.into(),
            fg.into(),
            bg.into(),
        );
    }

    /// Draw a filled box with double-line characters.
    pub fn draw_box_double<POS: Into<i32>, C: Into<RGBA>>(
        &self,
        x: POS,
        y: POS,
        width: POS,
        height: POS,
        fg: C,
        bg: C,
    ) {
        self.terminals.lock()[self.current_layer()].draw_box_double(
            x.into(),
            y.into(),
            width.into(),
            height.into(),
            fg.into(),
            bg.into(),
        );
    }

    /// Draw an empty box with double-line characters.
    pub fn draw_hollow_box_double<POS: Into<i32>, C: Into<RGBA>>(
        &self,
        x: POS,
        y: POS,
        width: POS,
        height: POS,
        fg: C,
        bg: C,
    ) {
        self.terminals.lock()[self.current_layer()].draw_hollow_box_double(
            x.into(),
            y.into(),
            width.into(),
            height.into(),
            fg.into(),
            bg.into(),
        );
    }

    /// Fill a region specified by a rectangle with a specified glyph, and colors.
    pub fn fill_region<C: Into<RGBA>>(&self, target: Rect, glyph: FontCharType, fg: C, bg: C) {
        self.terminals.lock()[self.current_layer()].fill_region(
            target,
            glyph,
            fg.into(),
            bg.into(),
        );
    }

    /// Draw a horizontal progress bar.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_bar_horizontal<POS: Into<i32>, C: Into<RGBA>>(
        &self,
        x: POS,
        y: POS,
        width: POS,
        n: POS,
        max: POS,
        fg: C,
        bg: C,
    ) {
        self.terminals.lock()[self.current_layer()].draw_bar_horizontal(
            x.into(),
            y.into(),
            width.into(),
            n.into(),
            max.into(),
            fg.into(),
            bg.into(),
        );
    }

    /// Draw a vertical progress bar.
    #[allow(clippy::too_many_arguments)]
    pub fn draw_bar_vertical<POS: Into<i32>, C: Into<RGBA>>(
        &self,
        x: POS,
        y: POS,
        height: POS,
        n: POS,
        max: POS,
        fg: C,
        bg: C,
    ) {
        self.terminals.lock()[self.current_layer()].draw_bar_vertical(
            x.into(),
            y.into(),
            height.into(),
            n.into(),
            max.into(),
            fg.into(),
            bg.into(),
        );
    }

    /// Sets the alpha level on all foreground characters on the current layer.
    pub fn set_all_fg_alpha(&self, alpha: f32) {
        self.terminals.lock()[self.current_layer()].set_all_bg_alpha(alpha);
    }

    /// Sets the alpha level on all background characters on the current layer.
    pub fn set_all_bg_alpha(&self, alpha: f32) {
        self.terminals.lock()[self.current_layer()].set_all_bg_alpha(alpha);
    }

    /// Sets foreground and background alpha on the current player.
    pub fn set_all_alpha(&self, fg: f32, bg: f32) {
        self.terminals.lock()[self.current_layer()].set_all_alpha(fg, bg);
    }

    /// Retrieve a named color from the palette.
    /// Note that this replaces the `bracket_color` palette; there were performance problems
    /// using it on Bevy.
    pub fn get_named_color(&self, color: &str) -> Option<&RGBA> {
        self.color_palette.get(color)
    }

    pub(crate) fn resize_terminals(&mut self, scaler: &ScreenScaler) {
        let available_size = scaler.available_size();
        let mut lock = self.terminals.lock();
        lock.iter_mut().for_each(|t| t.resize(&available_size));
    }

    /// Create a new draw batch. Note that this is now a context variable,
    /// since contexts are `Res` not `ResMut` - and consequently don't
    /// affect scheduling.
    pub fn new_draw_batch(&self) -> DrawBatch {
        DrawBatch::new()
    }

    /// Submit a batch for rendering.
    pub fn submit_batch(&self, z_order: usize, mut batch: DrawBatch) {
        if batch.needs_sort {
            batch.batch.sort_by(|a, b| a.0.cmp(&b.0));
        }
        self.command_buffers.lock().push((z_order, batch));
    }

    /// Submit all draw batches for rendering.
    pub fn render_all_batches(&mut self) {
        let mut batches = self.command_buffers.lock();
        batches.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        batches.iter().for_each(|(_, batch)| {
            batch.batch.iter().for_each(|(_, cmd)| match cmd {
                DrawCommand::ClearScreen => self.cls(),
                DrawCommand::ClearToColor { color } => self.cls_bg(*color),
                DrawCommand::SetTarget { console } => self.set_active_console(*console),
                DrawCommand::Set { pos, color, glyph } => {
                    self.set(pos.x, pos.y, color.fg, color.bg, *glyph)
                }
                DrawCommand::SetBackground { pos, bg } => self.set_bg(pos.x, pos.y, *bg),
                DrawCommand::Print { pos, text } => self.print(pos.x, pos.y, &text),
                DrawCommand::PrintColor { pos, text, color } => {
                    self.print_color(pos.x, pos.y, &text, color.fg, color.bg)
                }
                DrawCommand::PrintCentered { y, text } => self.print_centered(*y, &text),
                DrawCommand::PrintColorCentered { y, text, color } => {
                    self.print_color_centered(*y, color.fg, color.bg, &text)
                }
                DrawCommand::PrintCenteredAt { pos, text } => {
                    self.print_centered_at(pos.x, pos.y, &text)
                }
                DrawCommand::PrintColorCenteredAt { pos, text, color } => {
                    self.print_color_centered_at(pos.x, pos.y, color.fg, color.bg, &text)
                }
                DrawCommand::PrintRight { pos, text } => self.print_right(pos.x, pos.y, text),
                DrawCommand::PrintColorRight { pos, text, color } => {
                    self.print_color_right(pos.x, pos.y, color.fg, color.bg, text)
                }
                DrawCommand::Printer {
                    pos,
                    text,
                    align,
                    background,
                } => self.printer(pos.x, pos.y, text, *align, *background),
                DrawCommand::Box { pos, color } => self.draw_box(
                    pos.x1,
                    pos.y1,
                    pos.width(),
                    pos.height(),
                    color.fg,
                    color.bg,
                ),
                DrawCommand::HollowBox { pos, color } => self.draw_hollow_box(
                    pos.x1,
                    pos.y1,
                    pos.width(),
                    pos.height(),
                    color.fg,
                    color.bg,
                ),
                DrawCommand::DoubleBox { pos, color } => self.draw_box_double(
                    pos.x1,
                    pos.y1,
                    pos.width(),
                    pos.height(),
                    color.fg,
                    color.bg,
                ),
                DrawCommand::HollowDoubleBox { pos, color } => self.draw_hollow_box_double(
                    pos.x1,
                    pos.y1,
                    pos.width(),
                    pos.height(),
                    color.fg,
                    color.bg,
                ),
                DrawCommand::FillRegion { pos, color, glyph } => {
                    self.fill_region::<RGBA>(*pos, *glyph, color.fg, color.bg)
                }
                DrawCommand::BarHorizontal {
                    pos,
                    width,
                    n,
                    max,
                    color,
                } => self.draw_bar_horizontal(pos.x, pos.y, *width, *n, *max, color.fg, color.bg),
                DrawCommand::BarVertical {
                    pos,
                    height,
                    n,
                    max,
                    color,
                } => self.draw_bar_vertical(pos.x, pos.y, *height, *n, *max, color.fg, color.bg),
                DrawCommand::SetClipping { clip } => self.set_clipping(*clip),
                DrawCommand::SetFgAlpha { alpha } => self.set_all_fg_alpha(*alpha),
                DrawCommand::SetBgAlpha { alpha } => self.set_all_fg_alpha(*alpha),
                DrawCommand::SetAllAlpha { fg, bg } => self.set_all_alpha(*fg, *bg),
            })
        });

        batches.clear();
    }

    pub(crate) fn set_mouse_pixel_position(&mut self, pos: (f32, f32), scaler: &ScreenScaler) {
        self.mouse_pixels = pos;
        self.terminals
            .lock()
            .iter_mut()
            .for_each(|t| t.set_mouse_position(pos, scaler));
    }

    pub fn get_mouse_position_in_pixels(&self) -> (f32, f32) {
        self.mouse_pixels
    }

    pub fn get_mouse_position_for_current_layer(&self) -> Point {
        self.terminals.lock()[self.current_layer()].get_mouse_position_for_current_layer()
    }
}
