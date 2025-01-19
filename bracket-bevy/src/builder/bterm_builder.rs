use crate::{
    consoles::{
        apply_all_batches, default_gutter_size, replace_meshes, update_mouse_position,
        update_timing, window_resize, ScreenScaler,
    },
    fix_images, load_terminals, update_consoles, RandomNumbers, TerminalBuilderFont, TerminalLayer,
};
use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::{Plugin, PostUpdate, PreUpdate, Resource, Startup},
    utils::HashMap,
};
use bracket_color::prelude::RGBA;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalScalingMode {
    Stretch,
    ResizeTerminals,
}

#[derive(Clone, Resource)]
pub struct BTermBuilder {
    pub(crate) fonts: Vec<TerminalBuilderFont>,
    pub(crate) layers: Vec<TerminalLayer>,
    pub(crate) palette: HashMap<String, RGBA>,
    pub(crate) with_ortho_camera: bool,
    pub(crate) with_random_number_generator: bool,
    pub(crate) with_diagnostics: bool,
    pub(crate) log_diagnostics: bool,
    pub(crate) scaling_mode: TerminalScalingMode,
    pub(crate) gutter: f32,
    pub(crate) auto_apply_batches: bool,
}

impl BTermBuilder {
    pub fn empty() -> Self {
        Self {
            fonts: Vec::new(),
            layers: Vec::new(),
            palette: HashMap::new(),
            with_ortho_camera: true,
            with_random_number_generator: false,
            with_diagnostics: true,
            log_diagnostics: false,
            scaling_mode: TerminalScalingMode::Stretch,
            gutter: default_gutter_size(),
            auto_apply_batches: true,
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
            palette: HashMap::new(),
            with_ortho_camera: true,
            with_random_number_generator: false,
            with_diagnostics: true,
            log_diagnostics: false,
            scaling_mode: TerminalScalingMode::Stretch,
            gutter: default_gutter_size(),
            auto_apply_batches: true,
        }
    }

    pub fn with_scaling_mode(mut self, scaling_mode: TerminalScalingMode) -> Self {
        self.scaling_mode = scaling_mode;
        self
    }

    pub fn with_ortho_camera(mut self, with_ortho_camera: bool) -> Self {
        self.with_ortho_camera = with_ortho_camera;
        self
    }

    pub fn with_random_number_generator(mut self, with_random_number_generator: bool) -> Self {
        self.with_random_number_generator = with_random_number_generator;
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
                TerminalLayer::Sparse { features, .. } => {
                    if with_background {
                        features.remove(&crate::SparseConsoleFeatures::WithoutBackground);
                    } else {
                        features.insert(crate::SparseConsoleFeatures::WithoutBackground);
                    }
                }
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

    pub fn with_named_color<S: ToString, C: Into<RGBA>>(mut self, name: S, color: C) -> Self {
        self.palette.insert(name.to_string(), color.into());
        self
    }

    pub fn with_timing_diagnostics(mut self, with_diagnostics: bool) -> Self {
        self.with_diagnostics = with_diagnostics;
        self
    }

    pub fn with_timing_log(mut self, with_diagnostics: bool) -> Self {
        self.log_diagnostics = with_diagnostics;
        self
    }

    pub fn with_auto_apply_batches(mut self, with_auto_batches: bool) -> Self {
        self.auto_apply_batches = with_auto_batches;
        self
    }

    pub fn with_gutter(mut self, gutter: f32) -> Self {
        self.gutter = gutter;
        self
    }

    pub fn with_simple_console(mut self, font_index: usize, width: i32, height: i32) -> Self {
        self.layers.push(TerminalLayer::Simple {
            font_index,
            width,
            height,
            features: HashSet::new(),
        });
        self
    }

    pub fn with_sparse_console(mut self, font_index: usize, width: i32, height: i32) -> Self {
        self.layers.push(TerminalLayer::Sparse {
            font_index,
            width,
            height,
            features: HashSet::new(),
        });
        self
    }
}

impl Plugin for BTermBuilder {
    fn build(&self, app: &mut bevy::prelude::App) {
        if self.with_diagnostics {
            app.add_plugins(FrameTimeDiagnosticsPlugin);
        }
        if self.log_diagnostics {
            app.add_plugins(LogDiagnosticsPlugin::default());
        }
        app.insert_resource(self.clone());
        app.insert_resource(ScreenScaler::new(self.gutter));
        app.add_systems(Startup, load_terminals);
        if self.with_diagnostics {
            app.add_systems(PreUpdate, (update_timing, update_mouse_position));
        }
        if self.auto_apply_batches {
            app.add_systems(PostUpdate, apply_all_batches);
        }
        app.add_systems(
            PostUpdate,
            (update_consoles, replace_meshes, window_resize, fix_images),
        );
        if self.with_random_number_generator {
            app.insert_resource(RandomNumbers::new());
        }
    }
}
