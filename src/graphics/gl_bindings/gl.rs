include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(feature = "gl_debug")]
macro_rules! gl_call {
    ($x:expr) => {
        unsafe {
            $crate::graphics::gl_bindings::gl_clear_error();
            let res = $x;
            $crate::graphics::gl_bindings::gl_print_err(&format!("File {}, Ln {}, Col {}", file!(), line!(), column!()));
            res
        } 
    };
}

#[cfg(not(feature = "gl_debug"))]
macro_rules! gl_call {
    ($x:expr) => {
        unsafe { $x } 
    };
}

pub(super) use gl_call;

#[cfg(feature = "gl_debug")]
unsafe fn gl_clear_error() {
    while GetError() != NO_ERROR {}
}

#[cfg(feature = "gl_debug")]
unsafe fn gl_print_err(print_metadata: &str) {
    loop {
        let err = GetError();
        if err == NO_ERROR {
            return;
        }

        let err = match err {
            INVALID_ENUM => "GL_INVALID_ENUM",
            INVALID_VALUE => "GL_INVALID_VALUE",
            INVALID_OPERATION => "GL_INVALID_OPERATION",
            INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
            OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
            STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
            STACK_OVERFLOW => "GL_STACK_OVERFLOW",
            _ => unreachable!()
        };

        eprintln!("{print_metadata}\t\t GL_ERROR: {err}");
    }
}
