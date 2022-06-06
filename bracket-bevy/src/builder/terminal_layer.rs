#[derive(Clone)]
pub enum TerminalLayer {
    Simple {
        font_index: usize,
        width: usize,
        height: usize,
    },
}
