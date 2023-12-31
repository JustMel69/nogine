use std::{fmt::Display, ffi::{CString, NulError}, sync::Arc};

use thiserror::Error;

use crate::{Res, assert_expr};

use super::{super::gl_call, DefaultShaders};

#[derive(Debug, Error)]
pub enum ShaderError {
    #[error("{0}")]
    NulError(#[from] NulError),
    #[error("{kind} Shader Compilation Error:\n{msg}")]
    CompilationError { kind: SubShaderType, msg: String },
    #[error("Shader Linking Error:\n{msg}")]
    LinkingError { msg: String },
}

#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum SubShaderType {
    Vert = gl::VERTEX_SHADER,
    Frag = gl::FRAGMENT_SHADER,
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

/// An individual component of the shading pipeline.
#[derive(Clone)]
pub struct SubShader {
    core: Option<Arc<SubShaderCore>>,
    kind: SubShaderType,
}

impl SubShader {
    pub(super) const fn invalid() -> Self {
        Self { core: None, kind: SubShaderType::Vert }
    }
    
    /// Compiles the sub shader.
    pub fn new(src: &str, kind: SubShaderType) -> Res<Self, ShaderError> {
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

        return Ok(Self { core: Some(Arc::new(SubShaderCore(id))), kind })
    }

    fn id(&self) -> u32 {
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



/// The main component of the shading pipeline. It defines how geometry must be drawn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shader {
    core: Option<Arc<ShaderCore>>
}

impl Shader {
    pub(super) const fn invalid() -> Self {
        Self { core: None }
    }

    /// Creates a shader from two subshaders.
    pub fn new(vert: &SubShader, frag: &SubShader) -> Res<Self, ShaderError> {
        assert_expr!(matches!(vert.kind, SubShaderType::Vert), "The vertex shader must be a vertex shader.");
        assert_expr!(matches!(frag.kind, SubShaderType::Frag), "The fragment shader must be a fragment shader.");

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

        return Ok(Self { core: Some(Arc::new(ShaderCore(id))) });
    }

    /// Creates a blit shader from a src.
    pub fn new_blit(src: &str) -> Res<Self, ShaderError> {
        let frag = SubShader::new(src, SubShaderType::Frag)?;
        return Self::new(&DefaultShaders::def_blit_vert(), &frag);
    }

    pub(super) fn enable(&self) {
        gl_call!(gl::UseProgram(self.id()));
    }

    pub(super) fn id(&self) -> u32 {
        self.core.as_ref().unwrap().0
    }
}