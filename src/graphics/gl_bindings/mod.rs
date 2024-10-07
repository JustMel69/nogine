pub mod buffer;
pub mod shader;
pub mod program;

mod gl;

#[allow(non_camel_case_types)] pub type gl_enum = gl::types::GLenum;
#[allow(non_camel_case_types)] pub type gl_uint = gl::types::GLuint;

pub fn gl_enable_blend() {
    gl_call!(gl::Enable(gl::BLEND));
}

pub enum GlBlendingMode {
    AlphaMix,
    Additive,
    Multiplicative,
}

pub fn gl_set_blend(mode: GlBlendingMode) {
    match mode {
        GlBlendingMode::AlphaMix => gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)), 
        GlBlendingMode::Additive => gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE)),
        GlBlendingMode::Multiplicative => gl_call!(gl::BlendFunc(gl::DST_COLOR, gl::ZERO)),
    }
}
