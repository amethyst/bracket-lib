use std::collections::HashMap;

use parking_lot::Mutex;

const TERMINAL_8_8_BYTES: &[u8] = include_bytes!("../resources/terminal8x8.png");
const TERMINAL_8_16_BYTES: &[u8] = include_bytes!("../resources/vga8x16.png");

lazy_static! {
    pub static ref EMBED: Mutex<Dictionary> = Mutex::new(Dictionary::new());
}

#[derive(Default)]
pub struct Dictionary {
    entries: HashMap<String, &'static [u8]>,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        let mut dict = Dictionary {
            entries: HashMap::new(),
        };
        dict.add_resource("resources/terminal8x8.png".to_string(), TERMINAL_8_8_BYTES);
        dict.add_resource("resources/vga8x16.png".to_string(), TERMINAL_8_16_BYTES);
        dict
    }

    pub fn get_resource(&self, path: String) -> Option<&'static [u8]> {
        let fixed_path = if std::path::MAIN_SEPARATOR != '/' {
            path.replace(std::path::MAIN_SEPARATOR, "/")
        } else {
            path
        };

        if self.entries.contains_key(&fixed_path) {
            return Some(self.entries[&fixed_path]);
        }
        None
    }

    pub fn add_resource(&mut self, path: String, bytes: &'static [u8]) {
        self.entries.insert(path, bytes);
    }
}
