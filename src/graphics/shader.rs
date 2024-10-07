use std::{fmt::Display, ffi::{CString, NulError}, sync::Arc};

use thiserror::Error;

use crate::{Res, assert_expr};

use super::{gl_bindings::{program::GlProgram, shader::{GlShader, GlShaderType}}, DefaultShaders};

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
    Vert = GlShaderType::Vert as u32,
    Frag = GlShaderType::Frag as u32,
}

impl Display for SubShaderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubShaderType::Vert => write!(f, "Vertex"),
            SubShaderType::Frag => write!(f, "Fragment"),
        }
    }
}

/// An individual component of the shading pipeline.
#[derive(Clone)]
pub struct SubShader {
    core: Option<Arc<GlShader>>,
    kind: SubShaderType,
}

impl SubShader {
    pub(super) const fn invalid() -> Self {
        Self { core: None, kind: SubShaderType::Vert }
    }
    
    /// Compiles the sub shader.
    pub fn new(src: &str, kind: SubShaderType) -> Res<Self, ShaderError> {
        let core = GlShader::new(src, kind.into())?;
        return Ok(Self { core: Some(Arc::new(core)), kind })
    }

    fn gl_shader(&self) -> &GlShader {
        self.core.as_ref().unwrap()
    }
}


/// The main component of the shading pipeline. It defines how geometry must be drawn.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shader {
    core: Option<Arc<GlProgram>>
}

impl Shader {
    pub(super) const fn invalid() -> Self {
        Self { core: None }
    }

    /// Creates a shader from two subshaders.
    pub fn new(vert: &SubShader, frag: &SubShader) -> Res<Self, ShaderError> {
        assert_expr!(matches!(vert.kind, SubShaderType::Vert), "The vertex shader must be a vertex shader.");
        assert_expr!(matches!(frag.kind, SubShaderType::Frag), "The fragment shader must be a fragment shader.");

        let core = GlProgram::new(vert.gl_shader(), frag.gl_shader())?;
        return Ok(Self { core: Some(Arc::new(core)) });
    }

    /// Creates a blit shader from a src.
    pub fn new_blit(src: &str) -> Res<Self, ShaderError> {
        let frag = SubShader::new(src, SubShaderType::Frag)?;
        return Self::new(&DefaultShaders::def_blit_vert(), &frag);
    }

    pub(super) fn enable(&self) {
        self.gl_program().enable();
    }

    fn gl_program(&self) -> &GlProgram {
        self.core.as_ref().unwrap()
    }
}
