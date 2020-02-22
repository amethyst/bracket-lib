use super::super::VirtualKeyCode;

/// Provides a global variable for keyboard events to be written to. I'm not really
/// a fan of using globals, but it was hard to find an alternative given the separation
/// between the web-side and the wasm side.
pub static mut GLOBAL_KEY: Option<VirtualKeyCode> = None;

/// Global for handling modifier key-state.
pub static mut GLOBAL_MODIFIERS: (bool, bool, bool) = (false, false, false);

/// Handler for on_key events from the browser. Sets the global variables, which are then
/// referenced by the main loop.
pub fn on_key(key: web_sys::KeyboardEvent) {
    //super::console::log("Key Event");
    unsafe {
        if key.get_modifier_state("Shift") {
            GLOBAL_MODIFIERS.0 = true;
        }
        if key.get_modifier_state("Control") {
            GLOBAL_MODIFIERS.1 = true;
        }
        if key.get_modifier_state("Alt") {
            GLOBAL_MODIFIERS.2 = true;
        }

        let code = key.key_code();
        match code {
            8 => GLOBAL_KEY = Some(VirtualKeyCode::Back),
            9 => GLOBAL_KEY = Some(VirtualKeyCode::Tab),
            13 => GLOBAL_KEY = Some(VirtualKeyCode::Return),
            20 => GLOBAL_KEY = Some(VirtualKeyCode::Capital),
            27 => GLOBAL_KEY = Some(VirtualKeyCode::Escape),
            32 => GLOBAL_KEY = Some(VirtualKeyCode::Space),
            33 => GLOBAL_KEY = Some(VirtualKeyCode::PageUp),
            34 => GLOBAL_KEY = Some(VirtualKeyCode::PageDown),
            35 => GLOBAL_KEY = Some(VirtualKeyCode::End),
            36 => GLOBAL_KEY = Some(VirtualKeyCode::Home),
            37 => GLOBAL_KEY = Some(VirtualKeyCode::Left),
            38 => GLOBAL_KEY = Some(VirtualKeyCode::Up),
            39 => GLOBAL_KEY = Some(VirtualKeyCode::Right),
            40 => GLOBAL_KEY = Some(VirtualKeyCode::Down),
            45 => GLOBAL_KEY = Some(VirtualKeyCode::Insert),
            46 => GLOBAL_KEY = Some(VirtualKeyCode::Delete),
            48 => GLOBAL_KEY = Some(VirtualKeyCode::Key0),
            49 => GLOBAL_KEY = Some(VirtualKeyCode::Key1),
            50 => GLOBAL_KEY = Some(VirtualKeyCode::Key2),
            51 => GLOBAL_KEY = Some(VirtualKeyCode::Key3),
            52 => GLOBAL_KEY = Some(VirtualKeyCode::Key4),
            53 => GLOBAL_KEY = Some(VirtualKeyCode::Key5),
            54 => GLOBAL_KEY = Some(VirtualKeyCode::Key6),
            55 => GLOBAL_KEY = Some(VirtualKeyCode::Key7),
            56 => GLOBAL_KEY = Some(VirtualKeyCode::Key8),
            57 => GLOBAL_KEY = Some(VirtualKeyCode::Key9),
            65 => GLOBAL_KEY = Some(VirtualKeyCode::A),
            66 => GLOBAL_KEY = Some(VirtualKeyCode::B),
            67 => GLOBAL_KEY = Some(VirtualKeyCode::C),
            68 => GLOBAL_KEY = Some(VirtualKeyCode::D),
            69 => GLOBAL_KEY = Some(VirtualKeyCode::E),
            70 => GLOBAL_KEY = Some(VirtualKeyCode::F),
            71 => GLOBAL_KEY = Some(VirtualKeyCode::G),
            72 => GLOBAL_KEY = Some(VirtualKeyCode::H),
            73 => GLOBAL_KEY = Some(VirtualKeyCode::I),
            74 => GLOBAL_KEY = Some(VirtualKeyCode::J),
            75 => GLOBAL_KEY = Some(VirtualKeyCode::K),
            76 => GLOBAL_KEY = Some(VirtualKeyCode::L),
            77 => GLOBAL_KEY = Some(VirtualKeyCode::M),
            78 => GLOBAL_KEY = Some(VirtualKeyCode::N),
            79 => GLOBAL_KEY = Some(VirtualKeyCode::O),
            80 => GLOBAL_KEY = Some(VirtualKeyCode::P),
            81 => GLOBAL_KEY = Some(VirtualKeyCode::Q),
            82 => GLOBAL_KEY = Some(VirtualKeyCode::R),
            83 => GLOBAL_KEY = Some(VirtualKeyCode::S),
            84 => GLOBAL_KEY = Some(VirtualKeyCode::T),
            85 => GLOBAL_KEY = Some(VirtualKeyCode::U),
            86 => GLOBAL_KEY = Some(VirtualKeyCode::V),
            87 => GLOBAL_KEY = Some(VirtualKeyCode::W),
            88 => GLOBAL_KEY = Some(VirtualKeyCode::X),
            89 => GLOBAL_KEY = Some(VirtualKeyCode::Y),
            90 => GLOBAL_KEY = Some(VirtualKeyCode::Z),
            97 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad1),
            98 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad2),
            99 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad3),
            100 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad4),
            101 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad5),
            102 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad6),
            103 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad7),
            104 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad8),
            105 => GLOBAL_KEY = Some(VirtualKeyCode::Numpad9),
            106 => GLOBAL_KEY = Some(VirtualKeyCode::Multiply),
            107 => GLOBAL_KEY = Some(VirtualKeyCode::Add),
            109 => GLOBAL_KEY = Some(VirtualKeyCode::Subtract),
            111 => GLOBAL_KEY = Some(VirtualKeyCode::Divide),
            186 => GLOBAL_KEY = Some(VirtualKeyCode::Semicolon),
            187 => GLOBAL_KEY = Some(VirtualKeyCode::Equals),
            188 => GLOBAL_KEY = Some(VirtualKeyCode::Comma),
            189 => GLOBAL_KEY = Some(VirtualKeyCode::Minus),
            190 => GLOBAL_KEY = Some(VirtualKeyCode::Period),
            191 => GLOBAL_KEY = Some(VirtualKeyCode::Slash),
            192 => GLOBAL_KEY = Some(VirtualKeyCode::Grave),
            219 => GLOBAL_KEY = Some(VirtualKeyCode::LBracket),
            220 => GLOBAL_KEY = Some(VirtualKeyCode::Backslash),
            221 => GLOBAL_KEY = Some(VirtualKeyCode::RBracket),
            222 => GLOBAL_KEY = Some(VirtualKeyCode::Apostrophe),
            _ => {
                GLOBAL_KEY = None;
                crate::console::log(&format!("Keycode: {}", code));
            }
        }
    }
}
