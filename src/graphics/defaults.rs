use std::sync::RwLock;

use crate::{Res, unwrap_res};

use super::{shader::{SubShader, Shader, ShaderError, SubShaderType}, material::Material};

const DEF_PLAIN_VERT: &str = include_str!("../inline/def_plain_shader.vert");
const DEF_UV_VERT: &str = include_str!("../inline/def_uv_shader.vert");
const DEF_BLIT_VERT: &str = include_str!("../inline/def_blit_shader.vert");

const DEF_PLAIN_FRAG: &str = include_str!("../inline/def_plain_shader.frag");
const DEF_TEX_FRAG: &str = include_str!("../inline/def_tex_shader.frag");
const DEF_ELLIPSE_FRAG: &str = include_str!("../inline/def_ellipse_shader.frag");
const DEF_BLIT_FRAG: &str = include_str!("../inline/def_blit_shader.frag");

static SHADERS: RwLock<DefaultShaders> = RwLock::new(DefaultShaders::invalid());

pub struct DefaultShaders {
    def_plain_vert: SubShader,
    def_uv_vert: SubShader,
    def_blit_vert: SubShader,
    
    def_plain_frag: SubShader,
    def_tex_frag: SubShader,
    def_ellipse_frag: SubShader,
    def_blit_frag: SubShader,

    def_rect_shader: Shader,
    def_tex_shader: Shader,
    def_ellipse_shader: Shader,
    def_blit_shader: Shader,
}

impl DefaultShaders {
    const fn invalid() -> Self {
        return Self {
            def_plain_vert: SubShader::invalid(), def_uv_vert: SubShader::invalid(), def_blit_vert: SubShader::invalid(),
            def_plain_frag: SubShader::invalid(), def_tex_frag: SubShader::invalid(), def_ellipse_frag: SubShader::invalid(), def_blit_frag: SubShader::invalid(),
            def_rect_shader: Shader::invalid(), def_tex_shader: Shader::invalid(), def_ellipse_shader: Shader::invalid(), def_blit_shader: Shader::invalid() };
    }

    fn new() -> Res<Self, ShaderError> {
        let def_plain_vert = SubShader::new(&DEF_PLAIN_VERT, SubShaderType::Vert)?;
        let def_plain_frag = SubShader::new(&DEF_PLAIN_FRAG, SubShaderType::Frag)?;
        let def_uv_vert = SubShader::new(&DEF_UV_VERT, SubShaderType::Vert)?;
        let def_tex_frag = SubShader::new(&DEF_TEX_FRAG, SubShaderType::Frag)?;
        let def_ellipse_frag = SubShader::new(&DEF_ELLIPSE_FRAG, SubShaderType::Frag)?;
        let def_blit_vert = SubShader::new(&DEF_BLIT_VERT, SubShaderType::Vert)?;
        let def_blit_frag = SubShader::new(&DEF_BLIT_FRAG, SubShaderType::Frag)?;
        
        let def_rect_shader = Shader::new(&def_plain_vert, &def_plain_frag)?;
        let def_tex_shader = Shader::new(&def_uv_vert, &def_tex_frag)?;
        let def_ellipse_shader = Shader::new(&def_uv_vert, &def_ellipse_frag)?;
        let def_blit_shader = Shader::new(&def_blit_vert, &def_blit_frag)?;

        return Ok(Self { def_plain_vert, def_plain_frag, def_uv_vert, def_tex_frag, def_ellipse_frag, def_rect_shader, def_tex_shader, def_ellipse_shader, def_blit_vert, def_blit_frag, def_blit_shader });
    }

    pub(super) fn init() {
        *SHADERS.write().unwrap() = unwrap_res!(Self::new());
    }

    /// Vert subshader with `[xy, rgba]` layout.
    pub fn def_plain_vert() -> SubShader { SHADERS.read().unwrap().def_plain_vert.clone() }

    /// Frag subshader with `rgba` input. Output color is vertex color.
    pub fn def_plain_frag() -> SubShader { SHADERS.read().unwrap().def_plain_frag.clone() }

    /// Vert subshader with `[xy, rgba, uv]` layout.
    pub fn def_uv_vert() -> SubShader { SHADERS.read().unwrap().def_uv_vert.clone() }

    /// Frag subshader with `rgba` and `uv` input. Output color is texture.
    pub fn def_tex_frag() -> SubShader { SHADERS.read().unwrap().def_tex_frag.clone() }

    /// Frag subshader with `rgba` and `uv` input. Output color is an ellipse.
    pub fn def_ellipse_frag() -> SubShader { SHADERS.read().unwrap().def_ellipse_frag.clone() }

    /// Vert subshader with `[xy, uv]` layout.
    pub fn def_blit_vert() -> SubShader { SHADERS.read().unwrap().def_blit_vert.clone() }

    /// Frag subshader with `uv` input.
    pub fn def_blit_frag() -> SubShader { SHADERS.read().unwrap().def_blit_frag.clone() }

    /// Shader for rects and lines. `plain_vert` + `plain_frag`.
    pub fn def_rect_shader() -> Shader { SHADERS.read().unwrap().def_rect_shader.clone() }

    /// Shader for textured rects. `uv_vert` + `tex_frag`.
    pub fn def_tex_shader() -> Shader { SHADERS.read().unwrap().def_tex_shader.clone() }

    /// Shader for ellipses. `uv_vert` + `ellipse_frag`.
    pub fn def_ellipse_shader() -> Shader { SHADERS.read().unwrap().def_ellipse_shader.clone() }

    /// Shader for blit. `blit_vert` + `blit_frag`.
    pub fn def_blit_shader() -> Shader { SHADERS.read().unwrap().def_blit_shader.clone() }
}


static MATERIALS: RwLock<DefaultMaterials> = RwLock::new(DefaultMaterials::invalid());

pub struct DefaultMaterials {
    def_line_material: Material,
    def_rect_material: Material,
    def_tex_material: Material,
    def_ellipse_material: Material,
    def_blit_material: Material,
}

impl DefaultMaterials {
    const fn invalid() -> Self {
        return Self {
            def_rect_material: Material::invalid(), def_tex_material: Material::invalid(), def_ellipse_material: Material::invalid(), def_blit_material: Material::invalid(), def_line_material: Material::invalid(),
        };
    }

    pub(super) fn init() {
        *MATERIALS.write().unwrap() = Self::new();
    }

    fn new() -> Self{
        let shaders = SHADERS.read().unwrap();

        let def_line_material = Material::new(&shaders.def_rect_shader, &[]);
        let def_rect_material = Material::new(&shaders.def_rect_shader, &[]);
        let def_tex_material = Material::new(&shaders.def_tex_shader, &[]);
        let def_ellipse_material = Material::new(&shaders.def_ellipse_shader, &[]);
        let def_blit_material = Material::new(&shaders.def_blit_shader, &[]);

        return Self { def_rect_material, def_tex_material, def_ellipse_material, def_blit_material, def_line_material };
    }

    
    pub fn def_line_material() -> Material { MATERIALS.read().unwrap().def_line_material.clone() }
    pub fn def_rect_material() -> Material { MATERIALS.read().unwrap().def_rect_material.clone() }
    pub fn def_tex_material() -> Material { MATERIALS.read().unwrap().def_tex_material.clone() }
    pub fn def_ellipse_material() -> Material { MATERIALS.read().unwrap().def_ellipse_material.clone() }
    pub fn def_blit_material() -> Material { MATERIALS.read().unwrap().def_blit_material.clone() }
}