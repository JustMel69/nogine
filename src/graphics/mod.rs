use std::{sync::RwLock, f32::consts::PI};

use crate::{math::{Vector2, Matrix3x3, Rect}, color::{Color4, Color}};

use self::{shader::{Shader, SubShader, SubShaderType}, texture::{Texture, Sprite}, uniforms::Uniform, batch::{BatchMesh, BatchProduct}};

use self::batch::RefBatchState;

use super::gl_call;

pub mod shader;
pub mod texture;
pub mod uniforms;

mod buffers;
mod verts;
mod batch;


static GRAPHICS: RwLock<Graphics> = RwLock::new(Graphics::new());
static BATCH_DATA: RwLock<BatchData> = RwLock::new(BatchData::new());

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
    const fn invalid() -> Self {
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

    /// Vert subshader with `[xy, rgba]` layout.
    pub fn def_plain_vert() -> SubShader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_plain_vert.clone() }

    /// Frag subshader with `rgba` input. Output color is vertex color.
    pub fn def_plain_frag() -> SubShader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_plain_frag.clone() }

    /// Vert subshader with `[xy, rgba, uv]` layout.
    pub fn def_uv_vert() -> SubShader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_uv_vert.clone() }

    /// Frag subshader with `rgba` and `uv` input. Output color is texture.
    pub fn def_tex_frag() -> SubShader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_tex_frag.clone() }

    /// Frag subshader with `rgba` and `uv` input. Output color is an ellipse.
    pub fn def_ellipse_frag() -> SubShader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_ellipse_frag.clone() }

    /// Shader for rects. `plain_vert` + `plain_frag`.
    pub fn def_rect_shader() -> Shader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_rect_shader.clone() }

    /// Shader for textured rects. `uv_vert` + `tex_frag`.
    pub fn def_tex_shader() -> Shader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_tex_shader.clone() }

    /// Shader for ellipses. `uv_vert` + `ellipse_frag`.
    pub fn def_ellipse_shader() -> Shader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_ellipse_shader.clone() }
}

struct BatchData {
    curr_batch: Option<BatchMesh>,
    ready_batches: Vec<BatchProduct>,
    render_batches: Vec<BatchProduct>,
}

impl BatchData {
    const fn new() -> Self {
        return Self { curr_batch: None, ready_batches: Vec::new(), render_batches: Vec::new() };
    }

    fn send(&mut self, state: RefBatchState<'_>, verts: &[f32], tris: &[u32]) {
        self.check_state(&state);

        if self.curr_batch.is_none() {
            self.curr_batch = Some(BatchMesh::new(state.into()));
        }

        self.curr_batch.as_mut().unwrap().push(verts, tris);
    }

    fn check_state(&mut self, state: &RefBatchState<'_>) {
        if self.curr_batch.is_none() {
            return;
        }
        
        if !self.curr_batch.as_ref().unwrap().is_of_state(&state) {
            self.finalize_batch();
        }
    }

    fn finalize_batch(&mut self) {
        let mut batch: Option<BatchMesh> = None;
        std::mem::swap(&mut batch, &mut self.curr_batch);
        
        if let Some(x) = batch {
            let product = x.consume();
            self.ready_batches.push(product);
        }
    }

    fn swap_batch_buffers(&mut self) {
        std::mem::swap(&mut self.ready_batches, &mut self.render_batches);
        self.ready_batches.clear();
    }
}

/// Main struct for drawing.
pub struct Graphics {
    scheduled_cam_data: CamData,
    curr_cam_mat: Matrix3x3,
    
    pixels_per_unit: f32,

    default_shaders: DefaultShaders,

    rect_shader: Option<Shader>,
    tex_shader: Option<Shader>,
    ellipse_shader: Option<Shader>,
    custom_shader: Option<Shader>,

    blending: BlendingMode,
    uniforms: Vec<(Box<[u8]>, Uniform)>,
}

impl Graphics {
    const fn new() -> Self {
        return Self { scheduled_cam_data: DEFAULT_CAM_DATA, curr_cam_mat: Matrix3x3::IDENTITY, pixels_per_unit: 1.0, default_shaders: DefaultShaders::invalid(), rect_shader: None, tex_shader: None, ellipse_shader: None, custom_shader: None, blending: BlendingMode::AlphaMix, uniforms: Vec::new() };
    }

    pub(crate) fn init() {
        {
            let mut writer = GRAPHICS.write().unwrap();
            writer.default_shaders = DefaultShaders::new();
        }
        
        gl_call!(gl::Enable(gl::BLEND));
        Self::set_blending_mode(BlendingMode::AlphaMix);
    }

