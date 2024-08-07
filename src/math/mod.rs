use std::ops::{Add, Mul};

pub mod vector3;
pub mod vector2;
pub mod mat3x3;
pub mod rect;
pub mod quad;

// reexports to not break literally everything
pub use vector3::*;
pub use vector2::*;
pub use mat3x3::mat3;
pub use rect::*;


pub fn lerp<T: Add<Output = T> + Mul<f32, Output = T>>(a: T, b: T, t: f32) -> T {
    let t = t.clamp(0.0, 1.0);
    return b * t + a * (1.0 - t);
}

pub fn lerp_unclamped<T: Add<Output = T> + Mul<f32, Output = T>>(a: T, b: T, t: f32) -> T {
    return b * t + a * (1.0 - t);
}