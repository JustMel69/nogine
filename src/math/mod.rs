use std::ops::{Add, Mul};

pub mod vector2;
pub mod mat3x3;
pub mod rect;
pub mod rotrect;

// reexports to not break literally everything
pub use vector2::Vector2;
pub use mat3x3::Matrix3x3;
pub use rect::Rect;


pub fn lerp<T: Add<Output = T> + Mul<f32, Output = T>>(a: T, b: T, t: f32) -> T {
    let t = t.clamp(0.0, 1.0);
    return b * t + a * (1.0 - t);
}

pub fn lerp_unclamped<T: Add<Output = T> + Mul<f32, Output = T>>(a: T, b: T, t: f32) -> T {
    return b * t + a * (1.0 - t);
}