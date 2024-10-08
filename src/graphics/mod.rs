use std::sync::RwLock;

use gl_bindings::{gl_enable_blend, gl_set_blend, GlBlendingMode};

use crate::{assert_expr, color::{Color, Color4}, graphics::defaults::{DefaultMaterials, DefaultShaders}, log_info, math::{mat3, quad::Quad, uvec2, vec2, Rect}, window::Window};

use self::{material::Material, pipeline::{RenderPipeline, RenderTexture}, render_scope::{RenderScope, Snapping}, texture::{Sprite, Texture}, ui::{text::{SourcedFromGraphics, Text}, UI}};

use super::gl_call;


pub mod render_scope;
pub mod material;
pub mod shader;
pub mod texture;
pub mod uniforms;
pub mod pipeline;
pub mod gfx;
pub mod defaults;
pub mod ui;
pub mod consts;

mod buffers;
mod verts;
mod batch;
mod gl_bindings;


static GRAPHICS: RwLock<Graphics> = RwLock::new(Graphics::new());

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Unset,
    Line,
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


const DEFAULT_CAM_DATA: CamData = CamData { pos: vec2::ZERO, half_size: vec2::ONE };

#[derive(Clone, Copy)]
pub struct CamData {
    pos: vec2,
    half_size: vec2,
}

impl CamData {
    pub fn pos(&self) -> vec2 {
        self.pos
    }

    pub fn half_size(&self) -> vec2 {
        self.half_size
    }
}


/// Main struct for drawing.
pub struct Graphics {
    active_scope: RenderScope,
}

impl Graphics {
    const fn new() -> Self {
        return Self {
            active_scope: RenderScope::new_global(),
        };
    }

    pub(crate) fn init() {
        DefaultShaders::init();
        DefaultMaterials::init();

        gl_enable_blend(); 
        Self::set_blending_mode(BlendingMode::AlphaMix);

        log_info!("Graphics initialized.");
    }

    pub(crate) fn using_scope<T>(func: impl Fn(&mut RenderScope) -> T) -> T {
        let mut writer = GRAPHICS.write().unwrap();
        return func(&mut writer.active_scope);
    }

    pub fn with_scope<T, F: FnMut() -> T>(scope: &mut RenderScope, mut render_fn: F) -> T {
        {
            let mut writer = GRAPHICS.write().unwrap();
            std::mem::swap(scope, &mut writer.active_scope); // Swap render scopes
        }

        let res = render_fn();

        {
            let mut writer = GRAPHICS.write().unwrap();
            std::mem::swap(scope, &mut writer.active_scope); // Rever swapping
        }

        return res;
    }



    // |>-<   Rect Drawing   >-<| //
    
    /// Draws a non rotated rect.
    pub fn draw_rect(pos: vec2, extents: vec2, color: Color4) -> Quad {
        Self::draw_rect_full(pos, extents, 0.0, [color; 4])
    }
    
    /// Draws a rotated rect with control over the color of every vert.<br> 
    /// - The order of the colors for the colors array is<br>
    /// 1 2<br>
    /// 0 3
    pub fn draw_rect_full(pos: vec2, extents: vec2, rot: f32, colors: [Color4; 4]) -> Quad {
        GRAPHICS.write().unwrap().active_scope.draw_rect(pos, extents, rot, colors)
    }



    // |>-<   Texture Drawing   >-<| //

    /// Draws a rotated texture.<br>
    /// - The size of the rect depends on the stablished pixels-per-unit and the scale.
    pub fn draw_texture(pos: vec2, scale: vec2, rot: f32, tex: &Texture) -> Quad {
        Self::draw_texture_full(pos, scale, rot, Rect::IDENT, [Color4::WHITE; 4], tex)
    }

    /// Draws a rotated sprite.<br>
    /// - The size of the rect depends on the stablished pixels-per-unit and the scale.
    pub fn draw_sprite(pos: vec2, scale: vec2, rot: f32, sprite: Sprite) -> Quad {
        Self::draw_texture_full(pos, scale, rot, sprite.rect(), [Color4::WHITE; 4], sprite.tex())
    }

