use glow::HasContext;

#[macro_export]
macro_rules! gl_error_wrap {
    ($gl:expr, $call:expr) => {
        $call;
        #[cfg(debug_assertions)]
        crate::hal::gl_error($gl);
    };
}

pub fn gl_error(gl: &glow::Context) {
    let error;
    unsafe {
        error = gl.get_error();
    }
    if error != glow::NO_ERROR {
        match error {
            glow::INVALID_ENUM => panic!("[GL] Error: INVALID_ENUM"),
            glow::INVALID_VALUE => panic!("[GL] Error: INVALID_ENUM"),
            glow::INVALID_OPERATION => panic!("[GL] Error: INVALID_ENUM"),
            glow::STACK_OVERFLOW => panic!("[GL] Error: INVALID_ENUM"),
            glow::STACK_UNDERFLOW => panic!("[GL] Error: INVALID_ENUM"),
            glow::OUT_OF_MEMORY => panic!("[GL] Error: INVALID_ENUM"),
            glow::INVALID_FRAMEBUFFER_OPERATION => panic!("[GL] Error: INVALID_ENUM"),
            _ => panic!("[GL] Error: {}", error),
        }
    }
}
