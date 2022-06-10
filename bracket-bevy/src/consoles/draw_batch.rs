use super::TextAlign;
use bracket_color::prelude::{ColorPair, RGBA};
use bracket_geometry::prelude::{Point, Rect};

pub struct DrawBatch {
    pub(crate) batch: Vec<(u32, DrawCommand)>,
    pub(crate) z_count: u32,
    pub(crate) needs_sort: bool,
}

impl DrawBatch {
    pub(crate) fn new() -> Self {
        Self {
            batch: Vec::new(),
            z_count: 0,
            needs_sort: false,
        }
    }

    fn next_z(&mut self) -> u32 {
        let result = self.z_count;
        self.z_count += 1;
        result
    }

    /// Adds a CLS (clear screen) to the drawing batch
    pub fn cls(&mut self) -> &mut Self {
        let z = self.next_z();
        self.batch.push((z, DrawCommand::ClearScreen));
        self
    }

    /// Adds a CLS (clear screen) to the drawing batch
    pub fn cls_color<COLOR>(&mut self, color: COLOR) -> &mut Self
    where
        COLOR: Into<RGBA>,
    {
        let z = self.next_z();
        self.batch.push((
            z,
            DrawCommand::ClearToColor {
                color: color.into(),
            },
        ));
        self
    }

    /// Sets the target console for rendering
    pub fn target(&mut self, console: usize) -> &mut Self {
        let z = self.next_z();
        self.batch.push((z, DrawCommand::SetTarget { console }));
        self
    }

    /// Sets an individual cell glyph
    pub fn set<G: TryInto<u16>>(&mut self, pos: Point, color: ColorPair, glyph: G) -> &mut Self {
        let z = self.next_z();
        self.batch.push((
            z,
            DrawCommand::Set {
                pos,
                color,
                glyph: glyph.try_into().ok().expect("Must be u16 convertible"),
            },
        ));
        self
    }

    /// Sets an individual cell glyph with a specified ordering within the batch
    pub fn set_with_z<G: TryInto<u16>>(
        &mut self,
        pos: Point,
        color: ColorPair,
        glyph: G,
        z: u32,
    ) -> &mut Self {
        self.batch.push((
            z,
            DrawCommand::Set {
                pos,
                color,
                glyph: glyph.try_into().ok().expect("Must be u16 convertible"),
            },
        ));
        self.needs_sort = true;
        self
    }

    /// Sets an individual cell glyph
    pub fn set_bg<COLOR>(&mut self, pos: Point, bg: COLOR) -> &mut Self
    where
        COLOR: Into<RGBA>,
    {
        let z = self.next_z();
        self.batch
            .push((z, DrawCommand::SetBackground { pos, bg: bg.into() }));
        self
    }

    /// Sets an individual cell glyph with specified render order
    pub fn set_bg_with_z<COLOR>(&mut self, pos: Point, bg: COLOR, z: u32) -> &mut Self
    where
        COLOR: Into<RGBA>,
    {
        self.batch
            .push((z, DrawCommand::SetBackground { pos, bg: bg.into() }));
        self.needs_sort = true;
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
        let z = self.next_z();
        self.batch.push((
            z,
            DrawCommand::Printer {
                pos,
                text: text.to_string(),
                align,
                background,
            },
        ));
        self
    }

    /// Prints formatted text, using the doryen_rs convention. For example:
    /// "#[blue]This blue text contains a #[pink]pink#[] word"
    /// You can specify the render order.
    pub fn printer_with_z<S: ToString>(
        &mut self,
        pos: Point,
        text: S,
        align: TextAlign,
        background: Option<RGBA>,
        z: u32,
    ) -> &mut Self {
        self.batch.push((
            z,
            DrawCommand::Printer {
                pos,
                text: text.to_string(),
                align,
                background,
            },
        ));
        self.needs_sort = true;
        self
    }

