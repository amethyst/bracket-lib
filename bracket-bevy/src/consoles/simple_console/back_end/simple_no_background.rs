use super::SimpleConsoleBackend;
use crate::consoles::{scaler::FontScaler, BracketMesh, ScreenScaler, SimpleConsole};
use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};

pub(crate) struct SimpleBackendNoBackground {
    pub(crate) mesh_handle: Option<Handle<Mesh>>,
    pub(crate) font_height_pixels: (f32, f32),
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) scaler: FontScaler,
}

impl SimpleBackendNoBackground {
    pub(crate) fn new(
        parent: &SimpleConsole,
        meshes: &mut Assets<Mesh>,
        chars_per_row: u16,
        n_rows: u16,
        font_height_pixels: (f32, f32),
        width: i32,
        height: i32,
    ) -> Self {
        let mut back_end = Self {
            mesh_handle: None,
            font_height_pixels,
            width,
            height,
            scaler: FontScaler::new(chars_per_row, n_rows, font_height_pixels),
        };
        let mesh = back_end.build_mesh(parent, &ScreenScaler::default());
        let mesh_handle = meshes.add(mesh);
        back_end.mesh_handle = Some(mesh_handle);
        back_end
    }

    pub fn build_mesh(&self, parent: &SimpleConsole, screen_scaler: &ScreenScaler) -> Mesh {
        let capacity = (self.width * self.height) as usize;
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(capacity * 4);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(capacity * 4);
        let mut uv: Vec<[f32; 2]> = Vec::with_capacity(capacity * 4);
        let mut colors: Vec<[f32; 4]> = Vec::with_capacity(capacity * 4);
        let mut indices: Vec<u32> = Vec::with_capacity(capacity * 6);
        let mut index_count = 0;
        let scale = screen_scaler.calc_step(self.width, self.height);
        let top_left = screen_scaler.top_left();

        // Build the foreground
        for y in 0..self.height {
            let screen_y = top_left.1 + (y as f32 * scale.1);
            let mut idx = ((self.height - 1 - y) * self.width) as usize;
            for x in 0..self.width {
                let screen_x = top_left.0 + (x as f32 * scale.0);
                vertices.push([screen_x, screen_y, 0.5]);
                vertices.push([screen_x + scale.0, screen_y, 0.5]);
                vertices.push([screen_x, screen_y + scale.1, 0.5]);
                vertices.push([screen_x + scale.0, screen_y + scale.1, 0.5]);
                for _ in 0..4 {
                    normals.push([0.0, 1.0, 0.0]);
                }
                let tex = self.scaler.texture_coords(parent.terminal[idx].glyph);
                uv.push([tex[0], tex[3]]);
                uv.push([tex[2], tex[3]]);
                uv.push([tex[0], tex[1]]);
                uv.push([tex[2], tex[1]]);

                colors.push(parent.terminal[idx].foreground);
                colors.push(parent.terminal[idx].foreground);
                colors.push(parent.terminal[idx].foreground);
                colors.push(parent.terminal[idx].foreground);

                indices.push(index_count);
                indices.push(index_count + 1);
                indices.push(index_count + 2);

                indices.push(index_count + 3);
                indices.push(index_count + 2);
                indices.push(index_count + 1);

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
}

impl SimpleConsoleBackend for SimpleBackendNoBackground {
    fn new_mesh(
        &self,
        front_end: &SimpleConsole,
        meshes: &mut Assets<Mesh>,
        scaler: &ScreenScaler,
    ) -> Handle<Mesh> {
        meshes.add(self.build_mesh(front_end, scaler))
    }

    fn spawn(&self, commands: &mut Commands, material: Handle<ColorMaterial>, idx: usize) {
        if let Some(mesh_handle) = &self.mesh_handle {
            commands
                .spawn(MaterialMesh2dBundle {
                    mesh: mesh_handle.clone().into(),
                    transform: Transform::default(),
                    material,
                    ..default()
                })
                .insert(BracketMesh(idx));
        }
    }

    fn get_pixel_size(&self) -> (f32, f32) {
        self.font_height_pixels
    }

    fn resize(&mut self, available_size: &(f32, f32)) -> (i32, i32) {
        self.width = (available_size.0 / self.font_height_pixels.0).floor() as i32;
        self.height = (available_size.1 / self.font_height_pixels.1).floor() as i32;

        (self.width, self.height)
    }
}
