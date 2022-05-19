#[derive(Clone)]
pub(crate) struct TerminalBuilderFont {
    pub(crate) filename: String,
    pub(crate) chars_per_row: u16,
    pub(crate) n_rows: u16,
    pub(crate) font_height_pixels: (f32, f32),
}

impl TerminalBuilderFont {
    pub(crate) fn new<S: ToString>(image_filename: S, chars_per_row: u16, n_rows: u16, font_height_pixels: (f32, f32)) -> Self {
        Self {
            filename: image_filename.to_string(),
            chars_per_row,
            n_rows,
            font_height_pixels
        }
    }
}