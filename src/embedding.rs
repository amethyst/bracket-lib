use std::collections::HashMap;

use std::sync::Mutex;

const TERMINAL_8_8_BYTES: &'static [u8] = include_bytes!("../resources/terminal8x8.jpg");
const TERMINAL_8_16_BYTES: &'static [u8] = include_bytes!("../resources/vga8x16.jpg");

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
        dict.add_resource("resources/terminal8x8.jpg".to_string(), TERMINAL_8_8_BYTES);
        dict.add_resource("resources/vga8x16.jpg".to_string(), TERMINAL_8_16_BYTES);
        dict
    }

    pub fn get_resource(&self, path: String) -> Option<&'static [u8]> {
        if self.entries.contains_key(&path) {
            return Some(&self.entries[&path]);
        }
        None
    }

    pub fn add_resource(&mut self, path: String, bytes: &'static [u8]) {
        self.entries.insert(path, bytes);
    }
}
