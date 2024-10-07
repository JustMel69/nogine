use std::ffi::CString;

use crate::{graphics::{gl_bindings::gl, shader::{ShaderError, SubShaderType}}, Res};

use super::{gl::gl_call, gl_uint};

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum GlShaderType {
    Vert = gl::VERTEX_SHADER,
    Frag = gl::FRAGMENT_SHADER,
}

impl From<SubShaderType> for GlShaderType {
    fn from(value: SubShaderType) -> Self {
        return unsafe { std::mem::transmute(value) };
    }
}

pub struct GlShader {
    id: gl_uint,
}

impl GlShader {
    pub fn new(src: &str, kind: GlShaderType) -> Res<Self, ShaderError> {
        let id = gl_call!(gl::CreateShader(kind as u32));

        let src = CString::new(src).map_err(|e| ShaderError::from(e))?;
        gl_call!(gl::ShaderSource(id, 1, &src.as_ptr(), std::ptr::null()));
        gl_call!(gl::CompileShader(id));

        let mut success = 0;
        let mut info_log = [0u8; 512];
        gl_call!(gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success));
        if success == 0 {
            gl_call!(gl::GetShaderInfoLog(id, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut i8));
            let str_form = std::str::from_utf8(&info_log).unwrap();
            return Err(ShaderError::CompilationError { kind, msg: str_form.into() } );
        }

        return Ok(Self { id });
    }

    pub fn id(&self) -> gl_uint {
        self.id
    }
}

impl Drop for GlShader {
    fn drop(&mut self) {
        gl_call!(gl::DeleteShader(self.id));
    }
}
