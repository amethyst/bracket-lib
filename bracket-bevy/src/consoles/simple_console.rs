use bevy::{prelude::{Component, Handle, Mesh, Color}, render::mesh::{PrimitiveTopology, Indices}};
use crate::cp437::string_to_cp437;

#[derive(Component)]
pub struct SimpleConsoleMarker(pub usize);

#[derive(Clone, Copy)]
pub(crate) struct TerminalGlyph {
    glyph: u16,
    foreground: [f32; 4]
}

impl Default for TerminalGlyph {
    fn default() -> Self {
        Self {
            glyph: 65,
            foreground: Color::WHITE.as_rgba_f32(),
        }
    }
}

pub(crate) struct SimpleConsole {
    pub(crate) font_index: usize,
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) terminal: Vec<TerminalGlyph>,
    pub(crate) mesh_handle: Option<Handle<Mesh>>
}


impl SimpleConsole {
    pub fn new(font_index: usize, width: usize, height: usize) -> Self {
        Self {
            font_index, width, height, terminal: vec![TerminalGlyph::default(); width*height],
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

    pub fn build_mesh(&self, chars_per_row: u16, n_rows: u16, font_height_pixels: (f32, f32)) -> Mesh {
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(self.width * self.height * 4);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(self.width * self.height * 4);
        let mut uv: Vec<[f32; 2]> = Vec::with_capacity(self.width * self.height * 4);
        let mut colors: Vec<[f32; 4]> = Vec::with_capacity(self.width * self.height * 4);
        let mut indices: Vec<u32> = Vec::with_capacity(self.width * self.height * 6);
        let mut index_count = 0;
        let half_height = self.height as f32 / 2.0;
        let half_width = self.width as f32 / 2.0;
        for y in 0..self.height {
            let screen_y = (y as f32 - half_height) * font_height_pixels.1;
            let mut idx = (self.height-1 -y) * self.width;
            for x in 0..self.width {
                let screen_x = (x as f32 - half_width) * font_height_pixels.0;
                vertices.push([ screen_x, screen_y, 0.0 ]);
                vertices.push([ screen_x + font_height_pixels.0, screen_y, 0.0 ]);
                vertices.push([ screen_x, screen_y + font_height_pixels.1, 0.0 ]);
                vertices.push([ screen_x + font_height_pixels.0, screen_y + font_height_pixels.1, 0.0 ]);
                for _ in 0..4 {
                    normals.push([0.0, 1.0, 0.0]);
                }
                let tex = self.texture_coords(self.terminal[idx].glyph, chars_per_row, n_rows);
                uv.push([tex[0], tex[3]]);
                uv.push([tex[2], tex[3]]);
                uv.push([tex[0], tex[1]]);
                uv.push([tex[2], tex[1]]);

                colors.push(self.terminal[idx].foreground);
                colors.push(self.terminal[idx].foreground);
                colors.push(self.terminal[idx].foreground);
                colors.push(self.terminal[idx].foreground);

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

    pub fn build_uvs(&self, chars_per_row: u16, n_rows: u16) -> Vec<[f32; 2]> {
        let mut uv: Vec<[f32; 2]> = Vec::with_capacity(self.width * self.height * 4);
        for y in 0..self.height {
            let mut idx = y * self.width;
            for _ in 0..self.width {
                let tex = self.texture_coords(self.terminal[idx].glyph, chars_per_row, n_rows);
                uv.push([tex[0], tex[3]]);
                uv.push([tex[2], tex[3]]);
                uv.push([tex[0], tex[1]]);
                uv.push([tex[2], tex[1]]);
                idx += 1;
            }
        }
        uv
    }

    pub fn build_colors(&self) -> Vec<[f32; 4]> {
        let mut colors: Vec<[f32; 4]> = Vec::with_capacity(self.width * self.height * 4);
        for y in 0..self.height {
            let mut idx = y * self.width;
            for _ in 0..self.width {
                colors.push(self.terminal[idx].foreground);
                colors.push(self.terminal[idx].foreground);
                colors.push(self.terminal[idx].foreground);
                colors.push(self.terminal[idx].foreground);
                idx += 1;
            }
        }
        colors
    }

    pub fn cls(&mut self) {
        self.terminal.iter_mut().for_each(|c| c.glyph = 32);
    }

    pub fn print<S: ToString>(&mut self, mut x: usize, y: usize, text: S) {
        let bytes = string_to_cp437(&text.to_string());
        for glyph in bytes {
            let idx = self.at(x, y);
            self.terminal[idx] = TerminalGlyph{ glyph, foreground: Color::WHITE.as_rgba_f32() };
            x += 1;
        }
    }

    pub fn print_color<S: ToString>(&mut self, mut x: usize, y: usize, text: S, foreground: Color) {
        let bytes = string_to_cp437(&text.to_string());
        for glyph in bytes {
            let idx = self.at(x, y);
            self.terminal[idx] = TerminalGlyph{ glyph, foreground: foreground.as_rgba_f32() };
            x += 1;
        }
    }

    fn at(&self, x: usize, y: usize) -> usize {
        ((self.height - 1 - y) * self.width) + x
    }
}