// This package implements an alternate style of drawing to the console,
// designed to be used safely from within ECS systems in a potentially
// multi-threaded environment.

use crate::prelude::{BTerm, FontCharType, TextAlign};
use crate::Result;
use bracket_color::prelude::{ColorPair, RGBA};
use bracket_geometry::prelude::{Point, PointF, Radians, Rect};
use object_pool::{Pool, Reusable};
use parking_lot::Mutex;
use std::convert::TryInto;
use std::sync::Arc;

lazy_static! {
    static ref COMMAND_BUFFER: Mutex<Vec<(usize, DrawCommand)>> =
        Mutex::new(Vec::with_capacity(10000));
}

lazy_static! {
    static ref BUFFER_POOL: Arc<Pool<DrawBatch>> =
        Arc::new(Pool::new(128, || DrawBatch {
            batch: Vec::with_capacity(5000)
        }));
}

/// Clears the global command buffer. This is called internally by BTerm at the end of each
/// frame. You really shouldn't need to call this yourself.
pub fn clear_command_buffer() -> Result<()> {
    COMMAND_BUFFER.lock().clear();
    Ok(())
}

/// Represents a buffered drawing command that can be asynchronously submitted to the drawing
/// buffer, for application at the end of the frame.
#[derive(Clone)]
pub enum DrawCommand {
    ClearScreen,
    ClearToColor {
        color: RGBA,
    },
    SetTarget {
        console: usize,
    },
    Set {
        pos: Point,
        color: ColorPair,
        glyph: FontCharType,
    },
    SetBackground {
        pos: Point,
        bg: RGBA,
    },
    Print {
        pos: Point,
        text: String,
    },
    PrintColor {
        pos: Point,
        text: String,
        color: ColorPair,
    },
    PrintRight {
        pos: Point,
        text: String,
    },
    PrintColorRight {
        pos: Point,
        text: String,
        color: ColorPair,
    },
    PrintCentered {
        y: i32,
        text: String,
    },
    PrintColorCentered {
        y: i32,
        text: String,
        color: ColorPair,
    },
    PrintCenteredAt {
        pos: Point,
        text: String,
    },
    PrintColorCenteredAt {
        pos: Point,
        text: String,
        color: ColorPair,
    },
    Printer {
        pos: Point,
        text: String,
        align: TextAlign,
        background: Option<RGBA>,
    },
    Box {
        pos: Rect,
        color: ColorPair,
    },
    HollowBox {
        pos: Rect,
        color: ColorPair,
    },
    DoubleBox {
        pos: Rect,
        color: ColorPair,
    },
    HollowDoubleBox {
        pos: Rect,
        color: ColorPair,
    },
    FillRegion {
        pos: Rect,
        color: ColorPair,
        glyph: FontCharType,
    },
    BarHorizontal {
        pos: Point,
        width: i32,
        n: i32,
        max: i32,
        color: ColorPair,
    },
    BarVertical {
        pos: Point,
        height: i32,
        n: i32,
        max: i32,
        color: ColorPair,
    },
    SetClipping {
        clip: Option<Rect>,
    },
    SetFgAlpha {
        alpha: f32,
    },
    SetBgAlpha {
        alpha: f32,
    },
    SetAllAlpha {
        fg: f32,
        bg: f32,
    },
    SetFancy {
        position: PointF,
        z_order: i32,
        rotation: Radians,
        color: ColorPair,
        glyph: FontCharType,
        scale: PointF,
    },
}

/// Represents a batch of drawing commands, designed to be submitted together.
pub struct DrawBatch {
    batch: Vec<(usize, DrawCommand)>,
}