    pub(super) fn tick(aspect_ratio: f32) {
        // Update camera matrix
        let mut writer = GRAPHICS.write().unwrap();
        
        let cam_data = &writer.scheduled_cam_data;
        let size = Vector2(cam_data.height * aspect_ratio, cam_data.height);
        writer.curr_cam_mat = Matrix3x3::cam_matrix(cam_data.pos, size);
    }



    // |>-<   Rect Drawing   >-<| //
    
    /// Draws a non rotated rect.
    pub fn draw_rect(left_down: Vector2, extents: Vector2, color: Color4) {
        Self::draw_rect_full(left_down, extents, 0.0, [color; 4])
    }
    
    const RECT_TRIS: [u32; 6] = [0, 1, 2, 2, 3, 0];

    /// Draws a rotated rect with control over the color of every vert.<br> 
    /// - The rect is rotated around the left-down vertice.
    /// - The order of the colors for the colors array is<br>
    /// 1 2<br>
    /// 0 3
    pub fn draw_rect_full(left_down: Vector2, extents: Vector2, rot: f32, colors: [Color4; 4]) {
        #[repr(C)]
        struct Vert(Vector2, Color4);

        let tf_mat = Matrix3x3::transform_matrix(left_down, rot, extents);
        let vert_data = [Vert(&tf_mat * Vector2::ZERO, colors[0]), Vert(&tf_mat * Vector2::UP, colors[1]), Vert(&tf_mat * Vector2::ONE, colors[2]), Vert(&tf_mat * Vector2::RIGHT, colors[3])];

        let vert_data = convert_vert_data(&vert_data);

        let reader = GRAPHICS.read().unwrap();
        let state = reader.gen_ref_state(Mode::Rect, &[2, 4], &[]);
        
        BATCH_DATA.write().unwrap().send(state, vert_data, &Self::RECT_TRIS);
    }



    // |>-<   Texture Drawing   >-<| //

    /// Draws a rotated texture.<br>
    /// - The texture is rotated around the left-down vertice.
    /// - The size of the rect depends on the stablished pixels-per-unit and the scale.
    pub fn draw_texture(left_down: Vector2, scale: Vector2, rot: f32, tex: &Texture) {
        Self::draw_texture_full(left_down, scale, rot, Rect::IDENT, [Color4::WHITE; 4], tex)
    }

    /// Draws a rotated sprite.<br>
    /// - The sprite is rotated around the left-down vertice.
    /// - The size of the rect depends on the stablished pixels-per-unit and the scale.
    pub fn draw_sprite(left_down: Vector2, scale: Vector2, rot: f32, sprite: Sprite) {
        Self::draw_texture_full(left_down, scale, rot, sprite.rect(), [Color4::WHITE; 4], sprite.tex());
    }

    /// Draws a rotated texture with control over the color of each vert and the uv rect utilized.<br>
    /// - The texture is rotated around the left-down vertice.
    /// - The size of the rect depends on the stablished pixels-per-unit and the scale.
    /// - The order of the colors for the colors array is<br>
    /// 1 2<br>
    /// 0 3
    pub fn draw_texture_full(left_down: Vector2, scale: Vector2, rot: f32, uvs: Rect, colors: [Color4; 4], tex: &Texture) {
        #[repr(C)]
        struct Vert(Vector2, Color4, Vector2);

        let tex_res = tex.dims();
        let ppu = {
            let reader = GRAPHICS.read().unwrap();
            reader.pixels_per_unit
        };
        let extents = (Vector2(tex_res.0 as f32, tex_res.1 as f32) / ppu).scale(scale).scale(uvs.size());

        let tf_mat = Matrix3x3::transform_matrix(left_down, rot, extents);
        let vert_data = [Vert(&tf_mat * Vector2::ZERO, colors[0], uvs.lu()), Vert(&tf_mat * Vector2::UP, colors[1], uvs.ld()), Vert(&tf_mat * Vector2::ONE, colors[2], uvs.rd()), Vert(&tf_mat * Vector2::RIGHT, colors[3], uvs.ru())];

        let textures = &[tex];
        let vert_data = convert_vert_data(&vert_data);
        
        let reader = GRAPHICS.read().unwrap();
        let state = reader.gen_ref_state(Mode::Textured, &[2, 4, 2], textures);
        
        BATCH_DATA.write().unwrap().send(state, vert_data, &Self::RECT_TRIS);
    }


