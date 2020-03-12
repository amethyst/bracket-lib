use crate::prelude::{DrawBatch, string_to_cp437, Console, FontCharType, Tile};
use bracket_geometry::prelude::Point;
use bracket_color::prelude::{RGB, RGBA, ColorPair};

pub struct TextBlock {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    fg: RGBA,
    bg: RGBA,
    buffer: Vec<Tile>,
    cursor: (i32, i32),
}

impl TextBlock {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> TextBlock {
        TextBlock {
            x,
            y,
            width,
            height,
            fg: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
            bg: RGBA::from_f32(0.0, 0.0, 0.0, 1.0),
            buffer: vec![
                Tile {
                    glyph: 0,
                    fg: RGBA::from_f32(1.0, 1.0, 1.0, 1.0),
                    bg: RGBA::from_f32(0.0, 0.0, 0.0, 1.0)
                };
                width as usize * height as usize
            ],
            cursor: (0, 0),
        }
    }

    pub fn fg<COLOR>(&mut self, fg: RGB)
    where
        COLOR: Into<RGBA>,
    {
        self.fg = fg.into();
    }

    pub fn bg<COLOR>(&mut self, bg: COLOR)
    where
        COLOR: Into<RGBA>,
    {
        self.bg = bg.into();
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        self.cursor = (x, y);
    }

    fn at(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    pub fn render(&self, mut console: impl AsMut<dyn Console>) {
        for y in 0..self.height {
            for x in 0..self.width {
                console.as_mut().set(
                    x + self.x,
                    y + self.y,
                    self.buffer[self.at(x, y)].fg,
                    self.buffer[self.at(x, y)].bg,
                    self.buffer[self.at(x, y)].glyph,
                );
            }
        }
    }

    pub fn render_to_draw_batch(&self, draw_batch: &mut DrawBatch) {
        for y in 0..self.height {
            for x in 0..self.width {
                draw_batch.set(
                    Point::new(x + self.x, y + self.y),
                    ColorPair::new(self.buffer[self.at(x, y)].fg, self.buffer[self.at(x, y)].bg),
                    self.buffer[self.at(x, y)].glyph,
                );
            }
        }
    }

    pub fn print(&mut self, text: &TextBuilder) {
        for cmd in &text.commands {
            match cmd {
                CommandType::Text { block: t } => {
                    for c in t {
                        let idx = self.at(self.cursor.0, self.cursor.1);
                        self.buffer[idx].glyph = *c;
                        self.buffer[idx].fg = self.fg;
                        self.buffer[idx].bg = self.bg;
                        self.cursor.0 += 1;
                        if self.cursor.0 >= self.width {
                            self.cursor.0 = 0;
                            self.cursor.1 += 1;
                        }
                    }
                }

                CommandType::Centered { block: t } => {
                    let text_width = t.len() as i32;
                    let half_width = text_width / 2;
                    self.cursor.0 = (self.width / 2) - half_width;
                    for c in t {
                        let idx = self.at(self.cursor.0, self.cursor.1);
                        self.buffer[idx].glyph = *c;
                        self.buffer[idx].fg = self.fg;
                        self.buffer[idx].bg = self.bg;
                        self.cursor.0 += 1;
                        if self.cursor.0 >= self.width {
                            self.cursor.0 = 0;
                            self.cursor.1 += 1;
                        }
                    }
                }

                CommandType::NewLine {} => {
                    self.cursor.0 = 0;
                    self.cursor.1 += 1;
                }

                CommandType::Foreground { col } => self.fg = *col,
                CommandType::Background { col } => self.bg = *col,
                CommandType::Reset {} => {
                    self.cursor = (0, 0);
                    self.fg = RGBA::from_f32(1.0, 1.0, 1.0, 1.0);
                    self.bg = RGBA::from_f32(0.0, 0.0, 0.0, 1.0);
                }

                CommandType::TextWrapper { block: t } => {
                    for word in t.split(' ') {
                        let mut chrs = string_to_cp437(&word);
                        chrs.push(32);
                        if self.cursor.0 + chrs.len() as i32 >= self.width {
                            self.cursor.0 = 0;
                            self.cursor.1 += 1;
                        }
                        for c in chrs {
                            let idx = self.at(self.cursor.0, self.cursor.1);
                            self.buffer[idx].glyph = c;
                            self.buffer[idx].fg = self.fg;
                            self.buffer[idx].bg = self.bg;
                            self.cursor.0 += 1;
                            if self.cursor.0 >= self.width {
                                self.cursor.0 = 0;
                                self.cursor.1 += 1;
                            }
                        }
                    }
                }
            }
        }
    }
}

pub enum CommandType {
    Text { block: Vec<FontCharType> },
    Centered { block: Vec<FontCharType> },
    NewLine {},
    Foreground { col: RGBA },
    Background { col: RGBA },
    TextWrapper { block: String },
    Reset {},
}

pub struct TextBuilder {
    commands: Vec<CommandType>,
}

impl TextBuilder {
    pub fn empty() -> TextBuilder {
        TextBuilder {
            commands: Vec::new(),
        }
    }

    pub fn append(&mut self, text: &str) -> &mut Self {
        let chrs = string_to_cp437(&text);
        self.commands.push(CommandType::Text { block: chrs });
        self
    }
    pub fn centered(&mut self, text: &str) -> &mut Self {
        let chrs = string_to_cp437(&text);
        self.commands.push(CommandType::Centered { block: chrs });
        self
    }
    pub fn reset(&mut self) -> &mut Self {
        self.commands.push(CommandType::Reset {});
        self
    }
    pub fn ln(&mut self) -> &mut Self {
        self.commands.push(CommandType::NewLine {});
        self
    }
    pub fn fg<COLOR>(&mut self, col: COLOR) -> &mut Self
    where
        COLOR: Into<RGBA>,
    {
        self.commands
            .push(CommandType::Foreground { col: col.into() });
        self
    }
    pub fn bg<COLOR>(&mut self, col: COLOR) -> &mut Self
    where
        COLOR: Into<RGBA>,
    {
        self.commands
            .push(CommandType::Background { col: col.into() });
        self
    }
    pub fn line_wrap(&mut self, text: &str) -> &mut Self {
        self.commands.push(CommandType::TextWrapper {
            block: text.to_string(),
        });
        self
    }
}