    /// Draws a rotated texture with control over the color of each vert and the uv rect utilized.<br>
    /// - The size of the rect depends on the stablished pixels-per-unit and the scale.
    /// - The order of the colors for the colors array is<br>
    /// 1 2<br>
    /// 0 3
    pub fn draw_texture_full(pos: vec2, scale: vec2, rot: f32, uvs: Rect, colors: [Color4; 4], tex: &Texture) -> Quad {
        GRAPHICS.write().unwrap().active_scope.draw_texture(pos, scale, rot, uvs, colors, tex)
    }


    // |>-<   Ellipse Drawing   >-<| //

    /// Draws a circle.
    pub fn draw_circle(center: vec2, radius: f32, color: Color4) -> Quad {
        Self::draw_ellipse(center, vec2(radius, radius), 0.0, color)
    }

    /// Draws a rotated ellipse.
    /// - The ellipse is rotated around the center.
    pub fn draw_ellipse(center: vec2, half_extents: vec2, rot: f32, color: Color4) -> Quad {
        GRAPHICS.write().unwrap().active_scope.draw_ellipse(center, half_extents, rot, color)
    }



    // |>-<   N-sided polygon   >-<| //

    /// Draws a rotated polygon.
    /// - The polygon is rotated around the center.
    pub fn draw_polygon(center: vec2, radius: f32, rot: f32, sides: u32, color: Color4) {
        Self::draw_polygon_ext(center, vec2(radius, radius), rot, sides, color);
    }

    /// Draws an scaled and rotated polygon.
    /// - The polygon is rotated around the center.
    pub fn draw_polygon_ext(center: vec2, half_extents: vec2, rot: f32, sides: u32, color: Color4) {
        GRAPHICS.write().unwrap().active_scope.draw_polygon(center, half_extents, rot, sides, color);
    }
    


    // |>-<   Line Drawing   >-<| //

    /// Draws a line with the desired color.
    pub fn draw_line(from: vec2, to: vec2, color: Color4) {
        Self::draw_line_ext(from, to, [color; 2])
    }

    /// Draws a line with the desired colors. The first color is the start color, and the last is the end color.
    pub fn draw_line_ext(from: vec2, to: vec2, colors: [Color4; 2]) {
        GRAPHICS.write().unwrap().active_scope.draw_line(from, to, colors);
    }


    /// Draw a custom mesh. Prone to not behaving. Not affected by pivot.
    pub unsafe fn draw_custom_mesh(pos: vec2, rot: f32, scale: vec2, vert_data: &[f32], tri_data: &[u32], vert_attribs: &[usize], textures: &[&Texture]) {
        GRAPHICS.write().unwrap().active_scope.draw_custom_mesh(pos, rot, scale, vert_data, tri_data, vert_attribs, textures);
    }

