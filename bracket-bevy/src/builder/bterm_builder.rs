use crate::{load_terminals, update_consoles, TerminalBuilderFont, TerminalLayer};
use bevy::prelude::{CoreStage, Plugin, SystemStage};
use std::collections::HashSet;

#[derive(Clone)]
pub struct BTermBuilder {
    pub(crate) fonts: Vec<TerminalBuilderFont>,
    pub(crate) layers: Vec<TerminalLayer>,
    pub(crate) with_ortho_camera: bool,
}

impl BTermBuilder {
    pub fn empty() -> Self {
        Self {
            fonts: Vec::new(),
            layers: Vec::new(),
            with_ortho_camera: true,
        }
    }

    pub fn simple_80x50() -> Self {
        Self {
            fonts: vec![TerminalBuilderFont::new(
                "terminal8x8.png",
                16,
                16,
                (8.0, 8.0),
            )],
            layers: vec![TerminalLayer::Simple {
                font_index: 0,
                width: 80,
                height: 50,
                features: HashSet::new(),
            }],
            with_ortho_camera: true,
        }
    }

    pub fn with_ortho_camera(mut self, with_ortho_camera: bool) -> Self {
        self.with_ortho_camera = with_ortho_camera;
        self
    }

    pub fn with_background(mut self, with_background: bool) -> Self {
        if !self.layers.is_empty() {
            let last_index = self.layers.len() - 1;
            match &mut self.layers[last_index] {
                TerminalLayer::Simple { features, .. } => {
                    if with_background {
                        features.remove(&crate::SimpleConsoleFeatures::WithoutBackground);
                    } else {
                        features.insert(crate::SimpleConsoleFeatures::WithoutBackground);
                    }
                }
                _ => {}
            }
        }
        self
    }

    pub fn with_dirty_optimization(mut self, with_dirty_optimization: bool) -> Self {
        if !self.layers.is_empty() {
            let last_index = self.layers.len() - 1;
            match &mut self.layers[last_index] {
                TerminalLayer::Simple { features, .. } => {
                    if !with_dirty_optimization {
                        features.remove(&crate::SimpleConsoleFeatures::NoDirtyOptimization);
                    } else {
                        features.insert(crate::SimpleConsoleFeatures::NoDirtyOptimization);
                    }
                }
                _ => {}
            }
        }
        self
    }

    pub fn with_font(
        mut self,
        filename: &str,
        glyphs_per_row: u16,
        rows: u16,
        pixel_size: (f32, f32),
    ) -> Self {
        self.fonts.push(TerminalBuilderFont::new(
            filename,
            glyphs_per_row,
            rows,
            pixel_size,
        ));
        self
    }

    pub fn with_simple_console(mut self, font_index: usize, width: usize, height: usize) -> Self {
        self.layers.push(TerminalLayer::Simple {
            font_index,
            width,
            height,
            features: HashSet::new(),
        });
        self
    }

    pub fn with_sparse_console(mut self, font_index: usize, width: usize, height: usize) -> Self {
        self.layers.push(TerminalLayer::Sparse {
            font_index,
            width,
            height,
        });
        self
    }
}

impl Plugin for BTermBuilder {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(self.clone());
        app.add_startup_system(load_terminals);
        app.add_stage_after(
            CoreStage::Update,
            "bracket_term_update",
            SystemStage::single_threaded(),
        );
        app.add_system(update_consoles);
    }
}
