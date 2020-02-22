/// This module handles binding external web elements, for example movement buttons.
/// Based heavily on Zireael07's pull request, but modified to be significantly more generic.
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub static mut GLOBAL_BUTTON: Option<String> = None;

pub fn register_html_button<S: ToString>(element_id: S) {
    let button = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id(&element_id.to_string())
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();

    let html_callback = Closure::wrap(Box::new(|e: web_sys::Event| {
        on_external_element_click(e.clone());
    }) as Box<dyn FnMut(_)>);

    button.set_onclick(Some(html_callback.as_ref().unchecked_ref()));
    html_callback.forget();
}

pub fn on_external_element_click(event: web_sys::Event) {
    //set_command(Command::MoveLeft);
    unsafe {
        GLOBAL_BUTTON = Some(
            event
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap()
                .id(),
        );
        crate::console::log(format!("{}", GLOBAL_BUTTON.clone().unwrap()));
    }
}
