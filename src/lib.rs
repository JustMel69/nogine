#![feature(const_fn_floating_point_arithmetic)]

pub mod window;
pub mod input;
pub mod color;
pub mod graphics;
pub mod math;
pub mod audio;
pub mod utils {
    pub mod heap;
}
pub mod logging;

type Res<T, E> = Result<T, E>;


#[cfg(feature = "gl_debug")]
macro_rules! gl_call {
    ($x:expr) => {
        unsafe {
            $crate::gl_clear_error();
            let res = $x;
            $crate::gl_print_err(&format!("File {}, Ln {}, Col {}", file!(), line!(), column!()));
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

pub(crate) use gl_call;

#[cfg(feature = "gl_debug")]
unsafe fn gl_clear_error() {
    while gl::GetError() != gl::NO_ERROR {}
}

#[cfg(feature = "gl_debug")]
unsafe fn gl_print_err(print_metadata: &str) {
    loop {
        let err = gl::GetError();
        if err == gl::NO_ERROR {
            return;
        }

        let err = match err {
            gl::INVALID_ENUM => "GL_INVALID_ENUM",
            gl::INVALID_VALUE => "GL_INVALID_VALUE",
            gl::INVALID_OPERATION => "GL_INVALID_OPERATION",
            gl::INVALID_FRAMEBUFFER_OPERATION => "GL_INVALID_FRAMEBUFFER_OPERATION",
            gl::OUT_OF_MEMORY => "GL_OUT_OF_MEMORY",
            gl::STACK_UNDERFLOW => "GL_STACK_UNDERFLOW",
            gl::STACK_OVERFLOW => "GL_STACK_OVERFLOW",
            _ => unreachable!()
        };

        eprintln!("{print_metadata}\t\t GL_ERROR: {err}");
    }
}

pub fn version() -> &'static str {
    return "v0.2.0";
}