    /// Prints text in the default colors at a given location
    pub fn print<S: ToString>(&mut self, pos: Point, text: S) -> &mut Self {
        let z = self.next_z();
        self.batch.push((
            z,
            DrawCommand::Print {
                pos,
                text: text.to_string(),
            },
        ));
        self
    }

    /// Prints text in the default colors at a given location & render order
    pub fn print_with_z<S: ToString>(&mut self, pos: Point, text: S, z: u32) -> &mut Self {
        self.batch.push((
            z,
            DrawCommand::Print {
                pos,
                text: text.to_string(),
            },
        ));
        self.needs_sort = true;
        self
    }

    /// Prints text in the default colors at a given location
    pub fn print_color<S: ToString>(&mut self, pos: Point, text: S, color: ColorPair) -> &mut Self {
        let z = self.next_z();
        self.batch.push((
            z,
            DrawCommand::PrintColor {
                pos,
                text: text.to_string(),
                color,
            },
        ));
        self
    }

    /// Prints text in the default colors at a given location & render order
    pub fn print_color_with_z<S: ToString>(
        &mut self,
        pos: Point,
        text: S,
        color: ColorPair,
        z: u32,
    ) -> &mut Self {
        self.batch.push((
            z,
            DrawCommand::PrintColor {
                pos,
                text: text.to_string(),
                color,
            },
        ));
        self.needs_sort = true;
        self
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    pub fn print_centered<S: ToString, Y: TryInto<i32>>(&mut self, y: Y, text: S) -> &mut Self {
        let z = self.next_z();
        self.batch.push((
            z,
            DrawCommand::PrintCentered {
                y: y.try_into().ok().expect("Must be i32 convertible"),
                text: text.to_string(),
            },
        ));
        self
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    pub fn print_centered_with_z<S: ToString, Y: TryInto<i32>>(
        &mut self,
        y: Y,
        text: S,
        z: u32,
    ) -> &mut Self {
        self.batch.push((
            z,
            DrawCommand::PrintCentered {
                y: y.try_into().ok().expect("Must be i32 convertible"),
                text: text.to_string(),
            },
        ));
        self.needs_sort = true;
        self
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    pub fn print_color_centered<S: ToString, Y: TryInto<i32>>(
        &mut self,
        y: Y,
        text: S,
        color: ColorPair,
    ) -> &mut Self {
        let z = self.next_z();
        self.batch.push((
            z,
            DrawCommand::PrintColorCentered {
                y: y.try_into().ok().expect("Must be i32 convertible"),
                text: text.to_string(),
                color,
            },
        ));
        self
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    pub fn print_color_centered_with_z<S: ToString, Y: TryInto<i32>>(
        &mut self,
        y: Y,
        text: S,
        color: ColorPair,
        z: u32,
    ) -> &mut Self {
        self.batch.push((
            z,
            DrawCommand::PrintColorCentered {
                y: y.try_into().ok().expect("Must be i32 convertible"),
                text: text.to_string(),
                color,
            },
        ));
        self.needs_sort = true;
        self
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    pub fn print_centered_at<S: ToString>(&mut self, pos: Point, text: S) -> &mut Self {
        let z = self.next_z();
        self.batch.push((
            z,
            DrawCommand::PrintCenteredAt {
                pos,
                text: text.to_string(),
            },
        ));
        self
    }

    /// Prints text, centered to the whole console width, at vertical location y with render order.
    pub fn print_centered_at_with_z<S: ToString>(
        &mut self,
        pos: Point,
        text: S,
        z: u32,
    ) -> &mut Self {
        self.batch.push((
            z,
            DrawCommand::PrintCenteredAt {
                pos,
                text: text.to_string(),
            },
        ));
        self.needs_sort = true;
        self
    }

    /// Prints text, centered to the whole console width, at vertical location y.
    pub fn print_color_centered_at<S: ToString>(
        &mut self,
        pos: Point,
        text: S,
        color: ColorPair,
    ) -> &mut Self {
        let z = self.next_z();
        self.batch.push((
            z,
            DrawCommand::PrintColorCenteredAt {
                pos,
                text: text.to_string(),
                color,
            },
        ));
        self
    }

    /// Prints text, centered to the whole console width, at vertical location y with render order.
    pub fn print_color_centered_at_with_z<S: ToString>(
        &mut self,
        pos: Point,
        text: S,
        color: ColorPair,
        z: u32,
    ) -> &mut Self {
        self.batch.push((
            z,
            DrawCommand::PrintColorCenteredAt {
                pos,
                text: text.to_string(),
                color,
            },
        ));
        self.needs_sort = true;
        self
    }

    /// Prints right aligned text
    pub fn print_right<S: ToString>(&mut self, pos: Point, text: S) -> &mut Self {
        let z = self.next_z();
        self.batch.push((
            z,
            DrawCommand::PrintRight {
                pos,
                text: text.to_string(),
            },
        ));
        self
    }

    /// Prints right aligned text with render order
    pub fn print_right_z<S: ToString>(&mut self, pos: Point, text: S, z: u32) -> &mut Self {
        self.batch.push((
            z,
            DrawCommand::PrintRight {
                pos,
                text: text.to_string(),
            },
        ));
        self.needs_sort = true;
        self
    }

    /// Prints right aligned text
    pub fn print_color_right<S: ToString>(
        &mut self,
        pos: Point,
        text: S,
        color: ColorPair,
    ) -> &mut Self {
        let z = self.next_z();
        self.batch.push((
            z,
            DrawCommand::PrintColorRight {
                pos,
                text: text.to_string(),
                color,
            },
        ));
        self
    }

    /// Prints right aligned text with render order
    pub fn print_color_right_with_z<S: ToString>(
        &mut self,
        pos: Point,
        text: S,
        color: ColorPair,
        z: u32,
    ) -> &mut Self {
        self.batch.push((
            z,
            DrawCommand::PrintColorRight {
                pos,
                text: text.to_string(),
                color,
            },
        ));
        self.needs_sort = true;
        self
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_box(&mut self, pos: Rect, color: ColorPair) -> &mut Self {
        let z = self.next_z();
        self.batch.push((z, DrawCommand::Box { pos, color }));
        self
    }

    /// Draws a box, starting at x/y with the extents width/height using CP437 line characters. With render order.
    pub fn draw_box_with_z(&mut self, pos: Rect, color: ColorPair, z: u32) -> &mut Self {
        self.batch.push((z, DrawCommand::Box { pos, color }));
        self.needs_sort = true;
        self
    }

    /// Draws a non-filled (hollow) box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_hollow_box(&mut self, pos: Rect, color: ColorPair) -> &mut Self {
        let z = self.next_z();
        self.batch.push((z, DrawCommand::HollowBox { pos, color }));
        self
    }

    /// Draws a non-filled (hollow) box, starting at x/y with the extents width/height using CP437 line characters. With render order.
    pub fn draw_hollow_box_with_z(&mut self, pos: Rect, color: ColorPair, z: u32) -> &mut Self {
        self.batch.push((z, DrawCommand::HollowBox { pos, color }));
        self.needs_sort = true;
        self
    }

    /// Draws a double-lined box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_double_box(&mut self, pos: Rect, color: ColorPair) -> &mut Self {
        let z = self.next_z();
        self.batch.push((z, DrawCommand::DoubleBox { pos, color }));
        self
    }

    /// Draws a double-lined box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_double_box_with_z(&mut self, pos: Rect, color: ColorPair, z: u32) -> &mut Self {
        self.batch.push((z, DrawCommand::DoubleBox { pos, color }));
        self.needs_sort = true;
        self
    }

    /// Draws a non-filled (hollow) double-lined box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_hollow_double_box(&mut self, pos: Rect, color: ColorPair) -> &mut Self {
        let z = self.next_z();
        self.batch
            .push((z, DrawCommand::HollowDoubleBox { pos, color }));
        self
    }

    /// Draws a non-filled (hollow) double-lined box, starting at x/y with the extents width/height using CP437 line characters
    pub fn draw_hollow_double_box_with_z(
        &mut self,
        pos: Rect,
        color: ColorPair,
        z: u32,
    ) -> &mut Self {
        self.batch
            .push((z, DrawCommand::HollowDoubleBox { pos, color }));
        self.needs_sort = true;
        self
    }

    /// Fills a region with a glyph/color combination.
    pub fn fill_region<G: TryInto<u16>>(
        &mut self,
        pos: Rect,
        color: ColorPair,
        glyph: G,
    ) -> &mut Self {
        let z = self.next_z();
        self.batch.push((
            z,
            DrawCommand::FillRegion {
                pos,
                color,
                glyph: glyph.try_into().ok().expect("Must be u16 convertible"),
            },
        ));
        self
    }

    /// Fills a region with a glyph/color combination.
    pub fn fill_region_with_z<G: TryInto<u16>>(
        &mut self,
        pos: Rect,
        color: ColorPair,
        glyph: G,
        z: u32,
    ) -> &mut Self {
        self.batch.push((
            z,
            DrawCommand::FillRegion {
                pos,
                color,
                glyph: glyph.try_into().ok().expect("Must be u16 convertible"),
            },
        ));
        self.needs_sort = true;
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
        let z = self.next_z();
        self.batch.push((
            z,
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
    pub fn bar_horizontal_with_z<W, N, MAX>(
        &mut self,
        pos: Point,
        width: W,
        n: N,
        max: MAX,
        color: ColorPair,
        z: u32,
    ) -> &mut Self
    where
        W: TryInto<i32>,
        N: TryInto<i32>,
        MAX: TryInto<i32>,
    {
        self.batch.push((
            z,
            DrawCommand::BarHorizontal {
                pos,
                width: width.try_into().ok().expect("Must be i32 convertible"),
                n: n.try_into().ok().expect("Must be i32 convertible"),
                max: max.try_into().ok().expect("Must be i32 convertible"),
                color,
            },
        ));
        self.needs_sort = true;
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
        let z = self.next_z();
        self.batch.push((
            z,
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

    /// Draw a horizontal progress bar
    pub fn bar_vertical_with_z<H, N, MAX>(
        &mut self,
        pos: Point,
        height: H,
        n: N,
        max: MAX,
        color: ColorPair,
        z: u32,
    ) -> &mut Self
    where
        H: TryInto<i32>,
        N: TryInto<i32>,
        MAX: TryInto<i32>,
    {
        self.batch.push((
            z,
            DrawCommand::BarVertical {
                pos,
                height: height.try_into().ok().expect("Must be i32 convertible"),
                n: n.try_into().ok().expect("Must be i32 convertible"),
                max: max.try_into().ok().expect("Must be i32 convertible"),
                color,
            },
        ));
        self.needs_sort = true;
        self
    }

    /// Sets a clipping rectangle for the current console
    pub fn set_clipping(&mut self, clip: Option<Rect>) -> &mut Self {
        let z = self.next_z();
        self.batch.push((z, DrawCommand::SetClipping { clip }));
        self
    }

    /// Apply an alpha channel value to all cells' foregrounds in the current terminal.
    pub fn set_all_fg_alpha(&mut self, alpha: f32) -> &mut Self {
        let z = self.next_z();
        self.batch.push((z, DrawCommand::SetFgAlpha { alpha }));
        self
    }

    /// Apply an alpha channel value to all cells' backgrounds in the current terminal.
    pub fn set_all_bg_alpha(&mut self, alpha: f32) -> &mut Self {
        let z = self.next_z();
        self.batch.push((z, DrawCommand::SetBgAlpha { alpha }));
        self
    }

    /// Apply fg/bg alpha channel values to all cells in the current terminal.
    pub fn set_all_alpha(&mut self, fg: f32, bg: f32) -> &mut Self {
        let z = self.next_z();
        self.batch.push((z, DrawCommand::SetAllAlpha { fg, bg }));
        self
    }
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
        glyph: u16,
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
        glyph: u16,
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
}
