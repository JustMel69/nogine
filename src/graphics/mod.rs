use std::{sync::RwLock, f32::consts::PI, ffi::CString};

use crate::{math::{Vector2, Matrix3x3}, color::{Color4, Color}, graphics::{buffers::{GlBuffer, GlVAO}, verts::set_vertex_attribs}};

use self::{shader::{Shader, SubShader, SubShaderType}, texture::Texture, uniforms::Uniform};

use super::gl_call;

pub mod shader;
pub mod buffers;
pub mod verts;
pub mod texture;
pub mod uniforms;


static GRAPHICS: RwLock<Graphics> = RwLock::new(Graphics::new());

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Unset,
    Rect,
    Textured,
    Ellipse,
    Custom,
}

impl Mode {
    pub fn matches(&self, other: &Self) -> bool {
        if matches!(self, Mode::Custom) || matches!(other, Mode::Custom) {
            return false;
        }

        return self == other;
    }
}


const DEF_PLAIN_VERT: &str = include_str!("../inline/def_plain_shader.vert");
const DEF_PLAIN_FRAG: &str = include_str!("../inline/def_plain_shader.frag");

const DEF_UV_VERT: &str = include_str!("../inline/def_uv_shader.vert");
const DEF_TEX_FRAG: &str = include_str!("../inline/def_tex_shader.frag");

const DEF_ELLIPSE_FRAG: &str = include_str!("../inline/def_ellipse_shader.frag");

const DEFAULT_CAM_DATA: CamData = CamData { pos: Vector2::ZERO, height: 1.0 };
struct CamData {
    pos: Vector2,
    height: f32,
}

pub struct DefaultShaders {
    def_plain_vert: SubShader,
    def_plain_frag: SubShader,
    def_uv_vert: SubShader,
    def_tex_frag: SubShader,
    def_ellipse_frag: SubShader,

    def_rect_shader: Shader,
    def_tex_shader: Shader,
    def_ellipse_shader: Shader,
}

impl DefaultShaders {
    pub const fn invalid() -> Self {
        return Self { def_plain_vert: SubShader::invalid(), def_plain_frag: SubShader::invalid(), def_uv_vert: SubShader::invalid(), def_tex_frag: SubShader::invalid(), def_ellipse_frag: SubShader::invalid(), def_rect_shader: Shader::invalid(), def_tex_shader: Shader::invalid(), def_ellipse_shader: Shader::invalid() };
    }

    fn new() -> Self {
        let def_plain_vert = SubShader::new(&DEF_PLAIN_VERT, SubShaderType::Vert);
        let def_plain_frag = SubShader::new(&DEF_PLAIN_FRAG, SubShaderType::Frag);
        let def_uv_vert = SubShader::new(&DEF_UV_VERT, SubShaderType::Vert);
        let def_tex_frag = SubShader::new(&DEF_TEX_FRAG, SubShaderType::Frag);
        let def_ellipse_frag = SubShader::new(&DEF_ELLIPSE_FRAG, SubShaderType::Frag);
        
        let def_rect_shader = Shader::new(&def_plain_vert, &def_plain_frag);
        let def_tex_shader = Shader::new(&def_uv_vert, &def_tex_frag);
        let def_ellipse_shader = Shader::new(&def_uv_vert, &def_ellipse_frag);

        return Self { def_plain_vert, def_plain_frag, def_uv_vert, def_tex_frag, def_ellipse_frag, def_rect_shader, def_tex_shader, def_ellipse_shader };
    }

    pub fn def_plain_vert() -> SubShader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_plain_vert.clone() }
    pub fn def_plain_frag() -> SubShader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_plain_frag.clone() }
    pub fn def_uv_vert() -> SubShader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_uv_vert.clone() }
    pub fn def_tex_frag() -> SubShader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_tex_frag.clone() }
    pub fn def_ellipse_frag() -> SubShader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_ellipse_frag.clone() }
    pub fn def_rect_shader() -> Shader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_rect_shader.clone() }
    pub fn def_tex_shader() -> Shader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_tex_shader.clone() }
    pub fn def_ellipse_shader() -> Shader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_ellipse_shader.clone() }
}


