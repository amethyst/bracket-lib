use crate::prelude::{string_to_cp437, Console, Tile};
use bracket_color::prelude::RGB;

pub struct TextBlock {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    fg: RGB,
    bg: RGB,
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
            fg: RGB::from_f32(1.0, 1.0, 1.0),
            bg: RGB::from_f32(0.0, 0.0, 0.0),
            buffer: vec![
                Tile {
                    glyph: 0,
                    fg: RGB::from_f32(1.0, 1.0, 1.0),
                    bg: RGB::from_f32(0.0, 0.0, 0.0)
                };
                width as usize * height as usize
            ],
            cursor: (0, 0),
        }
    }

    pub fn fg(&mut self, fg: RGB) {
        self.fg = fg;
    }
    pub fn bg(&mut self, bg: RGB) {
        self.bg = bg;
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
                    self.fg = RGB::from_f32(1.0, 1.0, 1.0);
                    self.bg = RGB::from_f32(0.0, 0.0, 0.0);
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
    Text { block: Vec<u8> },
    Centered { block: Vec<u8> },
    NewLine {},
    Foreground { col: RGB },
    Background { col: RGB },
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
    pub fn fg(&mut self, col: RGB) -> &mut Self {
        self.commands.push(CommandType::Foreground { col });
        self
    }
    pub fn bg(&mut self, col: RGB) -> &mut Self {
        self.commands.push(CommandType::Background { col });
        self
    }
    pub fn line_wrap(&mut self, text: &str) -> &mut Self {
        self.commands.push(CommandType::TextWrapper {
            block: text.to_string(),
        });
        self
    }
}
