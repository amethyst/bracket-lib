use crate::BracketContext;
use bracket_color::prelude::*;

#[derive(Debug)]
pub struct ColoredTextSpans {
    pub length: usize,
    pub spans: Vec<(RGBA, String)>,
}

impl ColoredTextSpans {
    pub fn new(context: &BracketContext, text: &str) -> Self {
        let black: RGBA = RGBA::from_u8(0, 0, 0, 255);
        let mut result = Self {
            length: 0,
            spans: Vec::new(),
        };
        let mut color_stack: Vec<&RGBA> = Vec::new();

        for color_span in text.split("#[") {
            if color_span.is_empty() {
                continue;
            }
            let mut col_text = color_span.splitn(2, ']');
            let col_name = col_text.next().unwrap();
            if let Some(text_span) = col_text.next() {
                if !col_name.is_empty() {
                    color_stack.push(context.get_named_color(col_name).unwrap_or(&black));
                } else {
                    color_stack.pop();
                }
                result.spans.push((
                    **color_stack
                        .last()
                        .unwrap_or(&&RGBA::from_u8(255, 255, 255, 255)),
                    text_span.to_string(),
                ));
                result.length += text_span.chars().count();
            }
        }

        result
    }
}
