use crate::prelude::RGB;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref PALETTE: Mutex<HashMap<String, RGB>> = Mutex::new(HashMap::new());
}

/// Register a palette color by name with the global registry.
pub fn register_palette_color<S: ToString>(name: S, color: RGB) {
    let color_name = name.to_string();
    PALETTE.lock().unwrap().insert(color_name, color);
}

/// Retrieve a palette color by name from the global registry.
#[allow(clippy::module_name_repetitions)]
pub fn palette_color<S: ToString>(name: S) -> Option<RGB> {
    let plock = PALETTE.lock().unwrap();
    if let Some(col) = plock.get(&name.to_string()) {
        Some(*col)
    } else {
        None
    }
}
