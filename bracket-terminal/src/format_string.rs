use bracket_color::prelude::*;

#[derive(Debug)]
pub struct ColoredTextSpans {
    pub length: usize,
    pub spans: Vec<(RGB, String)>,
}

fn find_color(col_name: &str) -> RGB {
    if let Some(palette) = palette_color(col_name) {
        palette
    } else {
        RGB::from_u8(255, 255, 255)
    }
}

impl ColoredTextSpans {
    pub fn new(text: &str) -> Self {
        let mut result = Self {
            length: 0,
            spans: Vec::new(),
        };
        let mut color_stack = Vec::new();

        for color_span in text.to_owned().split("#[") {
            if color_span.is_empty() {
                continue;
            }
            let mut col_text = color_span.splitn(2, ']');
            let col_name = col_text.next().unwrap();
            if let Some(text_span) = col_text.next() {
                if !col_name.is_empty() {
                    color_stack.push(find_color(col_name));
                } else {
                    color_stack.pop();
                }
                result.spans.push((
                    *color_stack.last().unwrap_or(&RGB::from_u8(255, 255, 255)),
                    text_span.to_string(),
                ));
                result.length += text_span.chars().count();
            }
        }

        result
    }
}
