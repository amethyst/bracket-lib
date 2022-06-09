use super::SparseConsoleBackend;
use crate::consoles::{BracketMesh, SparseConsole};
use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::MaterialMesh2dBundle,
};

pub(crate) struct SparseBackendWithBackground {
    pub(crate) mesh_handle: Option<Handle<Mesh>>,
    pub(crate) chars_per_row: u16,
    pub(crate) n_rows: u16,
    pub(crate) font_height_pixels: (f32, f32),
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl SparseBackendWithBackground {
    pub(crate) fn new(
        parent: &SparseConsole,
        meshes: &mut Assets<Mesh>,
        chars_per_row: u16,
        n_rows: u16,
        font_height_pixels: (f32, f32),
        width: usize,
        height: usize,
    ) -> Self {
        let mut back_end = Self {
            mesh_handle: None,
            chars_per_row,
            n_rows,
            font_height_pixels,
            width,
            height,
        };
        let mesh = back_end.build_mesh(parent);
        let mesh_handle = meshes.add(mesh);
        back_end.mesh_handle = Some(mesh_handle);
        back_end
    }

    fn texture_coords(&self, glyph: u16) -> [f32; 4] {
        let base_x = glyph % self.chars_per_row;
        let base_y = glyph / self.n_rows;
        let scale_x = 1.0 / self.chars_per_row as f32;
        let scale_y = 1.0 / self.n_rows as f32;
        [
            base_x as f32 * scale_x,
            base_y as f32 * scale_y,
            (base_x + 1) as f32 * scale_x,
            (base_y + 1) as f32 * scale_y,
        ]
    }

    pub fn build_mesh(&self, parent: &SparseConsole) -> Mesh {
        let n_elements = parent.terminal.len();
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(n_elements * 8);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(n_elements * 8);
        let mut uv: Vec<[f32; 2]> = Vec::with_capacity(n_elements * 8);
        let mut colors: Vec<[f32; 4]> = Vec::with_capacity(n_elements * 8);
        let mut indices: Vec<u32> = Vec::with_capacity(n_elements * 12);
        let mut index_count = 0;
        let half_height = self.height as f32 / 2.0;
        let half_width = self.width as f32 / 2.0;

        for (x, y, chr) in parent.terminal.iter() {
            let screen_x = (*x as f32 - half_width) * self.font_height_pixels.0;
            let actual_y = self.height - 1 - y;
            let screen_y = (actual_y as f32 - half_height) * self.font_height_pixels.1;

            // Background
            vertices.push([screen_x, screen_y, 0.0]);
            vertices.push([screen_x + self.font_height_pixels.0, screen_y, 0.0]);
            vertices.push([screen_x, screen_y + self.font_height_pixels.1, 0.0]);
            vertices.push([
                screen_x + self.font_height_pixels.0,
                screen_y + self.font_height_pixels.1,
                0.0,
            ]);
            for _ in 0..4 {
                normals.push([0.0, 1.0, 0.0]);
            }

            let tex = self.texture_coords(219);
            uv.push([tex[0], tex[3]]);
            uv.push([tex[2], tex[3]]);
            uv.push([tex[0], tex[1]]);
            uv.push([tex[2], tex[1]]);

            colors.push(chr.background);
            colors.push(chr.background);
            colors.push(chr.background);
            colors.push(chr.background);

            indices.push(index_count);
            indices.push(index_count + 1);
            indices.push(index_count + 2);

            indices.push(index_count + 3);
            indices.push(index_count + 2);
            indices.push(index_count + 1);

            index_count += 4;

            // Foreground
            vertices.push([screen_x, screen_y, 0.5]);
            vertices.push([screen_x + self.font_height_pixels.0, screen_y, 0.5]);
            vertices.push([screen_x, screen_y + self.font_height_pixels.1, 0.5]);
            vertices.push([
                screen_x + self.font_height_pixels.0,
                screen_y + self.font_height_pixels.1,
                0.5,
            ]);
            for _ in 0..4 {
                normals.push([0.0, 1.0, 0.0]);
            }

            let tex = self.texture_coords(chr.glyph);
            uv.push([tex[0], tex[3]]);
            uv.push([tex[2], tex[3]]);
            uv.push([tex[0], tex[1]]);
            uv.push([tex[2], tex[1]]);

            colors.push(chr.foreground);
            colors.push(chr.foreground);
            colors.push(chr.foreground);
            colors.push(chr.foreground);

            indices.push(index_count);
            indices.push(index_count + 1);
            indices.push(index_count + 2);

            indices.push(index_count + 3);
            indices.push(index_count + 2);
            indices.push(index_count + 1);

            index_count += 4;
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

impl SparseConsoleBackend for SparseBackendWithBackground {
    fn new_mesh(&self, front_end: &SparseConsole, meshes: &mut Assets<Mesh>) -> Handle<Mesh> {
        meshes.add(self.build_mesh(front_end))
    }

    fn spawn(&self, commands: &mut Commands, material: Handle<ColorMaterial>, idx: usize) {
        if let Some(mesh_handle) = &self.mesh_handle {
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: mesh_handle.clone().into(),
                    transform: Transform::default(),
                    material,
                    ..default()
                })
                .insert(BracketMesh(idx));
        }
    }
}
