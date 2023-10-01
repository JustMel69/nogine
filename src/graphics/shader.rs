use std::{fmt::Display, ffi::CString};

use super::super::gl_call;

pub enum SubShaderType {
    Vert, Frag
}

impl Display for SubShaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubShaderType::Vert => write!(f, "Vertex"),
            SubShaderType::Frag => write!(f, "Fragment"),
        }
    }
}


pub struct SubShader {
    id: gl::types::GLuint,
    kind: SubShaderType,
}

impl SubShader {
    pub const fn invalid() -> Self {
        Self { id: 0, kind: SubShaderType::Vert }
    }
    
    pub fn new(src: &str, kind: SubShaderType) -> Self {
        let id = gl_call!(gl::CreateShader(match kind {
            SubShaderType::Vert => gl::VERTEX_SHADER,
            SubShaderType::Frag => gl::FRAGMENT_SHADER,
        }));

        let src = CString::new(src).unwrap();
        gl_call!(gl::ShaderSource(id, 1, &src.as_ptr(), std::ptr::null()));
        gl_call!(gl::CompileShader(id));

        let mut success = 0;
        let mut info_log = [0u8; 512];
        gl_call!(gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success));
        if success == 0 {
            gl_call!(gl::GetShaderInfoLog(id, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut i8));
            let str_form = std::str::from_utf8(&info_log).unwrap();
            eprintln!("{kind} Shader Compilation Error: {str_form}");
        }

        Self { id, kind }
    }
}

impl Drop for SubShader {
    fn drop(&mut self) {
        if self.id == 0 {
            return;
        }

        gl_call!(gl::DeleteShader(self.id));
    }
}


pub struct Shader {
    id: gl::types::GLuint,
    _vert: SubShader,
    _frag: SubShader,
}

impl Shader {
    pub const fn invalid() -> Self {
        Self { id: 0, _vert: SubShader::invalid(), _frag: SubShader::invalid() }
    }

    pub fn new(vert: SubShader, frag: SubShader) -> Self {
        assert!(matches!(vert.kind, SubShaderType::Vert));
        assert!(matches!(frag.kind, SubShaderType::Frag));

        let id = gl_call!(gl::CreateProgram());
        gl_call!(gl::AttachShader(id, vert.id));
        gl_call!(gl::AttachShader(id, frag.id));
        gl_call!(gl::LinkProgram(id));

        let mut success = 0;
        let mut info_log = [0u8; 512];
        gl_call!(gl::GetProgramiv(id, gl::LINK_STATUS, &mut success));
        if success == 0 {
            gl_call!(gl::GetProgramInfoLog(id, 512, std::ptr::null_mut(), info_log.as_mut_ptr() as *mut i8));
            let str_form = std::str::from_utf8(&info_log).unwrap();
            eprintln!("Shader Linking Error: {str_form}");
        }

        return Self { _vert: vert, _frag: frag, id };
    }

    pub fn enable(&self) {
        gl_call!(gl::UseProgram(self.id));
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        if self.id == 0 {
            return;
        }

        gl_call!(gl::DeleteProgram(self.id));
    }
}