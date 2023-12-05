use std::{fmt::Display, ffi::CString, sync::Arc};

use super::super::gl_call;

#[derive(Clone)]
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


struct SubShaderCore(gl::types::GLuint);
impl Drop for SubShaderCore {
    fn drop(&mut self) {
        gl_call!(gl::DeleteShader(self.0));
    }
}

#[derive(Clone)]
pub struct SubShader {
    core: Option<Arc<SubShaderCore>>,
    kind: SubShaderType,
}

impl SubShader {
    pub const fn invalid() -> Self {
        Self { core: None, kind: SubShaderType::Vert }
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
            eprintln!("{kind} Shader Compilation Error:\n{str_form}");
        }

        Self { core: Some(Arc::new(SubShaderCore(id))), kind }
    }

    pub fn id(&self) -> u32 {
        return self.core.as_ref().unwrap().0;
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct ShaderCore(gl::types::GLuint);
impl Drop for ShaderCore {
    fn drop(&mut self) {
        gl_call!(gl::DeleteProgram(self.0));
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shader {
    core: Option<Arc<ShaderCore>>
}

impl Shader {
    pub const fn invalid() -> Self {
        Self { core: None }
    }

    pub fn new(vert: &SubShader, frag: &SubShader) -> Self {
        assert!(matches!(vert.kind, SubShaderType::Vert));
        assert!(matches!(frag.kind, SubShaderType::Frag));

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
            eprintln!("Shader Linking Error:\n{str_form}");
        }

        return Self { core: Some(Arc::new(ShaderCore(id))) };
    }

    pub fn enable(&self) {
        gl_call!(gl::UseProgram(self.id()));
    }

    pub fn id(&self) -> u32 {
        self.core.as_ref().unwrap().0
    }
}