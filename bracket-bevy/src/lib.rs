// Provides Bracket-Lib style CP437/ASCII terminal options to Bevy
mod cp437;
use cp437::*;
use bevy::{prelude::*, render::{mesh::{PrimitiveTopology, Indices}}, sprite::MaterialMesh2dBundle};

#[derive(Clone)]
pub struct TerminalFont {
    filename: String,
    chars_per_row: u16,
    n_rows: u16,
}

impl TerminalFont {
    pub fn new<S: ToString>(image_filename: S, chars_per_row: u16, n_rows: u16) -> Self {
        Self {
            filename: image_filename.to_string(),
            chars_per_row,
            n_rows,
        }
    }
}

#[derive(Clone)]
pub enum TerminalLayer {
    Simple { font_index: usize, width: usize, height: usize },
}

#[derive(Clone)]
pub struct BTermBuilder {
    fonts: Vec<TerminalFont>,
    layers: Vec<TerminalLayer>,
}

impl BTermBuilder {
    pub fn simple_80x50() -> Self {
        Self {
            fonts: vec![TerminalFont::new("terminal8x8.png", 16, 16)],
            layers: vec![TerminalLayer::Simple{ font_index: 0, width: 80, height: 50 }]
        }
    }
}

impl Plugin for BTermBuilder {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.insert_resource(self.clone());
        app.add_startup_system(load_terminals);
        app.add_system(update_consoles);
    }
}

struct FontStore {
    texture_handle: Handle<Image>,
    material_handle: Handle<ColorMaterial>,
    chars_per_row: u16,
    n_rows: u16,
}

pub struct BracketContext {
    fonts: Vec<FontStore>,
    terminals: Vec<SimpleConsole>,
    current_layer: usize,
}

impl BracketContext {
    fn new() -> Self {
        Self { fonts: Vec::new(), terminals: Vec::new(), current_layer: 0 }
    }

    pub fn set_layer(&mut self, layer: usize) {
        self.current_layer = layer;
    }

    pub fn cls(&mut self) {
        self.terminals[self.current_layer].cls();
    }

    pub fn print<S: ToString>(&mut self, x: usize, y: usize, text: S) {
        self.terminals[self.current_layer].print(x, y, text);
    }
}

#[derive(Component)]
pub struct SimpleConsoleMarker(pub usize);

pub fn load_terminals(
    context: Res<BTermBuilder>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // TODO: Make this optional
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Setup the new context
    let mut new_context = BracketContext::new();

    // Load the fonts
    for font in context.fonts.iter() {
        let texture_handle = asset_server.load(&font.filename);
        let material_handle = materials.add(ColorMaterial::from(texture_handle.clone()));
        new_context.fonts.push(FontStore{
            texture_handle, material_handle,
            chars_per_row: font.chars_per_row,
            n_rows: font.n_rows,
        });
    }

    // Setup the consoles
    for (idx, terminal) in context.layers.iter().enumerate() {
        match terminal {
            TerminalLayer::Simple { font_index, width, height } => {
                let mut console = SimpleConsole::new(*font_index, *width, *height);
                let mesh = console.build_mesh(new_context.fonts[console.font_index].chars_per_row, new_context.fonts[console.font_index].n_rows);
                let mesh_handle = meshes.add(mesh);
                console.mesh_handle=Some(mesh_handle.clone());

                // Test code
                commands.spawn_bundle(MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    transform: Transform::default(),
                    material: new_context.fonts[*font_index].material_handle.clone(),
                    ..default()
                })
                .insert(SimpleConsoleMarker(idx));

                new_context.terminals.push(console);
            }
        }
    }

    // Clean up after the building process
    commands.remove_resource::<BTermBuilder>();
    commands.insert_resource(new_context);
}

pub fn update_consoles(
    ctx: ResMut<BracketContext>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for terminal in ctx.terminals.iter() {
        if let Some(mesh_handle) = &terminal.mesh_handle {
            if let Some(mesh) = meshes.get_mut(mesh_handle.clone()) {
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, terminal.build_uvs(
                    ctx.fonts[terminal.font_index].chars_per_row,
                    ctx.fonts[terminal.font_index].n_rows,
                ));
            }
        }
    }
}

