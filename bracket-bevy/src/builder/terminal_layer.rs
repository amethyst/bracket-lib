use std::collections::HashSet;

#[derive(Debug, Clone)]
pub enum TerminalLayer {
    Simple {
        font_index: usize,
        width: usize,
        height: usize,
        features: HashSet<SimpleConsoleFeatures>,
    },
    Sparse {
        font_index: usize,
        width: usize,
        height: usize,
        features: HashSet<SparseConsoleFeatures>,
    },
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SimpleConsoleFeatures {
    WithoutBackground,
    NoDirtyOptimization,
}

#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
pub enum SparseConsoleFeatures {
    WithoutBackground,
}
