#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Uniform {
    Float(f32),
    Float2(f32, f32),
    Float3(f32, f32, f32),
    Float4(f32, f32, f32, f32),
    Int(i32),
    Uint(u32),
}