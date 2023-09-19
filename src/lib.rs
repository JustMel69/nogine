#![feature(const_fn_floating_point_arithmetic)]

pub mod window;
pub mod input;
pub mod color;
pub mod graphics;
pub mod math;

type Res<T, E> = Result<T, E>;


#[cfg(debug_assertions)]
macro_rules! gl_call {
    ($x:expr) => {
        unsafe { $x } 
    };
}

#[cfg(not(debug_assertions))]
macro_rules! gl_call {
    ($x:expr) => {
        unsafe { x } 
    };
}

pub(crate) use gl_call;