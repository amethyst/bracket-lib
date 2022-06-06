use bevy::{prelude::*, render::mesh::{PrimitiveTopology, Indices}, sprite::MaterialMesh2dBundle};
use super::{SimpleConsole, SimpleConsoleMarker};

pub(crate) trait SimpleConsoleBackend: Sync + Send {
    fn update_mesh(&self, front_end: &SimpleConsole, meshes: &mut Assets<Mesh>);
    fn spawn(&self, commands: &mut Commands, material: Handle<ColorMaterial>, idx:usize);
}

pub(crate) struct SimpleBackend {
    pub(crate) mesh_handle: Option<Handle<Mesh>>,
    pub(crate) chars_per_row: u16,
    pub(crate) n_rows: u16,
    pub(crate) font_height_pixels: (f32, f32),
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl SimpleBackend {
    pub(crate) fn new(chars_per_row: u16, n_rows: u16, font_height_pixels: (f32, f32), width: usize, height: usize) -> Self {
        Self { 
            mesh_handle: None,
            chars_per_row,
            n_rows,
            font_height_pixels,
            width,
            height,
        }
    }

    fn texture_coords(&self, glyph: u16) -> [f32;4] {
        let base_x = glyph % self.chars_per_row;
        let base_y = glyph / self.n_rows;
        let scale_x = 1.0 / self.chars_per_row as f32;
        let scale_y = 1.0 / self.n_rows as f32;
        return [
            base_x as f32 * scale_x,
            base_y as f32 * scale_y,
            (base_x+1) as f32 * scale_x,
            (base_y+1) as f32 * scale_y,
        ];
    }

    pub fn build_mesh(&self, parent: &SimpleConsole) -> Mesh {
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(self.width * self.height * 4);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(self.width * self.height * 4);
        let mut uv: Vec<[f32; 2]> = Vec::with_capacity(self.width * self.height * 4);
        let mut colors: Vec<[f32; 4]> = Vec::with_capacity(self.width * self.height * 4);
        let mut indices: Vec<u32> = Vec::with_capacity(self.width * self.height * 6);
        let mut index_count = 0;
        let half_height = self.height as f32 / 2.0;
        let half_width = self.width as f32 / 2.0;
        for y in 0..self.height {
            let screen_y = (y as f32 - half_height) * self.font_height_pixels.1;
            let mut idx = (self.height-1 -y) * self.width;
            for x in 0..self.width {
                let screen_x = (x as f32 - half_width) * self.font_height_pixels.0;
                vertices.push([ screen_x, screen_y, 0.0 ]);
                vertices.push([ screen_x + self.font_height_pixels.0, screen_y, 0.0 ]);
                vertices.push([ screen_x, screen_y + self.font_height_pixels.1, 0.0 ]);
                vertices.push([ screen_x + self.font_height_pixels.0, screen_y + self.font_height_pixels.1, 0.0 ]);
                for _ in 0..4 {
                    normals.push([0.0, 1.0, 0.0]);
                }
                let tex = self.texture_coords(parent.terminal[idx].glyph);
                uv.push([tex[0], tex[3]]);
                uv.push([tex[2], tex[3]]);
                uv.push([tex[0], tex[1]]);
                uv.push([tex[2], tex[1]]);

                colors.push(parent.terminal[idx].foreground);
                colors.push(parent.terminal[idx].foreground);
                colors.push(parent.terminal[idx].foreground);
                colors.push(parent.terminal[idx].foreground);

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
        mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }

    pub fn build_uvs(&self, parent: &SimpleConsole) -> Vec<[f32; 2]> {
        let mut uv: Vec<[f32; 2]> = Vec::with_capacity(self.width * self.height * 4);
        for y in 0..self.height {
            let mut idx = y * self.width;
            for _ in 0..self.width {
                let tex = self.texture_coords(parent.terminal[idx].glyph);
                uv.push([tex[0], tex[3]]);
                uv.push([tex[2], tex[3]]);
                uv.push([tex[0], tex[1]]);
                uv.push([tex[2], tex[1]]);
                idx += 1;
            }
        }
        uv
    }

    pub fn build_colors(&self, parent: &SimpleConsole) -> Vec<[f32; 4]> {
        let mut colors: Vec<[f32; 4]> = Vec::with_capacity(self.width * self.height * 4);
        for y in 0..self.height {
            let mut idx = y * self.width;
            for _ in 0..self.width {
                colors.push(parent.terminal[idx].foreground);
                colors.push(parent.terminal[idx].foreground);
                colors.push(parent.terminal[idx].foreground);
                colors.push(parent.terminal[idx].foreground);
                idx += 1;
            }
        }
        colors
    }
}

impl SimpleConsoleBackend for SimpleBackend {
    fn update_mesh(&self, front_end: &SimpleConsole, meshes: &mut Assets<Mesh>) {
        if let Some(mesh_handle) = &self.mesh_handle {
            if let Some(mesh) = meshes.get_mut(mesh_handle.clone()) {
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, self.build_uvs(
                    front_end,
                ));
                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, self.build_colors(front_end));
            }
        }
    }

    fn spawn(&self, commands: &mut Commands, material: Handle<ColorMaterial>, idx: usize) {
        if let Some(mesh_handle) = &self.mesh_handle {
            commands.spawn_bundle(MaterialMesh2dBundle {
                mesh: mesh_handle.clone().into(),
                transform: Transform::default(),
                material,
                ..default()
            })
            .insert(SimpleConsoleMarker(idx));
        }
    }
}