impl DrawBatch {
    /// Obtain a new, empty draw batch
    pub fn new() -> Reusable<'static, DrawBatch> {
        BUFFER_POOL.pull(|| { panic!("No pooling!") })
    }

    /// Submits a batch to the global drawing buffer, and empties the batch.
    pub fn submit(&mut self, z_order: usize) -> Result<()> {
        self.batch.iter_mut().enumerate().for_each(|(i, (z, _))| {
            *z = z_order + i;
        });
        COMMAND_BUFFER.lock().append(&mut self.batch);
        Ok(())
    }

    /// Adds a CLS (clear screen) to the drawing batch
    pub fn cls(&mut self) -> &mut Self {
        self.batch.push((0, DrawCommand::ClearScreen));
        self
    }

    /// Adds a CLS (clear screen) to the drawing batch
    pub fn cls_color<COLOR>(&mut self, color: COLOR) -> &mut Self
    where
        COLOR: Into<RGBA>,
    {
        self.batch.push((
            0,
            DrawCommand::ClearToColor {
                color: color.into(),
            },
        ));
        self
    }

    /// Sets the target console for rendering
    pub fn target(&mut self, console: usize) -> &mut Self {
        self.batch.push((0, DrawCommand::SetTarget { console }));
        self
    }

    /// Sets an individual cell glyph
    pub fn set<G: TryInto<FontCharType>>(
        &mut self,
        pos: Point,
        color: ColorPair,
        glyph: G,
    ) -> &mut Self {
        self.batch.push((
            0,
            DrawCommand::Set {
                pos,
                color,
                glyph: glyph.try_into().ok().expect("Must be u16 convertible"),
            },
        ));
        self
    }

    /// Pushes a fancy terminal character
    pub fn set_fancy<ANGLE: Into<Radians>, Z: TryInto<i32>, G: TryInto<FontCharType>>(
        &mut self,
        position: PointF,
        z_order: Z,
        rotation: ANGLE,
        scale: PointF,
        color: ColorPair,
        glyph: G,
    ) -> &mut Self {
        self.batch.push((
            0,
            DrawCommand::SetFancy {
                position,
                z_order: z_order.try_into().ok().expect("Must be i32 convertible"),
                rotation: rotation.into(),
                color,
                glyph: glyph.try_into().ok().expect("Must be u16 convertible"),
                scale,
            },
        ));
        self
    }

    /// Sets an individual cell glyph
    pub fn set_bg<COLOR>(&mut self, pos: Point, bg: COLOR) -> &mut Self
    where
        COLOR: Into<RGBA>,
    {
        self.batch
            .push((0, DrawCommand::SetBackground { pos, bg: bg.into() }));
        self
    }

    /// Prints formatted text, using the doryen_rs convention. For example:
    /// "#[blue]This blue text contains a #[pink]pink#[] word"
    pub fn printer<S: ToString>(
        &mut self,
        pos: Point,
        text: S,
        align: TextAlign,
        background: Option<RGBA>,
    ) -> &mut Self {
        self.batch.push((
            0,
            DrawCommand::Printer {
                pos,
                text: text.to_string(),
                align,
                background,
            },
        ));
        self
    }

    /// Prints text in the default colors at a given location
    pub fn print<S: ToString>(&mut self, pos: Point, text: S) -> &mut Self {
        self.batch.push((
            0,
            DrawCommand::Print {
                pos,
                text: text.to_string(),
            },
        ));
        self
    }

    /// Prints text in the default colors at a given location
    pub fn print_color<S: ToString>(&mut self, pos: Point, text: S, color: ColorPair) -> &mut Self {
        self.batch.push((
            0,
            DrawCommand::PrintColor {
                pos,
                text: text.to_string(),
                color,
            },
        ));
        self
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    pub fn print_centered<S: ToString, Y: TryInto<i32>>(&mut self, y: Y, text: S) -> &mut Self {
        self.batch.push((
            0,
            DrawCommand::PrintCentered {
                y: y.try_into().ok().expect("Must be i32 convertible"),
                text: text.to_string(),
            },
        ));
        self
    }
    /// Prints text, centered to the whole console width, at vertical location y.
    pub fn print_color_centered<S: ToString, Y: TryInto<i32>>(
        &mut self,
        y: Y,
        text: S,
        color: ColorPair,
    ) -> &mut Self {
        self.batch.push((
            0,
            DrawCommand::PrintColorCentered {
                y: y.try_into().ok().expect("Must be i32 convertible"),
                text: text.to_string(),
                color,
            },
        ));
        self
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    pub fn print_centered_at<S: ToString>(&mut self, pos: Point, text: S) -> &mut Self {
        self.batch.push((
            0,
            DrawCommand::PrintCenteredAt {
                pos,
                text: text.to_string(),
            },
        ));
        self
    }
    /// Prints text, centered to the whole console width, at vertical location y.
    pub fn print_color_centered_at<S: ToString>(
        &mut self,
        pos: Point,
        text: S,
        color: ColorPair,
    ) -> &mut Self {
        self.batch.push((
            0,
            DrawCommand::PrintColorCenteredAt {
                pos,
                text: text.to_string(),
                color,
            },
        ));
        self
    }

    /// Prints right aligned text
    pub fn print_right<S: ToString>(&mut self, pos: Point, text: S) -> &mut Self {
        self.batch.push((
            0,
            DrawCommand::PrintRight {
                pos,
                text: text.to_string(),
            },
        ));
        self
    }

    /// Prints right aligned text
    pub fn print_color_right<S: ToString>(
        &mut self,
        pos: Point,
        text: S,
        color: ColorPair,
    ) -> &mut Self {
        self.batch.push((
            0,
            DrawCommand::PrintColorRight {
                pos,
                text: text.to_string(),
                color,
            },
        ));
        self
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_box(&mut self, pos: Rect, color: ColorPair) -> &mut Self {
        self.batch.push((0, DrawCommand::Box { pos, color }));
        self
    }

    /// Draws a non-filled (hollow) box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_hollow_box(&mut self, pos: Rect, color: ColorPair) -> &mut Self {
        self.batch.push((0, DrawCommand::HollowBox { pos, color }));
        self
    }

    /// Draws a double-lined box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_double_box(&mut self, pos: Rect, color: ColorPair) -> &mut Self {
        self.batch.push((0, DrawCommand::DoubleBox { pos, color }));
        self
    }

    /// Draws a non-filled (hollow) double-lined box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_hollow_double_box(&mut self, pos: Rect, color: ColorPair) -> &mut Self {
        self.batch
            .push((0, DrawCommand::HollowDoubleBox { pos, color }));
        self
    }

    /// Fills a region with a glyph/color combination.
    pub fn fill_region<G: TryInto<FontCharType>>(
        &mut self,
        pos: Rect,
        color: ColorPair,
        glyph: G,
    ) -> &mut Self {
        self.batch.push((
            0,
            DrawCommand::FillRegion {
                pos,
                color,
                glyph: glyph.try_into().ok().expect("Must be u16 convertible"),
            },
        ));
        self
    }

    /// Draw a horizontal progress bar
    pub fn bar_horizontal<W, N, MAX>(
        &mut self,
        pos: Point,
        width: W,
        n: N,
        max: MAX,
        color: ColorPair,
    ) -> &mut Self
    where
        W: TryInto<i32>,
        N: TryInto<i32>,
        MAX: TryInto<i32>,
    {
        self.batch.push((
            0,
            DrawCommand::BarHorizontal {
                pos,
                width: width.try_into().ok().expect("Must be i32 convertible"),
                n: n.try_into().ok().expect("Must be i32 convertible"),
                max: max.try_into().ok().expect("Must be i32 convertible"),
                color,
            },
        ));
        self
    }

    /// Draw a horizontal progress bar
    pub fn bar_vertical<H, N, MAX>(
        &mut self,
        pos: Point,
        height: H,
        n: N,
        max: MAX,
        color: ColorPair,
    ) -> &mut Self
    where
        H: TryInto<i32>,
        N: TryInto<i32>,
        MAX: TryInto<i32>,
    {
        self.batch.push((
            0,
            DrawCommand::BarVertical {
                pos,
                height: height.try_into().ok().expect("Must be i32 convertible"),
                n: n.try_into().ok().expect("Must be i32 convertible"),
                max: max.try_into().ok().expect("Must be i32 convertible"),
                color,
            },
        ));
        self
    }

    /// Sets a clipping rectangle for the current console
    pub fn set_clipping(&mut self, clip: Option<Rect>) -> &mut Self {
        self.batch.push((0, DrawCommand::SetClipping { clip }));
        self
    }

    /// Apply an alpha channel value to all cells' foregrounds in the current terminal.
    pub fn set_all_fg_alpha(&mut self, alpha: f32) -> &mut Self {
        self.batch.push((0, DrawCommand::SetFgAlpha { alpha }));
        self
    }

    /// Apply an alpha channel value to all cells' backgrounds in the current terminal.
    pub fn set_all_bg_alpha(&mut self, alpha: f32) -> &mut Self {
        self.batch.push((0, DrawCommand::SetBgAlpha { alpha }));
        self
    }

    /// Apply fg/bg alpha channel values to all cells in the current terminal.
    pub fn set_all_alpha(&mut self, fg: f32, bg: f32) -> &mut Self {
        self.batch.push((0, DrawCommand::SetAllAlpha { fg, bg }));
        self
    }
}

