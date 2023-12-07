use crate::{math::Matrix3x3, color::Color4, graphics::{buffers::{GlVAO, GlBuffer}, verts, DefaultShaders}};

use super::{gl_call, batch::BatchProduct, RenderStats, shader::Shader, texture::TextureFiltering, BlendingMode};

pub struct DefaultRenderPipeline;
impl RenderPipeline for DefaultRenderPipeline {
    fn render(&self, screen_rt: &mut RenderTexture, scene_data: &SceneRenderData, stats: &mut RenderStats) {
        screen_rt.clear(scene_data.clear_col);
        screen_rt.render_scene(scene_data, stats);
    }
}


pub trait RenderPipeline {
    fn render(&self, screen_rt: &mut RenderTexture, scene_data: &SceneRenderData, stats: &mut RenderStats);
}

pub struct RenderTexture {
    fbo: gl::types::GLuint,
    col_tex: gl::types::GLuint,
    res: (u32, u32),
    alpha: f32,
}

impl RenderTexture {
    pub(super) fn to_screen(res: (u32, u32)) -> Self {
        return Self { fbo: 0, col_tex: 0, res, alpha: 1.0 };
    }

    pub fn new(res: (u32, u32), filtering: TextureFiltering) -> Self {
        assert!(res.0 != 0 && res.1 != 0, "None of the resolution axis can be 0");
        
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

    pub fn sized_as(rt: &RenderTexture, filtering: TextureFiltering) -> Self {
        return Self::new(rt.res, filtering);
    }

    pub fn render_scene(&mut self, scene_data: &SceneRenderData, stats: &mut RenderStats) {
        gl_call!(gl::Viewport(0, 0, self.res.0 as i32, self.res.1 as i32));
        
        RenderTexture::bind(self);

        for b in scene_data.products {
            b.render(scene_data.cam_mat);
        }
        stats.draw_calls += scene_data.products.len();
        stats.batch_draw_calls += scene_data.products.len();

        RenderTexture::unbind();
    }

    pub fn clear(&mut self, color: Color4) {
        RenderTexture::bind(self);

        gl_call!(gl::ClearColor(color.0, color.1, color.2, color.3));
        gl_call!(gl::Clear(gl::COLOR_BUFFER_BIT));

        RenderTexture::unbind();
    }

    pub fn downscaled(&self, factor: u32, target_filtering: TextureFiltering, stats: &mut RenderStats) -> Self {
        assert!(factor != 0, "Scaling factor cannot be 0");
        
        let mut target_rt = RenderTexture::new(((self.res.0 / factor).max(1), (self.res.1 / factor).max(1)), target_filtering);
        target_rt.clear(Color4::CLEAR);
        target_rt.render_with_shader(&[&self], &DefaultShaders::def_blit_shader(), BlendingMode::AlphaMix, stats);

        return target_rt;
    }

    pub fn combine(&mut self, source: &Self, blending: BlendingMode, stats: &mut RenderStats) {
        self.render_with_shader(&[source], &DefaultShaders::def_blit_shader(), blending, stats);
    }

    /// None of the sources can be the Screen Render Texture.
    pub fn render_with_shader(&mut self, sources: &[&Self], shader: &Shader, blending: BlendingMode, stats: &mut RenderStats) {
        assert!(sources.iter().all(|x| x.fbo != 0), "No source can be a Screen Render Texture");
        
        gl_call!(gl::Viewport(0, 0, self.res.0 as i32, self.res.1 as i32));

        blending.apply();
        RenderTexture::bind(self);

        let vert_data = [-1.0f32, -1.0, 0.0, 0.0, sources[0].alpha, -1.0 , 1.0, 0.0, 1.0, sources[0].alpha, 1.0, 1.0, 1.0, 1.0, sources[0].alpha, 1.0, -1.0, 1.0, 0.0, sources[0].alpha];
        const TRI_DATA: [u32; 6] = [0, 1, 2, 2, 3, 0];

        shader.enable();

        let vao = GlVAO::new();
        vao.bind();

        let vbo = GlBuffer::new_vbo();
        vbo.set_data_from_slice(&vert_data);

        let ebo = GlBuffer::new_ebo();
        ebo.set_data_from_slice(&TRI_DATA);

        verts::set_vertex_attribs(&[2, 2, 1]);

        for (i, &x) in sources.iter().enumerate() {
            x.use_texture(i as u32);
        }

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
    pub(super) products: &'a [BatchProduct],
    pub(super) cam_mat: &'a Matrix3x3,
    pub(super) clear_col: Color4,
}

impl<'a> SceneRenderData<'a> {
    pub fn clear_col(&self) -> Color4 {
        self.clear_col
    }
}