pub struct SimpleConsole {
    font_index: usize,
    width: usize,
    height: usize,
    terminal: Vec<u16>,
    mesh_handle: Option<Handle<Mesh>>
}


const SIZE_TMP : f32 = 8.0;

impl SimpleConsole {
    pub fn new(font_index: usize, width: usize, height: usize) -> Self {
        Self {
            font_index, width, height, terminal: vec![65; width*height],
            mesh_handle: None,
        }
    }

    fn texture_coords(&self, glyph: u16, chars_per_row: u16, n_rows: u16) -> [f32;4] {
        let base_x = glyph % chars_per_row;
        let base_y = glyph / n_rows;
        let scale_x = 1.0 / chars_per_row as f32;
        let scale_y = 1.0 / n_rows as f32;
        return [
            base_x as f32 * scale_x,
            base_y as f32 * scale_y,
            (base_x+1) as f32 * scale_x,
            (base_y+1) as f32 * scale_y,
        ];
    }

    pub fn build_mesh(&self, chars_per_row: u16, n_rows: u16) -> Mesh {
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(self.width * self.height * 4);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(self.width * self.height * 4);
        let mut uv: Vec<[f32; 2]> = Vec::with_capacity(self.width * self.height * 4);
        let mut colors: Vec<[f32; 4]> = Vec::with_capacity(self.width * self.height * 4);
        let mut indices: Vec<u32> = Vec::with_capacity(self.width * self.height * 6);
        let mut index_count = 0;
        let half_height = self.height as f32 / 2.0;
        let half_width = self.width as f32 / 2.0;
        for y in 0..self.height {
            let screen_y = (y as f32 - half_height) * SIZE_TMP;
            let mut idx = (self.height-1 -y) * self.width;
            for x in 0..self.width {
                let screen_x = (x as f32 - half_width) * SIZE_TMP;
                vertices.push([ screen_x, screen_y, 0.0 ]);
                vertices.push([ screen_x + SIZE_TMP, screen_y, 0.0 ]);
                vertices.push([ screen_x, screen_y + SIZE_TMP, 0.0 ]);
                vertices.push([ screen_x + SIZE_TMP, screen_y + SIZE_TMP, 0.0 ]);
                for _ in 0..4 {
                    normals.push([0.0, 1.0, 0.0]);
                }
                let tex = self.texture_coords(self.terminal[idx], chars_per_row, n_rows);
                uv.push([tex[0], tex[3]]);
                uv.push([tex[2], tex[3]]);
                uv.push([tex[0], tex[1]]);
                uv.push([tex[2], tex[1]]);

                // Not convinced this does anything at all
                colors.push([1.0, 0.0, 0.0, 1.0]);
                colors.push([1.0, 0.0, 0.0, 1.0]);
                colors.push([1.0, 0.0, 0.0, 1.0]);
                colors.push([1.0, 0.0, 0.0, 1.0]);

                indices.push(index_count);
                indices.push(index_count+1);
                indices.push(index_count+2);

                indices.push(index_count+3);
                indices.push(index_count+2);
                indices.push(index_count+1);

                index_count += 4;
                idx += 1;
            }
        }
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv);
        //mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }

    fn build_uvs(&self, chars_per_row: u16, n_rows: u16) -> Vec<[f32; 2]> {
        let mut uv: Vec<[f32; 2]> = Vec::with_capacity(self.width * self.height * 4);
        for y in 0..self.height {
            let mut idx = y * self.width;
            for _ in 0..self.width {
                let tex = self.texture_coords(self.terminal[idx], chars_per_row, n_rows);
                uv.push([tex[0], tex[3]]);
                uv.push([tex[2], tex[3]]);
                uv.push([tex[0], tex[1]]);
                uv.push([tex[2], tex[1]]);
                idx += 1;
            }
        }
        uv
    }

    fn cls(&mut self) {
        self.terminal.iter_mut().for_each(|c| *c = 32);
    }

    pub fn print<S: ToString>(&mut self, mut x: usize, y: usize, text: S) {
        let bytes = string_to_cp437(&text.to_string());
        for glyph in bytes {
            let idx = self.at(x, y);
            self.terminal[idx] = glyph;
            x += 1;
        }
    }

    fn at(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y) * self.width) + x
    }
}