pub struct Graphics {
    mode: Mode,
    scheduled_cam_data: CamData,
    curr_cam_mat: Matrix3x3,
    
    pixels_per_unit: f32,

    default_shaders: DefaultShaders,

    rect_shader: Option<Shader>,
    tex_shader: Option<Shader>,
    ellipse_shader: Option<Shader>,
    custom_shader: Option<Shader>,
}

impl Graphics {
    const fn new() -> Self {
        return Self { mode: Mode::Unset, scheduled_cam_data: DEFAULT_CAM_DATA, curr_cam_mat: Matrix3x3::IDENTITY, pixels_per_unit: 1.0, default_shaders: DefaultShaders::invalid(), rect_shader: None, tex_shader: None, ellipse_shader: None, custom_shader: None };
    }

    pub(crate) fn init() {
        let mut writer = GRAPHICS.write().unwrap();
        writer.default_shaders = DefaultShaders::new();

        gl_call!(gl::Enable(gl::BLEND));
        Self::set_blending_mode(BlendingMode::AlphaMix);
    }

    pub fn tick(aspect_ratio: f32) {
        // Update camera matrix
        let mut writer = GRAPHICS.write().unwrap();
        
        let cam_data = &writer.scheduled_cam_data;
        let size = Vector2(cam_data.height * aspect_ratio, cam_data.height);
        //println!("{size:?}");
        writer.curr_cam_mat = Matrix3x3::cam_matrix(cam_data.pos, size);
    }



    // |>-<   Rect Drawing   >-<| //
    
    pub fn draw_rect(left_down: Vector2, extents: Vector2, color: Color4) {
        Self::draw_rect_full(left_down, extents, 0.0, [color; 4])
    }
    
    const RECT_TRIS: [u16; 6] = [0, 1, 2, 2, 3, 0];
    pub fn draw_rect_full(left_down: Vector2, extents: Vector2, rot: f32, colors: [Color4; 4]) {
        #[repr(C)]
        struct Vert(Vector2, Color4);
    
        Self::change_mode(Mode::Rect);

        let vert_data = [Vert(Vector2::ZERO, colors[0]), Vert(Vector2::UP, colors[1]), Vert(Vector2::ONE, colors[2]), Vert(Vector2::RIGHT, colors[3])];

        Self::draw_rect_internal(left_down, extents, rot, &vert_data, &[2, 4]);
    }



    // |>-<   Texture Drawing   >-<| //

    pub fn draw_texture(left_down: Vector2, scale: Vector2, rot: f32, tex: &Texture) {
        Self::draw_texture_full(left_down, scale, rot, [Color4::WHITE; 4], tex)
    }

    pub fn draw_texture_full(left_down: Vector2, scale: Vector2, rot: f32, colors: [Color4; 4], tex: &Texture) {
        #[repr(C)]
        struct Vert(Vector2, Color4, Vector2);

        Self::change_mode(Mode::Textured);

        let vert_data = [Vert(Vector2::ZERO, colors[0], Vector2::UP), Vert(Vector2::UP, colors[1], Vector2::ZERO), Vert(Vector2::ONE, colors[2], Vector2::RIGHT), Vert(Vector2::RIGHT, colors[3], Vector2::ONE)];

        tex.enable(0); // main_texture is always 0

        let tex_res = tex.dims();
        let ppu = {
            let reader = GRAPHICS.read().unwrap();
            reader.pixels_per_unit
        };
        let extents = (Vector2(tex_res.0 as f32, tex_res.1 as f32) / ppu).scale(scale);

        Self::draw_rect_internal(left_down, extents, rot, &vert_data, &[2, 4, 2]);
    }
    
