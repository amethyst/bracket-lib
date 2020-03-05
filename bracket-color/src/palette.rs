use std::sync::Mutex;
use crate::prelude::RGB;
use std::collections::HashMap;

lazy_static! {
    static ref PALETTE: Mutex<HashMap<String, RGB>> =
        Mutex::new(HashMap::new());
}

pub fn register_palette_color<S:ToString>(name: S, color: RGB) {
    let color_name = name.to_string();
    PALETTE.lock().unwrap().insert(color_name, color);
}

pub fn palette_color<S:ToString>(name: S) -> Option<RGB> {
    let plock = PALETTE.lock().unwrap();
    if let Some(col) = plock.get(&name.to_string()) {
        Some(col.clone())
    } else {
        None
    }
}