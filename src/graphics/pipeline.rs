use crate::{assert_expr, color::Color4, graphics::{buffers::GlVAO, gl_bindings::buffer::{GlBuffer, GlBufferKind, GlBufferUsage}, verts, DefaultMaterials}, math::{ivec2, mat3, uvec2, Rect}};

use super::{gl_call, batch::TargetBatchData, RenderStats, texture::{TextureFiltering, Texture}, BlendingMode, material::Material};

pub const DEFAULT_RENDER_TARGET: u8 = 0;

#[derive(Debug, Clone, Copy)]
pub struct ScreenRect {
    l: i32, r: i32, u: i32, d: i32,
}

impl ScreenRect {
    pub fn new(pos: ivec2, size: ivec2) -> Self {
        return Self { l: pos.0, r: pos.0 + size.0, d: pos.1, u: pos.1 + size.1 };
    }

    pub fn l(&self) -> i32 {
        self.l
    }

    pub fn r(&self) -> i32 {
        self.r
    }

    pub fn u(&self) -> i32 {
        self.u
    }

    pub fn d(&self) -> i32 {
        self.d
    }
}

pub struct DefaultRenderPipeline;
impl RenderPipeline for DefaultRenderPipeline {
    fn render(&self, screen_rt: &mut RenderTexture, scene_data: &SceneRenderData, ui_data: Option<&SceneRenderData>, stats: &mut RenderStats) {
        screen_rt.clear(scene_data.clear_col);
        screen_rt.render_scene(scene_data, DEFAULT_RENDER_TARGET, stats);
        
        if let Some(ui_data) = ui_data {
            screen_rt.render_scene(ui_data, DEFAULT_RENDER_TARGET, stats);
        }
    }
}


pub trait RenderPipeline {
    fn render(&self, screen_rt: &mut RenderTexture, scene_data: &SceneRenderData, ui_data: Option<&SceneRenderData>, stats: &mut RenderStats);
}

pub struct RenderTexture {
    fbo: gl::types::GLuint,
    col_tex: gl::types::GLuint,
    res: uvec2,
    alpha: f32,
}

impl RenderTexture {
    pub(super) fn to_screen(res: uvec2) -> Self {
        return Self { fbo: 0, col_tex: 0, res, alpha: 1.0 };
    }

