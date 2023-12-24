use std::{sync::RwLock, f32::consts::PI};

use crate::{math::{Vector2, Matrix3x3, Rect}, color::{Color4, Color}, Res, log_info, unwrap_res, window::Window, assert_expr};

use self::{shader::{Shader, SubShader, SubShaderType, ShaderError}, texture::{Texture, Sprite}, batch::{BatchMesh, BatchProduct}, pipeline::{RenderPipeline, SceneRenderData, RenderTexture, DEFAULT_RENDER_TARGET}, material::Material};

use self::batch::RefBatchState;

use super::gl_call;

pub mod material;
pub mod shader;
pub mod texture;
pub mod uniforms;
pub mod pipeline;
pub mod gfx;

mod buffers;
mod verts;
mod batch;


static GRAPHICS: RwLock<Graphics> = RwLock::new(Graphics::new());
static BATCH_DATA: RwLock<Option<BatchData>> = RwLock::new(None);

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

const DEF_BLIT_VERT: &str = include_str!("../inline/def_blit_shader.vert");
const DEF_BLIT_FRAG: &str = include_str!("../inline/def_blit_shader.frag");

const DEFAULT_CAM_DATA: CamData = CamData { pos: Vector2::ZERO, size: Vector2::ONE };
struct CamData {
    pos: Vector2,
    size: Vector2,
}

pub struct DefaultShaders {
    def_plain_vert: SubShader,
    def_plain_frag: SubShader,
    def_uv_vert: SubShader,
    def_tex_frag: SubShader,
    def_ellipse_frag: SubShader,
    def_blit_vert: SubShader,
    def_blit_frag: SubShader,

    def_rect_shader: Shader,
    def_tex_shader: Shader,
    def_ellipse_shader: Shader,
    def_blit_shader: Shader,
}

impl DefaultShaders {
    const fn invalid() -> Self {
        return Self { def_plain_vert: SubShader::invalid(), def_plain_frag: SubShader::invalid(), def_uv_vert: SubShader::invalid(), def_tex_frag: SubShader::invalid(), def_ellipse_frag: SubShader::invalid(), def_rect_shader: Shader::invalid(), def_tex_shader: Shader::invalid(), def_ellipse_shader: Shader::invalid(), def_blit_vert: SubShader::invalid(), def_blit_frag: SubShader::invalid(), def_blit_shader: Shader::invalid() };
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

    /// Vert subshader with `[xy, uv]` layout.
    pub fn def_blit_vert() -> SubShader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_blit_vert.clone() }

    /// Frag subshader with `uv` input.
    pub fn def_blit_frag() -> SubShader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_blit_frag.clone() }

    /// Shader for rects. `plain_vert` + `plain_frag`.
    pub fn def_rect_shader() -> Shader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_rect_shader.clone() }

    /// Shader for textured rects. `uv_vert` + `tex_frag`.
    pub fn def_tex_shader() -> Shader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_tex_shader.clone() }

    /// Shader for ellipses. `uv_vert` + `ellipse_frag`.
    pub fn def_ellipse_shader() -> Shader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_ellipse_shader.clone() }

    /// Shader for blit. `blit_vert` + `blit_frag`.
    pub fn def_blit_shader() -> Shader { let reader = GRAPHICS.read().unwrap(); reader.default_shaders.def_blit_shader.clone() }
}

pub struct DefaultMaterials {
    def_rect_material: Material,
    def_tex_material: Material,
    def_ellipse_material: Material,
    def_blit_material: Material,
}

impl DefaultMaterials {
    const fn invalid() -> Self {
        return Self {
            def_rect_material: Material::invalid(), def_tex_material: Material::invalid(), def_ellipse_material: Material::invalid(), def_blit_material: Material::invalid()
        };
    }

    fn new(default_shaders: &DefaultShaders) -> Self{
        let def_rect_material = Material::new(&default_shaders.def_rect_shader, &[]);
        let def_tex_material = Material::new(&default_shaders.def_tex_shader, &[]);
        let def_ellipse_material = Material::new(&default_shaders.def_ellipse_shader, &[]);
        let def_blit_material = Material::new(&default_shaders.def_blit_shader, &[]);

        return Self { def_rect_material, def_tex_material, def_ellipse_material, def_blit_material };
    }

    
    pub fn def_rect_material() -> Material { let reader = GRAPHICS.read().unwrap(); reader.default_materials.def_rect_material.clone() }
    pub fn def_tex_material() -> Material { let reader = GRAPHICS.read().unwrap(); reader.default_materials.def_tex_material.clone() }
    pub fn def_ellipse_material() -> Material { let reader = GRAPHICS.read().unwrap(); reader.default_materials.def_ellipse_material.clone() }
    pub fn def_blit_material() -> Material { let reader = GRAPHICS.read().unwrap(); reader.default_materials.def_blit_material.clone() }
}