    fn draw_rect_internal<T>(left_down: Vector2, extents: Vector2, rot: f32, vert_data: &[T], attribs: &[usize]) {
        let vao = GlVAO::new();
        vao.bind();
        
        let vbo = GlBuffer::new(gl::ARRAY_BUFFER);
        vbo.set_data(vert_data);
        
        let ebo = GlBuffer::new(gl::ELEMENT_ARRAY_BUFFER);
        ebo.set_data(&Self::RECT_TRIS);
        
        set_vertex_attribs(attribs);

        Self::set_tf_mat(Matrix3x3::transform_matrix(left_down, rot, extents));
        vao.bind();
        gl_call!(gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_SHORT, std::ptr::null()));
    }



    // |>-<   Ellipse Drawing   >-<| //

    pub fn draw_circle(center: Vector2, radius: f32, color: Color4) {
        Self::draw_ellipse(center, Vector2(radius, radius), 0.0, color);
    }

    pub fn draw_ellipse(center: Vector2, half_extents: Vector2, rot: f32, color: Color4) {
        #[repr(C)]
        struct Vert(Vector2, Color4, Vector2);

        Self::change_mode(Mode::Ellipse);

        let vert_data = [Vert(Vector2::ZERO, color, Vector2::UP), Vert(Vector2::UP, color, Vector2::ZERO), Vert(Vector2::ONE, color, Vector2::RIGHT), Vert(Vector2::RIGHT, color, Vector2::ONE)];

        Self::draw_rect_internal(center - half_extents, half_extents * 2.0, rot, &vert_data, &[2, 4, 2]);
    }



    // |>-<   N-sided polygon   >-<| //

    pub fn draw_polygon(center: Vector2, radius: f32, rot: f32, sides: u16, color: Color4) {
        Self::draw_polygon_ext(center, Vector2(radius, radius), rot, sides, color);
    }

    pub fn draw_polygon_ext(center: Vector2, half_extents: Vector2, rot: f32, sides: u16, color: Color4) {
        assert!(sides >= 3);

        #[repr(C)]
        struct Vert(Vector2, Color4);

        Self::change_mode(Mode::Rect);

        let delta_theta = (2.0 * PI) / (sides as f32);
        let mut verts = Vec::with_capacity(1 + sides as usize);

        verts.push(Vert(Vector2::ZERO, color));
        for i in 0..sides {
            let theta = delta_theta * (i as f32);
            let pos = Vector2::UP.rotate(theta);
            verts.push(Vert(pos, color));
        }
        let mut tris: Vec<u16> = Vec::with_capacity(sides as usize * 3);
        for i in 0..sides {
            let i = i + 1;
            let j = (i % sides) + 1;
            tris.extend_from_slice(&[0, i, j])
        }

        let vao = GlVAO::new();
        vao.bind();
        
        let vbo = GlBuffer::new(gl::ARRAY_BUFFER);
        vbo.set_data(verts.as_slice());
        
        let ebo = GlBuffer::new(gl::ELEMENT_ARRAY_BUFFER);
        ebo.set_data(tris.as_slice());
        
        set_vertex_attribs(&[2, 4]);

        Self::set_tf_mat(Matrix3x3::transform_matrix(center, rot, half_extents));
        vao.bind();
        gl_call!(gl::DrawElements(gl::TRIANGLES, tris.len() as i32, gl::UNSIGNED_SHORT, std::ptr::null()));
    }



    pub fn draw_custom_mesh<T>(pos: Vector2, rot: f32, scale: Vector2, vert_data: &[T], tri_data: &[u16], vert_attribs: &[usize]) {
        assert!(tri_data.len() % 3 == 0);

        Self::change_mode(Mode::Custom);

        let vao = GlVAO::new();
        vao.bind();
        
        let vbo = GlBuffer::new(gl::ARRAY_BUFFER);
        vbo.set_data(vert_data);
        
        let ebo = GlBuffer::new(gl::ELEMENT_ARRAY_BUFFER);
        ebo.set_data(tri_data);
        
        set_vertex_attribs(vert_attribs);

        Self::set_tf_mat(Matrix3x3::transform_matrix(pos, rot, scale));
        vao.bind();
        gl_call!(gl::DrawElements(gl::TRIANGLES, tri_data.len() as i32, gl::UNSIGNED_SHORT, std::ptr::null()));
    }

    pub fn set_shader(shader: Option<Shader>, mode: Mode) {
        assert!(!matches!(mode, Mode::Unset));

        Self::change_mode(mode);
        let mut writer = GRAPHICS.write().unwrap();
        match mode {
            Mode::Unset => unreachable!(),
            Mode::Rect => writer.rect_shader = shader,
            Mode::Textured => writer.tex_shader = shader,
            Mode::Ellipse => writer.ellipse_shader = shader,
            Mode::Custom => writer.custom_shader = shader,
        }
    }

    pub fn set_uniform(name: &str, value: Uniform) {
        let name = CString::new(name).unwrap();
        
        let location = {
            let reader = GRAPHICS.read().unwrap();
            let shader = reader.get_current_shader().unwrap();
            shader.enable();
            gl_call!(gl::GetUniformLocation(shader.id(), name.as_ptr()))
        };

        match value {
            Uniform::Float(x) => gl_call!(gl::Uniform1f(location, x)),
            Uniform::Float2(x, y) => gl_call!(gl::Uniform2f(location, x, y)),
            Uniform::Float3(x, y, z) => gl_call!(gl::Uniform3f(location, x, y, z)),
            Uniform::Float4(x, y, z, w) => gl_call!(gl::Uniform4f(location, x, y, z ,w)),
            Uniform::Int(x) => gl_call!(gl::Uniform1i(location, x)),
            Uniform::Uint(x) => gl_call!(gl::Uniform1ui(location, x)),
        }
    }

    pub fn set_blending_mode(mode: BlendingMode) {
        match mode {
            BlendingMode::AlphaMix => gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)),
            BlendingMode::Additive => gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE)),
            BlendingMode::Multiplicative => gl_call!(gl::BlendFunc(gl::DST_COLOR, gl::ZERO)),
        }
    }

    pub fn set_pixels_per_unit(ppu: f32) {
        assert!(ppu > 0.0);

        let mut writer = GRAPHICS.write().unwrap();
        writer.pixels_per_unit = ppu;
    }

    pub fn set_cam(pos: Vector2, height: f32) {
        assert!(height != 0.0);

        let mut writer = GRAPHICS.write().unwrap();
        writer.scheduled_cam_data = CamData { pos, height };
    }

    fn change_mode(mode: Mode) {
        let mut writer = GRAPHICS.write().unwrap();
        if writer.mode.matches(&mode) {
            return;
        }

        set_vertex_attribs(&[2, 4]);
        writer.mode = mode;
        writer.get_current_shader().unwrap().enable();
    }

    const MVM_NAME: [u8; 4] = [b'm', b'v', b'm', 0];
    fn set_tf_mat(mat: Matrix3x3) {
        let reader = GRAPHICS.read().unwrap();
        let shader = match reader.get_current_shader() {
            Some(x) => x,
            None => return,
        };

        let tf_mat_address = gl_call!(gl::GetUniformLocation(shader.id(), Self::MVM_NAME.as_ptr() as *const i8));
        assert!(tf_mat_address != -1);

        let mvm = &reader.curr_cam_mat * &mat;

        shader.enable(); // must enable for the next gl_call to not fucking scream and die
        gl_call!(gl::UniformMatrix3fv(tf_mat_address, 1, gl::TRUE, mvm.ptr()))
    }

    fn get_current_shader(&self) -> Option<&Shader> {
        match self.mode {
            Mode::Unset => None,
            Mode::Rect => Some(self.rect_shader.as_ref().unwrap_or(&self.default_shaders.def_rect_shader)),
            Mode::Textured => Some(self.tex_shader.as_ref().unwrap_or(&self.default_shaders.def_tex_shader)),
            Mode::Ellipse => Some(self.ellipse_shader.as_ref().unwrap_or(&self.default_shaders.def_ellipse_shader)),
            Mode::Custom => self.custom_shader.as_ref(),
        }
    }
}

pub enum BlendingMode {
    AlphaMix,
    Additive,
    Multiplicative,
}