/// Submits the current batch to the BTerm buffer and empties it
pub fn render_draw_buffer(bterm: &mut BTerm) -> Result<()> {
    let mut buffer = COMMAND_BUFFER.lock();
    buffer.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    buffer.iter().for_each(|(_, cmd)| match cmd {
        DrawCommand::ClearScreen => bterm.cls(),
        DrawCommand::ClearToColor { color } => bterm.cls_bg(*color),
        DrawCommand::SetTarget { console } => bterm.set_active_console(*console),
        DrawCommand::Set { pos, color, glyph } => {
            bterm.set(pos.x, pos.y, color.fg, color.bg, *glyph)
        }
        DrawCommand::SetBackground { pos, bg } => bterm.set_bg(pos.x, pos.y, *bg),
        DrawCommand::Print { pos, text } => bterm.print(pos.x, pos.y, &text),
        DrawCommand::PrintColor { pos, text, color } => {
            bterm.print_color(pos.x, pos.y, color.fg, color.bg, &text)
        }
        DrawCommand::PrintCentered { y, text } => bterm.print_centered(*y, &text),
        DrawCommand::PrintColorCentered { y, text, color } => {
            bterm.print_color_centered(*y, color.fg, color.bg, &text)
        }
        DrawCommand::PrintCenteredAt { pos, text } => bterm.print_centered_at(pos.x, pos.y, &text),
        DrawCommand::PrintColorCenteredAt { pos, text, color } => {
            bterm.print_color_centered_at(pos.x, pos.y, color.fg, color.bg, &text)
        }
        DrawCommand::PrintRight { pos, text } => bterm.print_right(pos.x, pos.y, text),
        DrawCommand::PrintColorRight { pos, text, color } => {
            bterm.print_color_right(pos.x, pos.y, color.fg, color.bg, text)
        }
        DrawCommand::Printer {
            pos,
            text,
            align,
            background,
        } => bterm.printer(pos.x, pos.y, text, *align, *background),
        DrawCommand::Box { pos, color } => bterm.draw_box(
            pos.x1,
            pos.y1,
            pos.width(),
            pos.height(),
            color.fg,
            color.bg,
        ),
        DrawCommand::HollowBox { pos, color } => bterm.draw_hollow_box(
            pos.x1,
            pos.y1,
            pos.width(),
            pos.height(),
            color.fg,
            color.bg,
        ),
        DrawCommand::DoubleBox { pos, color } => bterm.draw_box_double(
            pos.x1,
            pos.y1,
            pos.width(),
            pos.height(),
            color.fg,
            color.bg,
        ),
        DrawCommand::HollowDoubleBox { pos, color } => bterm.draw_hollow_box_double(
            pos.x1,
            pos.y1,
            pos.width(),
            pos.height(),
            color.fg,
            color.bg,
        ),
        DrawCommand::FillRegion { pos, color, glyph } => {
            bterm.fill_region::<RGBA, RGBA, FontCharType>(*pos, *glyph, color.fg, color.bg)
        }
        DrawCommand::BarHorizontal {
            pos,
            width,
            n,
            max,
            color,
        } => bterm.draw_bar_horizontal(pos.x, pos.y, *width, *n, *max, color.fg, color.bg),
        DrawCommand::BarVertical {
            pos,
            height,
            n,
            max,
            color,
        } => bterm.draw_bar_vertical(pos.x, pos.y, *height, *n, *max, color.fg, color.bg),
        DrawCommand::SetClipping { clip } => bterm.set_clipping(*clip),
        DrawCommand::SetFgAlpha { alpha } => bterm.set_all_fg_alpha(*alpha),
        DrawCommand::SetBgAlpha { alpha } => bterm.set_all_fg_alpha(*alpha),
        DrawCommand::SetAllAlpha { fg, bg } => bterm.set_all_alpha(*fg, *bg),
        DrawCommand::SetFancy {
            position,
            z_order,
            color,
            glyph,
            rotation,
            scale,
        } => {
            bterm.set_fancy(
                *position, *z_order, *rotation, *scale, color.fg, color.bg, *glyph,
            );
        }
    });
    buffer.clear();
    Ok(())
}
