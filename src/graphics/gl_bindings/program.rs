use crate::{graphics::{gl_bindings::gl, shader::ShaderError}, Res};

use super::{gl::gl_call, gl_uint, shader::GlShader};

pub struct GlProgram {
    id: gl_uint,
}

impl GlProgram {
    pub fn new(vert: &GlShader, frag: &GlShader) -> Res<Self, ShaderError> {
        let id = gl_call!(gl::CreateProgram());
        gl_call!(gl::AttachShader(id, vert.id()));
        gl_call!(gl::AttachShader(id, frag.id()));
        gl_call!(gl::LinkProgram(id));

        let mut success = 0;
        let mut info_log = [0u8; 512];
        gl_call!(gl::GetProgramiv(id, gl::LINK_STATUS, &mut success));
        if success == 0 {
            gl_call!(gl::GetProgramInfoLog(id, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut i8));
            let str_form = std::str::from_utf8(&info_log).unwrap();
            return Err(ShaderError::LinkingError { msg: str_form.into() });
        }

        return Ok(Self { id } );
    }

    pub fn id(&self) -> gl_uint {
        self.id
    }

    pub fn enable(&self) {
        gl_call!(gl::UseProgram(self.id));
    }
}

impl Drop for GlProgram {
    fn drop(&mut self) {
        gl_call!(gl::DeleteProgram(self.id));
    }
}