    // |>-<   Ellipse Drawing   >-<| //

    /// Draws a circle.
    pub fn draw_circle(center: Vector2, radius: f32, color: Color4) {
        Self::draw_ellipse(center, Vector2(radius, radius), 0.0, color);
    }

    /// Draws a rotated ellipse.
    /// - The ellipse is rotated around the center.
    pub fn draw_ellipse(center: Vector2, half_extents: Vector2, rot: f32, color: Color4) {
        #[repr(C)]
        struct Vert(Vector2, Color4, Vector2);

        let tf_mat = Matrix3x3::transform_matrix(center - half_extents, rot, half_extents * 2.0);
        let vert_data = [Vert(&tf_mat * Vector2::ZERO, color, Vector2::UP), Vert(&tf_mat * Vector2::UP, color, Vector2::ZERO), Vert(&tf_mat * Vector2::ONE, color, Vector2::RIGHT), Vert(&tf_mat * Vector2::RIGHT, color, Vector2::ONE)];

        let vert_data = convert_vert_data(&vert_data);

        let reader = GRAPHICS.read().unwrap();
        let state = reader.gen_ref_state(Mode::Ellipse, &[2, 4, 2], &[]);
        
        BATCH_DATA.write().unwrap().send(state, vert_data, &Self::RECT_TRIS);
    }



    // |>-<   N-sided polygon   >-<| //

    /// Draws a rotated polygon.
    /// - The polygon is rotated around the center.
    pub fn draw_polygon(center: Vector2, radius: f32, rot: f32, sides: u32, color: Color4) {
        Self::draw_polygon_ext(center, Vector2(radius, radius), rot, sides, color);
    }

    /// Draws an scaled and rotated polygon.
    /// - The polygon is rotated around the center.
    pub fn draw_polygon_ext(center: Vector2, half_extents: Vector2, rot: f32, sides: u32, color: Color4) {
        assert!(sides >= 3);

        #[repr(C)]
        struct Vert(Vector2, Color4);

        let delta_theta = (2.0 * PI) / (sides as f32);
        let mut verts = Vec::with_capacity(1 + sides as usize);

        let tf_mat = Matrix3x3::transform_matrix(center, rot, half_extents);

        verts.push(Vert(&tf_mat * Vector2::ZERO, color));
        for i in 0..sides {
            let theta = delta_theta * (i as f32);
            let pos = &tf_mat * Vector2::UP.rotate(theta);
            verts.push(Vert(pos, color));
        }
        let mut tris: Vec<u32> = Vec::with_capacity(sides as usize * 3);
        for i in 0..sides {
            let i = i + 1;
            let j = (i % sides) + 1;
            tris.extend_from_slice(&[0, i, j])
        }
        
        let vert_data = convert_vert_data(&verts);

        let reader = GRAPHICS.read().unwrap();
        let state = reader.gen_ref_state(Mode::Rect, &[2, 4], &[]);

        let mut b_writer = BATCH_DATA.write().unwrap();
        b_writer.send(state, vert_data, &tris);
    }


    /// Draw a custom mesh. Prone to not behaving.
    pub unsafe fn draw_custom_mesh(pos: Vector2, rot: f32, scale: Vector2, vert_data: &[f32], tri_data: &[u32], vert_attribs: &[usize], textures: &[&Texture]) {
        assert!(tri_data.len() % 3 == 0);

        let tf_mat = Matrix3x3::transform_matrix(pos, rot, scale);

        let stride = vert_attribs.iter().sum();
        let vert_data = vert_data.windows(2).step_by(stride).flat_map(|x| {
            let res = &tf_mat * Vector2(x[0], x[1]);
            [res.0, res.1].into_iter()
        }).collect::<Box<[_]>>();
        
        let reader = GRAPHICS.read().unwrap();
        let state = reader.gen_ref_state(Mode::Custom, vert_attribs, textures);

        let mut b_writer = BATCH_DATA.write().unwrap();
        b_writer.send(state, &vert_data, tri_data);
    }

    /// Sets a custom shader for a certain rendering mode.<br>
    /// - If `shader` is `None` the default shader will be restored for the given mode.
    /// - `mode` cannot be `Unset`
    pub fn set_shader(shader: Option<Shader>, mode: Mode) {
        assert!(!matches!(mode, Mode::Unset), "Mode cannot be unset!");

        let mut writer = GRAPHICS.write().unwrap();
        match mode {
            Mode::Unset => unreachable!(),
            Mode::Rect => writer.rect_shader = shader,
            Mode::Textured => writer.tex_shader = shader,
            Mode::Ellipse => writer.ellipse_shader = shader,
            Mode::Custom => writer.custom_shader = shader,
        }
    }

