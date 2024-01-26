#![feature(const_fn_floating_point_arithmetic)]
#![feature(const_maybe_uninit_zeroed)]

pub mod window;
pub mod input;
pub mod color;
pub mod graphics;
pub mod math;
pub mod audio;
pub mod utils {
    pub mod heap;
    pub mod rng;
    pub mod timer;
    pub(crate) mod ptr_slice;
}
pub mod resource;
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

use std::fmt::Display;

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

pub struct Version { pub major: u32, pub minor: u32, pub patch: u32 }
impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

pub fn version() -> Version {
    return Version { major: 0, minor: 3, patch: 2 };
}