    /// Creates a new text.
    #[must_use]
    pub fn text(pos: vec2, bounds_size: vec2, rot: f32, text: &str) -> Text<'_, SourcedFromGraphics> {
        return Text::<'_, SourcedFromGraphics>::new(pos, bounds_size, rot, text);
    }


    /// Draws a quad
    pub fn draw_debug_quad(quad: Quad, color: Color4) {
        let mut writer = GRAPHICS.write().unwrap();

        writer.active_scope.draw_line(quad.ld, quad.lu, [color; 2]);
        writer.active_scope.draw_line(quad.lu, quad.ru, [color; 2]);
        writer.active_scope.draw_line(quad.ru, quad.rd, [color; 2]);
        writer.active_scope.draw_line(quad.rd, quad.ld, [color; 2]);
    }

    /// Sets a custom material for a certain rendering mode.<br>
    /// - If `material` is `None` the default material will be restored for the given mode.
    /// - `mode` cannot be `Unset`
    pub fn set_material(material: Option<Material>, mode: Mode) {
        GRAPHICS.write().unwrap().active_scope.set_material(material, mode);
    }

    /// Sets the current blending mode.
    pub fn set_blending_mode(mode: BlendingMode) {
        GRAPHICS.write().unwrap().active_scope.blending = mode;
    }

    /// Returns the current blending mode.
    pub fn get_blending_mode() -> BlendingMode {
        return GRAPHICS.read().unwrap().active_scope.blending;
    }

    /// Sets the current render target.
    pub fn set_render_target(target: u8) {
        GRAPHICS.write().unwrap().active_scope.render_target = target;
    }

    /// Returns the current render target.
    pub fn get_render_target() -> u8 {
        return GRAPHICS.read().unwrap().active_scope.render_target;
    }

    /// Sets the current clear color.
    pub fn set_clear_col(col: Color4) {
        GRAPHICS.write().unwrap().active_scope.clear_col = col;
    }

    /// Gets the current clear color.
    pub fn get_clear_col() -> Color4 {
        return GRAPHICS.read().unwrap().active_scope.clear_col;
    }

    /// Gets the camera data.
    pub fn get_cam_data() -> CamData {
        return GRAPHICS.read().unwrap().active_scope.cam_data;
    }

    /// Sets the current pixels per unit.
    /// - `ppu` must be positive.
    pub fn set_pixels_per_unit(ppu: f32) {
        assert_expr!(ppu > 0.0, "Pixels per unit must be positive!");
        GRAPHICS.write().unwrap().active_scope.pixels_per_unit = ppu;
    }

    /// Returns the current pixels per unit.
    /// - `ppu` must be positive.
    pub fn get_pixels_per_unit() -> f32 {
        return GRAPHICS.read().unwrap().active_scope.pixels_per_unit;
    }

    /// Sets the pivot.
    pub fn set_pivot(pivot: vec2) {
        GRAPHICS.write().unwrap().active_scope.pivot = pivot;
    }

    /// Returns the pivot.
    pub fn get_pivot() -> vec2 {
        return GRAPHICS.read().unwrap().active_scope.pivot;
    }

    /// Sets snapping.
    pub fn set_snapping(grid_size: f32, apply_to_cam: bool) {
        GRAPHICS.write().unwrap().active_scope.set_snapping(Some(Snapping { grid_size, apply_to_cam }));
    }

    /// Clear snapping.
    pub fn clear_snapping() {
        GRAPHICS.write().unwrap().active_scope.set_snapping(None);
    }

    /// Gets snapping.
    pub fn get_snapping() -> Option<(f32, bool)> {
        GRAPHICS.read().unwrap().active_scope.snapping.as_ref().map(|x| (x.grid_size, x.apply_to_cam))
    }

    /// Sets the camera parameters.
    /// - 'half_size' must not have any axis be zero.
    /// - For the global render scope, changes will be applied the next frame.
    /// - For non global render scopes, changes will be applied on the next tick.
    pub fn set_cam(pos: vec2, half_size: vec2) {
        GRAPHICS.write().unwrap().active_scope.set_camera(CamData { pos, half_size });
    }

    /// Returns the camera matrix from the current camera config.
    /// - This matrix will not change for the global render scope when `set_cam` is called until the next frame.
    pub fn get_cam_matrix() -> mat3 {
        return GRAPHICS.read().unwrap().active_scope.cam_mat.clone();
    }

    /// Returns the current material for a mode.
    /// - The result will only be none if the mode is `Mode::Custom` and no custom material was provided. Otherwise, it's safe to unwrap.
    pub fn get_material(mode: Mode) -> Option<Material> {
        return GRAPHICS.read().unwrap().active_scope.get_material(mode);
    }

    pub(crate) fn render(pipeline: &dyn RenderPipeline, screen_res: uvec2, window: *mut Window) -> RenderStats {
        let reader = GRAPHICS.read().unwrap();
        assert_expr!(reader.active_scope.is_global, "The global render scope must be active at the end of the frame!");
        
        let mut screen_rt = RenderTexture::to_screen(screen_res);

        let stats = GRAPHICS.read().unwrap().active_scope.render_internal(&mut screen_rt, true, pipeline);
        unsafe { window.as_mut().unwrap_unchecked() }.swap_buffers();
        return stats;
    }

    pub(crate) fn finalize_batch() {
        GRAPHICS.write().unwrap().active_scope.finalize_batch();

        if UI::is_enabled() {
            UI::finalize_batch();
        }
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
            BlendingMode::AlphaMix => gl_set_blend(GlBlendingMode::AlphaMix),
            BlendingMode::Additive => gl_set_blend(GlBlendingMode::Additive),
            BlendingMode::Multiplicative => gl_set_blend(GlBlendingMode::Multiplicative),
        }
    }
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
