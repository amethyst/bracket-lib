use bevy::prelude::{Plugin, CoreStage, SystemStage};
use crate::{TerminalBuilderFont, TerminalLayer, load_terminals, update_consoles};

#[derive(Clone)]
pub struct BTermBuilder {
    pub(crate) fonts: Vec<TerminalBuilderFont>,
    pub(crate) layers: Vec<TerminalLayer>,
    pub(crate) with_ortho_camera: bool,
}

impl BTermBuilder {
    pub fn simple_80x50() -> Self {
        Self {
            fonts: vec![TerminalBuilderFont::new("terminal8x8.png", 16, 16, (8.0, 8.0))],
            layers: vec![TerminalLayer::Simple{ font_index: 0, width: 80, height: 50 }],
            with_ortho_camera: true,
        }
    }
}

impl Plugin for BTermBuilder {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(self.clone());
        app.add_startup_system(load_terminals);
        app.add_stage_after(CoreStage::Update, "bracket_term_update", SystemStage::single_threaded());
        app.add_system(update_consoles);
    }
}