    /// Sets the value of a given uniform.<br>
    /// - `name` must be a zero terminated ascii string. Ex: `b"uniform_name\0"`
    pub fn set_uniform(name: &[u8], value: Uniform) {
        assert!(name.len() > 0 && name[name.len() - 1] == b'\0', "Uniform names must be zero terminated u8 slices");
        
        let mut writer = GRAPHICS.write().unwrap();
        if let Some(i) = writer.uniforms.iter().position(|x| x.0.iter().eq(name.iter())) {
            writer.uniforms[i].1 = value;
        } else {
            writer.uniforms.push((name.into(), value));
        }
    }

    /// Sets the current blending mode.
    pub fn set_blending_mode(mode: BlendingMode) {
        let mut writer = GRAPHICS.write().unwrap();
        writer.blending = mode;
    }

    /// Sets the current pixels per unit.
    /// - `ppu` must be positive.
    pub fn set_pixels_per_unit(ppu: f32) {
        assert!(ppu > 0.0, "Pixels per unit must be positive!");

        let mut writer = GRAPHICS.write().unwrap();
        writer.pixels_per_unit = ppu;
    }

    /// Sets the camera parameters.
    /// - 'height' must not be zero.
    /// - Changes will be applied the next frame.
    pub fn set_cam(pos: Vector2, height: f32) {
        assert!(height != 0.0);

        let mut writer = GRAPHICS.write().unwrap();
        writer.scheduled_cam_data = CamData { pos, height };
    }

    /// Returns the camera matrix from the current camera config.
    /// - This matrix will not change when `set_cam` is called until the next frame.
    pub fn get_cam_matrix() -> Matrix3x3 {
        let reader = GRAPHICS.read().unwrap();
        return reader.curr_cam_mat.clone();
    }

    fn get_shader(mode: Mode) -> Option<Shader> {
        let reader = GRAPHICS.read().unwrap();
        match mode {
            Mode::Unset => None,
            Mode::Rect => Some(reader.rect_shader.clone().unwrap_or(reader.default_shaders.def_rect_shader.clone())),
            Mode::Textured => Some(reader.tex_shader.clone().unwrap_or(reader.default_shaders.def_tex_shader.clone())),
            Mode::Ellipse => Some(reader.ellipse_shader.clone().unwrap_or(reader.default_shaders.def_ellipse_shader.clone())),
            Mode::Custom => reader.custom_shader.clone(),
        }
    }

    pub(crate) fn render() -> RenderStats {
        let reader = GRAPHICS.read().unwrap();
        let b_reader = BATCH_DATA.read().unwrap();

        for b in &b_reader.render_batches {
            b.render(reader.curr_cam_mat.clone());
        }

        return RenderStats {
            draw_calls: b_reader.render_batches.len(),
        };
    }

    pub(crate) fn finalize_batch() {
        let mut b_writer = BATCH_DATA.write().unwrap();
        b_writer.finalize_batch();
        b_writer.swap_batch_buffers();
    }

    fn get_blending(&self) -> BlendingMode {
        return self.blending;
    }

    fn get_uniforms(&self) -> &[(Box<[u8]>, Uniform)] {
        return &self.uniforms
    }

    fn gen_ref_state<'a>(&'a self, mode: Mode, attribs: &'a [usize], textures: &'a [&'a Texture]) -> RefBatchState<'a> {
        return RefBatchState {
            shader: Self::get_shader(mode).unwrap(),
            uniforms: self.get_uniforms(),
            attribs,
            textures,
            blending: self.get_blending(),
        };
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlendingMode {
    /// Default blending mode, usual behavior with transparency.
    AlphaMix,
    /// Color values are added to the background.
    Additive,
    /// Color values are multiplied with those of the background.
    Multiplicative,
}

fn convert_vert_data<T>(src: &[T]) -> &[f32] {
    let mul = std::mem::size_of::<T>() / std::mem::size_of::<f32>();
    return unsafe { std::slice::from_raw_parts(src.as_ptr() as *const f32, src.len() * mul) };
}



#[derive(Debug)]
pub struct RenderStats {
    draw_calls: usize,
}

impl RenderStats {
    /// Returns the number of total draw calls in a given frame.
    pub fn draw_calls(&self) -> usize {
        self.draw_calls
    }
}