struct TargetBatchData {
    curr_batch: Option<BatchMesh>,
    ready_batches: Vec<BatchProduct>,
    render_batches: Vec<BatchProduct>,
}

impl TargetBatchData {
    const fn new() -> Self {
        return Self { curr_batch: None, ready_batches: Vec::new(), render_batches: Vec::new() }
    }
}

struct BatchData {
    targets: [Option<TargetBatchData>; 256],
}

impl BatchData {
    fn new() -> Self {
        return Self {
            targets: std::array::from_fn(|_| None),
        }
    }

    fn send(&mut self, target_id: u8, state: RefBatchState<'_>, verts: &[f32], tris: &[u32]) {
        self.check_state(target_id, &state);

        let target = self.realize_target(target_id);
        
        if target.curr_batch.is_none() {
            target.curr_batch = Some(BatchMesh::new(state.into()));
        }

        target.curr_batch.as_mut().unwrap().push(verts, tris);

    }

    fn check_state(&mut self, target_id: u8, state: &RefBatchState<'_>) {
        if let Some(target) = &mut self.targets[target_id as usize] {
            if target.curr_batch.is_none() {
                return;
            }
            
            if !target.curr_batch.as_ref().unwrap().is_of_state(&state) {
                self.finalize_batch(target_id);
            }
        }
    }

    fn finalize_batch(&mut self, target_id: u8) {
        if let Some(target) = &mut self.targets[target_id as usize] {    
            let mut batch: Option<BatchMesh> = None;
            std::mem::swap(&mut batch, &mut target.curr_batch);
            
            if let Some(x) = batch {
                let product = x.consume();
                target.ready_batches.push(product);
            }
        }
    }

    fn swap_batch_buffers(&mut self, target_id: u8) {
        if let Some(target) = &mut self.targets[target_id as usize] {
            std::mem::swap(&mut target.ready_batches, &mut target.render_batches);
            target.ready_batches.clear();
        }
    }

    fn realize_target(&mut self, target: u8) -> &mut TargetBatchData {
        if self.targets[target as usize].is_none() {
            self.targets[target as usize] = Some(TargetBatchData::new());
        }

        return self.targets[target as usize].as_mut().unwrap();
    }
}

/// Main struct for drawing.
pub struct Graphics {
    scheduled_cam_data: CamData,
    curr_cam_height: f32,
    curr_cam_mat: Matrix3x3,
    
    pixels_per_unit: f32,

    default_shaders: DefaultShaders,
    default_materials: DefaultMaterials,

    rect_material: Option<Material>,
    tex_material: Option<Material>,
    ellipse_material: Option<Material>,
    custom_material: Option<Material>,

    render_target: u8,
    clear_col: Color4,
    blending: BlendingMode,
}

impl Graphics {
    const fn new() -> Self {
        return Self { scheduled_cam_data: DEFAULT_CAM_DATA, curr_cam_mat: Matrix3x3::IDENTITY, pixels_per_unit: 1.0, default_shaders: DefaultShaders::invalid(), rect_material: None, tex_material: None, ellipse_material: None, custom_material: None, blending: BlendingMode::AlphaMix, clear_col: Color4::BLACK, curr_cam_height: 1.0, default_materials: DefaultMaterials::invalid(), render_target: DEFAULT_RENDER_TARGET };
    }

    pub(crate) fn init() {
        {
            let mut writer = GRAPHICS.write().unwrap();
            writer.default_shaders = unwrap_res!(DefaultShaders::new());
            writer.default_materials = DefaultMaterials::new(&writer.default_shaders);
        }

        {
            let mut writer = BATCH_DATA.write().unwrap();
            *writer = Some(BatchData::new());
        }
        
        gl_call!(gl::Enable(gl::BLEND));
        Self::set_blending_mode(BlendingMode::AlphaMix);

        log_info!("Graphics initialized.");
    }