    pub fn new(res: uvec2, filtering: TextureFiltering) -> Self {
        assert_expr!(res.0 != 0 && res.1 != 0, "None of the resolution axis can be 0");
        
        let mut fbo = 0;
        gl_call!(gl::GenFramebuffers(1, &mut fbo));
        gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, fbo));

        let mut col_tex = 0;
        gl_call!(gl::GenTextures(1, &mut col_tex));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, col_tex));
        gl_call!(gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, res.0 as i32, res.1 as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null()));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filtering as u32 as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filtering as u32 as i32));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));
        
        gl_call!(gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, col_tex, 0));
        gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, 0));

        return Self { fbo, col_tex, res, alpha: 1.0 };
    }

    pub(super) unsafe fn new_from_existing(tex: &Texture) -> Self {
        let mut fbo = 0;
        gl_call!(gl::GenFramebuffers(1, &mut fbo));
        gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, fbo));

        let col_tex = tex.core().inner();
        let res = tex.dims();

        gl_call!(gl::FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, col_tex, 0));
        gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, 0));

        return Self { fbo, col_tex, res, alpha: 1.0 };
    }

    pub fn sized_as(rt: &RenderTexture, filtering: TextureFiltering) -> Self {
        return Self::new(rt.res, filtering);
    }

    pub fn render_scene(&mut self, scene_data: &SceneRenderData, target: u8, stats: &mut RenderStats) {
        gl_call!(gl::Viewport(0, 0, self.res.0 as i32, self.res.1 as i32));
        
        RenderTexture::bind(self);

        if let Some(products) = scene_data.products.iter().find(|x| x.0 == target).map(|x| &x.1) {
            for b in &products.render_batches {
                b.render(scene_data.cam);
            }
            stats.draw_calls += products.render_batches.len();
            stats.batch_draw_calls += products.render_batches.len();
        }

        RenderTexture::unbind();
    }

    pub fn clear(&mut self, color: Color4) {
        RenderTexture::bind(self);

        gl_call!(gl::ClearColor(color.0, color.1, color.2, color.3));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT));

        RenderTexture::unbind();
    }

    pub fn downscaled(&self, factor: u32, target_filtering: TextureFiltering, stats: &mut RenderStats) -> Self {
        assert_expr!(factor != 0, "Scaling factor cannot be 0");
        
        let mut target_rt = RenderTexture::new(uvec2((self.res.0 / factor).max(1), (self.res.1 / factor).max(1)), target_filtering);
        target_rt.clear(Color4::CLEAR);
        target_rt.render_with_shader(&self, &DefaultMaterials::def_blit_material(), BlendingMode::AlphaMix, stats);

        return target_rt;
    }

    pub fn combine(&mut self, source: &Self, blending: BlendingMode, stats: &mut RenderStats) {
        self.render_with_shader(source, &DefaultMaterials::def_blit_material(), blending, stats);
    }

    pub fn combine_ext(&mut self, source: &Self, blending: BlendingMode, rect: ScreenRect, source_uvs: Rect, stats: &mut RenderStats) {
        self.render_with_shader_ext(source, &DefaultMaterials::def_blit_material(), blending, rect, source_uvs, stats);
    }

    /// Soure cannot be the Screen Render Texture.
    pub fn render_with_shader(&mut self, source: &Self, material: &Material, blending: BlendingMode, stats: &mut RenderStats) {
        self.render_with_shader_ext(source, material, blending, ScreenRect::new(ivec2::ZERO, ivec2(self.res.0 as i32, self.res.1 as i32)), Rect::IDENT, stats);
    }

    pub fn render_with_shader_ext(&mut self, source: &Self, material: &Material, blending: BlendingMode, rect: ScreenRect, source_uvs: Rect, stats: &mut RenderStats) {
        assert_expr!(source.fbo != 0, "No source can be a Screen Render Texture");
        
        gl_call!(gl::Viewport(rect.l, rect.d, rect.r - rect.l, rect.u - rect.d));

        blending.apply();
        RenderTexture::bind(self);

        let vert_data = [-1.0, -1.0, source_uvs.left(), source_uvs.down(), source.alpha, -1.0, 1.0, source_uvs.left(), source_uvs.up(), source.alpha, 1.0, 1.0, source_uvs.right(), source_uvs.up(), source.alpha, 1.0, -1.0, source_uvs.right(), source_uvs.down(), source.alpha];
        const TRI_DATA: [u32; 6] = [0, 1, 2, 2, 3, 0];

        material.enable();

        let vao = GlVAO::new();
        vao.bind();

        let vbo = GlBuffer::new(&vert_data, GlBufferKind::VBO, GlBufferUsage::StaticDraw);
        let ebo = GlBuffer::new(&TRI_DATA, GlBufferKind::EBO, GlBufferUsage::StaticDraw);

        vbo.bind();
        ebo.bind();

        verts::set_vertex_attribs(&[2, 2, 1]);
        source.use_texture(0);

        gl_call!(gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null()));
        stats.rt_draw_calls += 1;
        stats.draw_calls += 1;

        RenderTexture::unbind();
        BlendingMode::AlphaMix.apply();
    }

    fn bind(f: &Self) {
        gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, f.fbo));
    }

    fn unbind() {
        gl_call!(gl::BindFramebuffer(gl::FRAMEBUFFER, 0));
    }

    fn use_texture(&self, i: u32) {
        gl_call!(gl::ActiveTexture(gl::TEXTURE0 + i));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.col_tex));
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.alpha = alpha;
    }

    pub fn res(&self) -> uvec2 {
        self.res
    }

    pub fn statify(mut self) -> Texture {
        assert_expr!(self.fbo != 0, "Can't statify a render texture to the screen.");

        let texture = unsafe { Texture::from_raw_parts(self.col_tex, self.res) };
        self.col_tex = 0; // Change this so the gl texture is not freed when RenderTexture is dropped

        return texture;
    }

    pub(super) unsafe fn forget_tex(&mut self) {
        self.col_tex = 0;
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        if self.fbo != 0 {
            gl_call!(gl::DeleteFramebuffers(1, &self.fbo));
        }

        if self.col_tex != 0 {
            gl_call!(gl::DeleteTextures(1, &self.col_tex));
        }
    }
}

pub struct SceneRenderData<'a> {
    pub(super) products: &'a [(u8, TargetBatchData)],
    pub(super) clear_col: Color4,
    pub(super) cam: &'a mat3
}

impl<'a> SceneRenderData<'a> {
    pub fn clear_col(&self) -> Color4 {
        self.clear_col
    }
}