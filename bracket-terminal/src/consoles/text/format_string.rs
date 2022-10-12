use bracket_color::prelude::*;

#[derive(Debug)]
pub struct ColoredTextSpans {
    pub length: usize,
    pub spans: Vec<(RGBA, String)>,
}

fn find_color(col_name: &str) -> RGBA {
    if let Some(palette) = palette_color(&col_name) {
        palette
    } else {
        RGBA::from_u8(255, 255, 255, 255)
    }
}

impl ColoredTextSpans {
    pub fn new(text: &str) -> Self {
        let mut result = Self {
            length: 0,
            spans: Vec::new(),
        };
        let mut color_stack = Vec::new();

        let text: String = match text.starts_with("#[") {
            true => text.to_owned(),
            false => "#[white]".to_string() + text,
        };

        for color_span in text.split("#[") {
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
                    *color_stack
                        .last()
                        .unwrap_or(&RGBA::from_u8(255, 255, 255, 255)),
                    text_span.to_string(),
                ));
                result.length += text_span.chars().count();
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::prelude::*;

    #[test]
    fn no_colors() {
        let text = "This is a simple message.";
        let span = ColoredTextSpans::new(text);
        assert_eq!(span.length, text.len());
        assert_eq!(span.spans.len(), 1);
        assert_eq!(span.spans[0].0, RGBA::from_u8(255, 255, 255, 255));
        assert_eq!(span.spans[0].1, text);
    }

    #[test]
    fn no_start_color() {
        register_palette_color("blue", RGBA::from_u8(0, 0, 255, 255));

        let text = "This is a #[blue]simple#[] message.";
        let span = ColoredTextSpans::new(text);
        assert_eq!(span.length, 25);
        assert_eq!(span.spans.len(), 3);
        assert_eq!(span.spans[0].0, RGBA::from_u8(255, 255, 255, 255));
        assert_eq!(span.spans[0].1, "This is a ");
        assert_eq!(span.spans[1].0, RGBA::from_u8(0, 0, 255, 255));
        assert_eq!(span.spans[1].1, "simple");
        assert_eq!(span.spans[2].0, RGBA::from_u8(255, 255, 255, 255));
        assert_eq!(span.spans[2].1, " message.");
    }

    #[test]
    fn start_color() {
        let text = "#[white]This is a simple message.";
        let span = ColoredTextSpans::new(text);
        assert_eq!(span.length, text.len() - "$[white]".len());
        assert_eq!(span.spans.len(), 1);
        assert_eq!(span.spans[0].0, RGBA::from_u8(255, 255, 255, 255));
        assert_eq!(span.spans[0].1, "This is a simple message.");
    }

    #[test]
    fn color_with_pop() {
        register_palette_color("blue", RGBA::from_u8(0, 0, 255, 255));

        let text = "#[white]This is a #[blue]simple#[] message.";
        let span = ColoredTextSpans::new(text);
        assert_eq!(span.length, 25);
        assert_eq!(span.spans.len(), 3);
        assert_eq!(span.spans[0].0, RGBA::from_u8(255, 255, 255, 255));
        assert_eq!(span.spans[0].1, "This is a ");
        assert_eq!(span.spans[1].0, RGBA::from_u8(0, 0, 255, 255));
        assert_eq!(span.spans[1].1, "simple");
        assert_eq!(span.spans[2].0, RGBA::from_u8(255, 255, 255, 255));
        assert_eq!(span.spans[2].1, " message.");
    }

    #[test]
    fn pop_color() {
        let text = "#[white]This#[] is a simple message.";
        let span = ColoredTextSpans::new(text);
        assert_eq!(span.length, 25);
        assert_eq!(span.spans.len(), 2);
        assert_eq!(span.spans[0].0, RGBA::from_u8(255, 255, 255, 255));
        assert_eq!(span.spans[0].1, "This");
        assert_eq!(span.spans[1].0, RGBA::from_u8(255, 255, 255, 255));
        assert_eq!(span.spans[1].1, " is a simple message.");
    }
}
