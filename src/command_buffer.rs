// This package implements an alternate style of drawing to the console,
// designed to be used safely from within ECS systems in a potentially
// multi-threaded environment.

use crate::{ColorPair, Console, Point, Rect, Rltk, RGB};
use std::sync::Mutex;

lazy_static! {
    static ref COMMAND_BUFFER: Mutex<Vec<DrawCommand>> = Mutex::new(Vec::new());
}

/// Clears the global command buffer. This is called internally by RLTK at the end of each
/// frame. You really shouldn't need to call this yourself.
pub fn clear_command_buffer() {
    COMMAND_BUFFER.lock().unwrap().clear();
}

/// Represents a buffered drawing command that can be asynconously submitted to the drawing
/// buffer, for application at the end of the frame.
pub enum DrawCommand {
    ClearScreen,
    ClearToColor {
        color: RGB,
    },
    SetTarget {
        console: usize,
    },
    Set {
        pos: Point,
        color: ColorPair,
        glyph: u8,
    },
    SetBackground {
        pos: Point,
        bg: RGB,
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
    PrintCentered {
        y: i32,
        text: String,
    },
    PrintColorCentered {
        y: i32,
        text: String,
        color: ColorPair,
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
        glyph: u8,
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
}

/// Represents a batch of drawing commands, designed to be submitted together.
pub struct DrawBatch {
    batch: Vec<DrawCommand>,
}

impl DrawBatch {
    /// Obtain a new, empty draw batch
    pub fn new() -> Self {
        DrawBatch { batch: Vec::new() }
    }

    /// Submits a batch to the global drawing buffer, and empties the batch.
    pub fn submit(&mut self) {
        COMMAND_BUFFER.lock().unwrap().append(&mut self.batch);
    }

    /// Adds a CLS (clear screen) to the drawing batch
    pub fn cls(&mut self) -> &mut Self {
        self.batch.push(DrawCommand::ClearScreen);
        self
    }

    /// Adds a CLS (clear screen) to the drawing batch
    pub fn cls_color(&mut self, color: RGB) -> &mut Self {
        self.batch.push(DrawCommand::ClearToColor { color });
        self
    }

    /// Sets the target console for rendering
    pub fn target(&mut self, console: usize) -> &mut Self {
        self.batch.push(DrawCommand::SetTarget { console });
        self
    }

    /// Sets an individual cell glyph
    pub fn set(&mut self, pos: Point, color: ColorPair, glyph: u8) -> &mut Self {
        self.batch.push(DrawCommand::Set { pos, color, glyph });
        self
    }

    /// Sets an individual cell glyph
    pub fn set_bg(&mut self, pos: Point, bg: RGB) -> &mut Self {
        self.batch.push(DrawCommand::SetBackground { pos, bg });
        self
    }

    /// Prints text in the default colors at a given location
    pub fn print<S: ToString>(&mut self, pos: Point, text: S) -> &mut Self {
        self.batch.push(DrawCommand::Print {
            pos,
            text: text.to_string(),
        });
        self
    }

    /// Prints text in the default colors at a given location
    pub fn print_color<S: ToString>(&mut self, pos: Point, text: S, color: ColorPair) -> &mut Self {
        self.batch.push(DrawCommand::PrintColor {
            pos,
            text: text.to_string(),
            color,
        });
        self
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    pub fn print_centered<S: ToString>(&mut self, y: i32, text: S) -> &mut Self {
        self.batch.push(DrawCommand::PrintCentered {
            y,
            text: text.to_string(),
        });
        self
    }
    /// Prints text, centered to the whole console width, at vertical location y.
    pub fn print_color_centered<S: ToString>(
        &mut self,
        y: i32,
        text: S,
        color: ColorPair,
    ) -> &mut Self {
        self.batch.push(DrawCommand::PrintColorCentered {
            y,
            text: text.to_string(),
            color,
        });
        self
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_box(&mut self, pos: Rect, color: ColorPair) -> &mut Self {
        self.batch.push(DrawCommand::Box { pos, color });
        self
    }

    /// Draws a non-filled (hollow) box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_hollow_box(&mut self, pos: Rect, color: ColorPair) -> &mut Self {
        self.batch.push(DrawCommand::HollowBox { pos, color });
        self
    }

    /// Draws a double-lined box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_double_box(&mut self, pos: Rect, color: ColorPair) -> &mut Self {
        self.batch.push(DrawCommand::DoubleBox { pos, color });
        self
    }

    /// Draws a non-filled (hollow) double-lined box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_hollow_double_box(&mut self, pos: Rect, color: ColorPair) -> &mut Self {
        self.batch.push(DrawCommand::HollowDoubleBox { pos, color });
        self
    }

    /// Draws a non-filled (hollow) double-lined box, starting at x/y with the extents width/height using CP437 line characters
    pub fn fill_region(&mut self, pos: Rect, color: ColorPair, glyph: u8) -> &mut Self {
        self.batch
            .push(DrawCommand::FillRegion { pos, color, glyph });
        self
    }

    /// Draw a horizontal progress bar
    pub fn bar_horizontal(
        &mut self,
        pos: Point,
        width: i32,
        n: i32,
        max: i32,
        color: ColorPair,
    ) -> &mut Self {
        self.batch.push(DrawCommand::BarHorizontal {
            pos,
            width,
            n,
            max,
            color,
        });
        self
    }

    /// Draw a horizontal progress bar
    pub fn bar_vertical(
        &mut self,
        pos: Point,
        height: i32,
        n: i32,
        max: i32,
        color: ColorPair,
    ) -> &mut Self {
        self.batch.push(DrawCommand::BarVertical {
            pos,
            height,
            n,
            max,
            color,
        });
        self
    }
}

/// Submits the current batch to the RLTK buffer and empties it
pub fn render_draw_buffer(rltk: &mut Rltk) {
    let mut buffer = COMMAND_BUFFER.lock().unwrap();
    buffer.iter().for_each(|cmd| match cmd {
        DrawCommand::ClearScreen => rltk.cls(),
        DrawCommand::ClearToColor { color } => rltk.cls_bg(*color),
        DrawCommand::SetTarget { console } => rltk.set_active_console(*console),
        DrawCommand::Set { pos, color, glyph } => {
            rltk.set(pos.x, pos.y, color.fg, color.bg, *glyph)
        }
        DrawCommand::SetBackground { pos, bg } => rltk.set_bg(pos.x, pos.y, *bg),
        DrawCommand::Print { pos, text } => rltk.print(pos.x, pos.y, &text),
        DrawCommand::PrintColor { pos, text, color } => {
            rltk.print_color(pos.x, pos.y, color.fg, color.bg, &text)
        }
        DrawCommand::PrintCentered { y, text } => rltk.print_centered(*y, &text),
        DrawCommand::PrintColorCentered { y, text, color } => {
            rltk.print_color_centered(*y, color.fg, color.bg, &text)
        }
        DrawCommand::Box { pos, color } => rltk.draw_box(
            pos.x1,
            pos.y1,
            pos.width(),
            pos.height(),
            color.fg,
            color.bg,
        ),
        DrawCommand::HollowBox { pos, color } => rltk.draw_hollow_box(
            pos.x1,
            pos.y1,
            pos.width(),
            pos.height(),
            color.fg,
            color.bg,
        ),
        DrawCommand::DoubleBox { pos, color } => rltk.draw_box_double(
            pos.x1,
            pos.y1,
            pos.width(),
            pos.height(),
            color.fg,
            color.bg,
        ),
        DrawCommand::HollowDoubleBox { pos, color } => rltk.draw_hollow_box_double(
            pos.x1,
            pos.y1,
            pos.width(),
            pos.height(),
            color.fg,
            color.bg,
        ),
        DrawCommand::FillRegion { pos, color, glyph } => {
            rltk.fill_region(*pos, *glyph, color.fg, color.bg)
        }
        DrawCommand::BarHorizontal {
            pos,
            width,
            n,
            max,
            color,
        } => rltk.draw_bar_horizontal(pos.x, pos.y, *width, *n, *max, color.fg, color.bg),
        DrawCommand::BarVertical {
            pos,
            height,
            n,
            max,
            color,
        } => rltk.draw_bar_vertical(pos.x, pos.y, *height, *n, *max, color.fg, color.bg),
    });
    buffer.clear();
}