    pub(super) fn tick() {
        // Update camera matrix
        let mut writer = GRAPHICS.write().unwrap();
        
        let cam_data = &writer.scheduled_cam_data;
        let size = Vector2(cam_data.size.0, cam_data.size.1);
        writer.curr_cam_mat = Matrix3x3::cam_matrix(cam_data.pos, size);
        writer.curr_cam_height = size.1;
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
        let target = reader.render_target;

        BATCH_DATA.write().as_mut().unwrap().as_mut().unwrap().send(target, state, vert_data, &Self::RECT_TRIS);
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
        let target = reader.render_target;
        
        BATCH_DATA.write().unwrap().as_mut().unwrap().send(target, state, vert_data, &Self::RECT_TRIS);
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
        let target = reader.render_target;
        
        BATCH_DATA.write().unwrap().as_mut().unwrap().send(target, state, vert_data, &Self::RECT_TRIS);
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
        assert_expr!(sides >= 3, "Every polygon must have at least 3 sides.");

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
        let target = reader.render_target;

        {
            let mut b_writer = BATCH_DATA.write();
            let b_writer = b_writer.as_mut().unwrap().as_mut().unwrap();

            b_writer.send(target, state, vert_data, &tris);
        }
    }


    /// Draw a custom mesh. Prone to not behaving.
    pub unsafe fn draw_custom_mesh(pos: Vector2, rot: f32, scale: Vector2, vert_data: &[f32], tri_data: &[u32], vert_attribs: &[usize], textures: &[&Texture]) {
        assert_expr!(tri_data.len() % 3 == 0, "The number of indices must be a multiple of 3.");

        let tf_mat = Matrix3x3::transform_matrix(pos, rot, scale);

        let stride = vert_attribs.iter().sum();
        let vert_data = vert_data.windows(2).step_by(stride).flat_map(|x| {
            let res = &tf_mat * Vector2(x[0], x[1]);
            [res.0, res.1].into_iter()
        }).collect::<Box<[_]>>();
        
        let reader = GRAPHICS.read().unwrap();
        let state = reader.gen_ref_state(Mode::Custom, vert_attribs, textures);
        let target = reader.render_target;

        {
            let mut b_writer = BATCH_DATA.write();
            let b_writer = b_writer.as_mut().unwrap().as_mut().unwrap();

            b_writer.send(target, state, &vert_data, tri_data);
        }
    }

    /// Sets a custom material for a certain rendering mode.<br>
    /// - If `material` is `None` the default material will be restored for the given mode.
    /// - `mode` cannot be `Unset`
    pub fn set_material(material: Option<Material>, mode: Mode) {
        assert_expr!(!matches!(mode, Mode::Unset), "Mode cannot be unset!");

        let mut writer = GRAPHICS.write().unwrap();
        match mode {
            Mode::Unset => unreachable!(),
            Mode::Rect => writer.rect_material = material,
            Mode::Textured => writer.tex_material = material,
            Mode::Ellipse => writer.ellipse_material = material,
            Mode::Custom => writer.custom_material = material,
        }
    }

    /// Sets the current blending mode.
    pub fn set_blending_mode(mode: BlendingMode) {
        let mut writer = GRAPHICS.write().unwrap();
        writer.blending = mode;
    }

    /// Returns the current blending mode.
    pub fn get_blending_mode() -> BlendingMode {
        let reader = GRAPHICS.read().unwrap();
        return reader.blending;
    }

    /// Sets the current render target.
    pub fn set_render_target(target: u8) {
        let mut writer = GRAPHICS.write().unwrap();
        writer.render_target = target;
    }

    /// Returns the current render target.
    pub fn get_render_target() -> u8 {
        let reader = GRAPHICS.read().unwrap();
        return reader.render_target;
    }

    /// Sets the current clear color.
    pub fn set_clear_col(col: Color4) {
        let mut writer = GRAPHICS.write().unwrap();
        writer.clear_col = col;
    }

    /// Gets the current clear color.
    pub fn get_clear_col() -> Color4 {
        let reader = GRAPHICS.read().unwrap();
        return reader.clear_col;
    }

    /// Gets the camera height.
    pub fn get_cam_height() -> f32 {
        let reader = GRAPHICS.read().unwrap();
        return reader.curr_cam_height;
    }

    /// Sets the current pixels per unit.
    /// - `ppu` must be positive.
    pub fn set_pixels_per_unit(ppu: f32) {
        assert_expr!(ppu > 0.0, "Pixels per unit must be positive!");

        let mut writer = GRAPHICS.write().unwrap();
        writer.pixels_per_unit = ppu;
    }

    /// Returns the current pixels per unit.
    /// - `ppu` must be positive.
    pub fn get_pixels_per_unit() -> f32 {
        let reader = GRAPHICS.read().unwrap();
        return reader.pixels_per_unit;
    }

    /// Sets the camera parameters.
    /// - 'size' must not have any axis be zero.
    /// - Changes will be applied the next frame.
    pub fn set_cam(pos: Vector2, size: Vector2) {
        assert_expr!(size.0 != 0.0 && size.1 != 0.0, "The size of the camera must be a vector with non-zero components.");

        let mut writer = GRAPHICS.write().unwrap();
        writer.scheduled_cam_data = CamData { pos, size };
    }

