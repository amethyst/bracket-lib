use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum TerminalLayer {
    Simple {
        font_index: usize,
        width: i32,
        height: i32,
        features: HashSet<SimpleConsoleFeatures>,
    },
    Sparse {
        font_index: usize,
        width: i32,
        height: i32,
        features: HashSet<SparseConsoleFeatures>,
    },
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SimpleConsoleFeatures {
    WithoutBackground,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SparseConsoleFeatures {
    WithoutBackground,
}
