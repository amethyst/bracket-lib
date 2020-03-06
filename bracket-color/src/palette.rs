use crate::prelude::RGBA;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref PALETTE: Mutex<HashMap<String, RGBA>> = Mutex::new(HashMap::new());
}

/// Register a palette color by name with the global registry.
pub fn register_palette_color<S: ToString, COLOR: Into<RGBA>>(name: S, color: COLOR) {
    PALETTE
        .lock()
        .unwrap()
        .insert(name.to_string(), color.into());
}

/// Retrieve a palette color by name from the global registry.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::needless_pass_by_value)]
pub fn palette_color<S: ToString>(name: &S) -> Option<RGBA> {
    let plock = PALETTE.lock().unwrap();
    if let Some(col) = plock.get(&name.to_string()) {
        Some(*col)
    } else {
        None
    }
}