    /// Forces the camera temporarily to be a new value. This bypasses the camera scheduling, but the value will also be overriden by the scheduled one once the next frame starts.
    /// - 'size' must not have any axis be zero.
    pub unsafe fn force_cam_temp(pos: Vector2, size: Vector2) {
        assert_expr!(size.0 != 0.0 && size.1 != 0.0, "The size of the camera must be a vector with non-zero components.");

        let mut writer = GRAPHICS.write().unwrap();
        writer.curr_cam_mat = Matrix3x3::cam_matrix(pos, size);
        writer.curr_cam_height = size.1;
    }

    /// Returns the camera matrix from the current camera config.
    /// - This matrix will not change when `set_cam` is called until the next frame.
    pub fn get_cam_matrix() -> Matrix3x3 {
        let reader = GRAPHICS.read().unwrap();
        return reader.curr_cam_mat.clone();
    }

    fn get_material(mode: Mode) -> Option<Material> {
        let reader = GRAPHICS.read().unwrap();
        match mode {
            Mode::Unset => None,
            Mode::Rect => Some(reader.rect_material.clone().unwrap_or(reader.default_materials.def_rect_material.clone())),
            Mode::Textured => Some(reader.tex_material.clone().unwrap_or(reader.default_materials.def_tex_material.clone())),
            Mode::Ellipse => Some(reader.ellipse_material.clone().unwrap_or(reader.default_materials.def_ellipse_material.clone())),
            Mode::Custom => reader.custom_material.clone(),
        }
    }

    pub(crate) fn render(pipeline: &dyn RenderPipeline, screen_res: (u32, u32), window: *mut Window) -> RenderStats {
        let reader = GRAPHICS.read().unwrap();
        let b_reader = BATCH_DATA.read();
        let b_reader = b_reader.as_ref().unwrap().as_ref().unwrap();

        let mut products: [_; 256] = std::array::from_fn(|_| None);
        for i in 0..256 {
            if let Some(x) = &b_reader.targets[i] {
                products[i] = Some(x.render_batches.as_slice());
            }
        }

        let render_data = SceneRenderData { products, cam_mat: &reader.curr_cam_mat, clear_col: reader.clear_col };
        let mut screen_rt = RenderTexture::to_screen(screen_res);
        let mut render_stats = RenderStats {
            draw_calls: 0,
            batch_draw_calls: 0,
            rt_draw_calls: 0,
        };

        pipeline.render(&mut screen_rt, &render_data, &mut render_stats);
        unsafe { window.as_mut().unwrap_unchecked() }.swap_buffers();

        return render_stats;
    }

    pub(crate) fn finalize_batch() {
        let mut b_writer = BATCH_DATA.write();
        let b_writer = b_writer.as_mut().unwrap().as_mut().unwrap();

        for i in 0..=255 {
            b_writer.finalize_batch(i);
            b_writer.swap_batch_buffers(i);
        }
    }

    fn get_blending(&self) -> BlendingMode {
        return self.blending;
    }

    fn gen_ref_state<'a>(&'a self, mode: Mode, attribs: &'a [usize], textures: &'a [&'a Texture]) -> RefBatchState<'a> {
        return RefBatchState {
            material: Self::get_material(mode).unwrap(),
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

impl BlendingMode {
    pub(super) fn apply(&self) {
        match self {
            BlendingMode::AlphaMix => gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA)),
            BlendingMode::Additive => gl_call!(gl::BlendFunc(gl::SRC_ALPHA, gl::ONE)),
            BlendingMode::Multiplicative => gl_call!(gl::BlendFunc(gl::DST_COLOR, gl::ZERO)),
        }
    }
}


fn convert_vert_data<T>(src: &[T]) -> &[f32] {
    let mul = std::mem::size_of::<T>() / std::mem::size_of::<f32>();
    return unsafe { std::slice::from_raw_parts(src.as_ptr() as *const f32, src.len() * mul) };
}



#[derive(Debug)]
pub struct RenderStats {
    draw_calls: usize,
    batch_draw_calls: usize,
    rt_draw_calls: usize,
}

impl RenderStats {
    /// Returns the number of total draw calls in a given frame.
    pub fn draw_calls(&self) -> usize {
        self.draw_calls
    }

    /// Returns the number of draw calls performed on geometry batches.
    pub fn batch_draw_calls(&self) -> usize {
        self.batch_draw_calls
    }

    /// Returns the number of draw calls performed on render textures.
    pub fn rt_draw_calls(&self) -> usize {
        self.rt_draw_calls
    }
}