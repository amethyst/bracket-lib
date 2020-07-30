mod keyboard;
pub use keyboard::*;
mod mouse;
pub use mouse::*;
mod external;
pub use external::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub fn bind_wasm_events(canvas: &web_sys::HtmlCanvasElement) {
    // Handle keyboard input
    let key_callback = Closure::wrap(Box::new(|e: web_sys::KeyboardEvent| {
        on_key(e.clone());
    }) as Box<dyn FnMut(_)>);

    let document = web_sys::window().unwrap();
    document.set_onkeydown(Some(key_callback.as_ref().unchecked_ref()));
    key_callback.forget();

    let keyup_callback = Closure::wrap(Box::new(|e: web_sys::KeyboardEvent| {
        on_key_up(e.clone());
    }) as Box<dyn FnMut(_)>);
    document.set_onkeyup(Some(keyup_callback.as_ref().unchecked_ref()));
    keyup_callback.forget();

    // Handle mouse moving
    let mousemove_callback = Closure::wrap(Box::new(|e: web_sys::MouseEvent| {
        on_mouse_move(e.clone());
    }) as Box<dyn FnMut(_)>);

    canvas.set_onmousemove(Some(mousemove_callback.as_ref().unchecked_ref()));
    mousemove_callback.forget();

    // Handle mouse clicking
    let mouseclick_callback = Closure::wrap(Box::new(|e: web_sys::MouseEvent| {
        on_mouse_down(e.clone());
    }) as Box<dyn FnMut(_)>);

    canvas.set_onmousedown(Some(mouseclick_callback.as_ref().unchecked_ref()));
    mouseclick_callback.forget();

    // Handle mouse release
    let mouseunclick_callback = Closure::wrap(Box::new(|e: web_sys::MouseEvent| {
        on_mouse_up(e.clone());
    }) as Box<dyn FnMut(_)>);

    canvas.set_onmouseup(Some(mouseunclick_callback.as_ref().unchecked_ref()));
    mouseunclick_callback.forget();
}
