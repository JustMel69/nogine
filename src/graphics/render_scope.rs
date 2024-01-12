use std::{f32::consts::PI, hint::unreachable_unchecked};

use crate::{math::{Matrix3x3, Vector2, Rect, quad::Quad}, color::{Color4, Color}, graphics::Mode, assert_expr, utils::ptr_slice::PtrSlice};

use super::{CamData, material::Material, BlendingMode, batch::{BatchData, RefBatchState}, texture::{Texture, TextureFiltering}, DefaultMaterials, pipeline::{RenderPipeline, RenderTexture, SceneRenderData, DefaultRenderPipeline}, RenderStats, DEFAULT_CAM_DATA};

pub struct RenderScope {
    pub(super) is_global: bool,

    pub(super) scheduled_cam_data: CamData,
    pub(super) cam_data: CamData,
    pub(super) cam_mat: Matrix3x3,
    
    pub(super) pixels_per_unit: f32,
    pub(super) pivot: Vector2,

    line_material: Option<Material>,
    rect_material: Option<Material>,
    tex_material: Option<Material>,
    ellipse_material: Option<Material>,
    custom_material: Option<Material>,

    pub(super) render_target: u8,
    pub(super) clear_col: Color4,
    pub(super) blending: BlendingMode,

    batch_data: BatchData,
}

impl RenderScope {
    pub(super) const fn new_global() -> Self {
        Self {
            is_global: true,
            scheduled_cam_data: DEFAULT_CAM_DATA, cam_data: DEFAULT_CAM_DATA, cam_mat: Matrix3x3::IDENTITY, pixels_per_unit: 1.0, pivot: Vector2::ZERO,
            line_material: None, rect_material: None, tex_material: None, ellipse_material: None, custom_material: None,
            render_target: 0, clear_col: Color4::BLACK, blending: BlendingMode::AlphaMix,
            batch_data: BatchData::new()
        }
    }

    /// Clears the rendered data
    pub fn clear(&mut self) {
        self.batch_data.clear();
    }

    /// Renders the render scope to a texture
    pub fn render_to_texture(&mut self, res: (u32, u32), filtering: TextureFiltering, pipeline: Option<&dyn RenderPipeline>) -> (Texture, RenderStats) {
        self.finalize_batch();

        let pipeline = pipeline.unwrap_or(&DefaultRenderPipeline);

        let mut rt = RenderTexture::new(res, filtering);
        let stats = self.render_internal(&mut rt, pipeline);

        let texture = rt.statify();

        return (texture, stats);
    }

    pub fn tick(&mut self) {
        let cam_data = self.scheduled_cam_data;

        self.cam_data = cam_data;
        self.cam_mat = Matrix3x3::cam_matrix(cam_data.pos, cam_data.half_size);
    }


    
    const RECT_TRIS: [u32; 6] = [0, 1, 2, 2, 3, 0];
    pub(super) fn draw_rect(&mut self, pos: Vector2, extents: Vector2, rot: f32, colors: [Color4; 4]) -> Quad {
        #[repr(C)]
        struct Vert(Vector2, Color4);

        let tf_mat = Matrix3x3::transform_matrix(pos, rot, extents);
        let quad = internal::make_quad(self.pivot, &tf_mat);
        let vert_data = [Vert(quad.ld, colors[0]), Vert(quad.lu, colors[1]), Vert(quad.ru, colors[2]), Vert(quad.rd, colors[3])];
        
        let vert_data = internal::convert_vert_data(&vert_data);

        let state = self.gen_ref_state(Mode::Rect, &[2, 4], &[]);
        self.batch_data.send(self.render_target, state, vert_data, &Self::RECT_TRIS);

        return internal::fix_quad(quad);
    }

    pub(super) fn draw_texture(&mut self, pos: Vector2, scale: Vector2, rot: f32, uvs: Rect, colors: [Color4; 4], tex: &Texture) -> Quad {
        #[repr(C)]
        struct Vert(Vector2, Color4, Vector2);

        let tex_res = tex.dims();
        let extents = (Vector2(tex_res.0 as f32, tex_res.1 as f32) / self.pixels_per_unit).scale(scale).scale(uvs.size());

        let tf_mat = Matrix3x3::transform_matrix(pos, rot, extents);
        let quad = internal::make_quad(self.pivot, &tf_mat);
        let vert_data = [Vert(quad.ld, colors[0], uvs.lu()), Vert(quad.lu, colors[1], uvs.ld()), Vert(quad.ru, colors[2], uvs.rd()), Vert(quad.rd, colors[3], uvs.ru())];

        let textures = &[tex];
        let vert_data = internal::convert_vert_data(&vert_data);
        
        let state = self.gen_ref_state(Mode::Textured, &[2, 4, 2], textures);
        self.batch_data.send(self.render_target, state, vert_data, &Self::RECT_TRIS);

        return internal::fix_quad(quad);
    }

    pub(super) fn draw_ellipse(&mut self, center: Vector2, half_extents: Vector2, rot: f32, color: Color4) -> Quad {
        #[repr(C)]
        struct Vert(Vector2, Color4, Vector2);

        let tf_mat = Matrix3x3::transform_matrix(center - half_extents, rot, half_extents * 2.0);
        let quad = internal::make_quad(self.pivot, &tf_mat);
        let vert_data = [Vert(quad.ld, color, Vector2::UP), Vert(quad.lu, color, Vector2::ZERO), Vert(quad.ru, color, Vector2::RIGHT), Vert(quad.rd, color, Vector2::ONE)];

        let vert_data = internal::convert_vert_data(&vert_data);

        let state = self.gen_ref_state(Mode::Ellipse, &[2, 4, 2], &[]);
        self.batch_data.send(self.render_target, state, vert_data, &Self::RECT_TRIS);

        return internal::fix_quad(quad);
    }

    pub(super) fn draw_polygon(&mut self, center: Vector2, half_extents: Vector2, rot: f32, sides: u32, color: Color4) {
        assert_expr!(sides >= 3, "Every polygon must have at least 3 sides.");

        #[repr(C)]
        struct Vert(Vector2, Color4);

        let delta_theta = (2.0 * PI) / (sides as f32);
        let mut verts = Vec::with_capacity(1 + sides as usize);

        let tf_mat = Matrix3x3::transform_matrix(center, rot, half_extents);

        verts.push(Vert(&tf_mat * Vector2::ZERO, color));
        for i in 0..sides {
            let theta = delta_theta * (i as f32);
            let pos = &tf_mat * (Vector2::UP.rotate(theta) - self.pivot);
            verts.push(Vert(pos, color));
        }
        let mut tris: Vec<u32> = Vec::with_capacity(sides as usize * 3);
        for i in 0..sides {
            let i = i + 1;
            let j = (i % sides) + 1;
            tris.extend_from_slice(&[0, i, j])
        }
        
        let vert_data = internal::convert_vert_data(&verts);

        let state = self.gen_ref_state(Mode::Rect, &[2, 4], &[]);
        self.batch_data.send(self.render_target, state, vert_data, &tris);
    }

    const LINE_TRIS: [u32; 2] = [0, 1];
    pub(super) fn draw_line(&mut self, mut from: Vector2, mut to: Vector2, colors: [Color4; 2]) {
        #[repr(C)]
        struct Vert(Vector2, Color4);

        from.1 = -from.1;
        to.1 = -to.1;

        let vert_data = [Vert(from, colors[0]), Vert(to, colors[1])];
        let vert_data = internal::convert_vert_data(&vert_data);

        let state = self.gen_ref_state(Mode::Line, &[2, 4], &[]);
        self.batch_data.send(self.render_target, state, vert_data, &Self::LINE_TRIS);
    }

    pub(super) unsafe fn draw_custom_mesh(&mut self, pos: Vector2, rot: f32, scale: Vector2, vert_data: &[f32], tri_data: &[u32], vert_attribs: &[usize], textures: &[&Texture]) {
        assert_expr!(tri_data.len() % 3 == 0, "The number of indices must be a multiple of 3.");

        let tf_mat = Matrix3x3::transform_matrix(pos, rot, scale);

        let stride = vert_attribs.iter().sum();
        let vert_data = vert_data.windows(2).step_by(stride).flat_map(|x| {
            let res = &tf_mat * Vector2(x[0], x[1]);
            [res.0, res.1].into_iter()
        }).collect::<Box<[_]>>();
        
        let state = self.gen_ref_state(Mode::Custom, vert_attribs, textures);
        self.batch_data.send(self.render_target, state, &vert_data, tri_data);
    }

    pub(super) fn set_material(&mut self, material: Option<Material>, mode: Mode) {
        assert_expr!(!matches!(mode, Mode::Unset), "Mode cannot be unset!");

        match mode {
            Mode::Unset => unsafe { unreachable_unchecked() },
            Mode::Line => self.line_material = material,
            Mode::Rect => self.rect_material = material,
            Mode::Textured => self.tex_material = material,
            Mode::Ellipse => self.ellipse_material = material,
            Mode::Custom => self.custom_material = material,
        }
    }

    pub(super) fn get_material(&self, mode: Mode) -> Option<Material> {
        assert_expr!(!matches!(mode, Mode::Unset), "Mode cannot be unset!");
        
        return match mode {
            Mode::Unset => unsafe { unreachable_unchecked() },
            Mode::Line => Some(self.line_material.clone().unwrap_or(DefaultMaterials::def_line_material())),
            Mode::Rect => Some(self.rect_material.clone().unwrap_or(DefaultMaterials::def_rect_material())),
            Mode::Textured => Some(self.tex_material.clone().unwrap_or(DefaultMaterials::def_tex_material())),
            Mode::Ellipse => Some(self.ellipse_material.clone().unwrap_or(DefaultMaterials::def_ellipse_material())),
            Mode::Custom => self.custom_material.clone(),
        };
    }

    pub(super) fn render_internal(&self, target: &mut RenderTexture, pipeline: &dyn RenderPipeline) -> RenderStats {
        let render_data = SceneRenderData { products: self.batch_data.targets.as_slice(), clear_col: self.clear_col, cam: &self.cam_mat };
        let mut render_stats = RenderStats {
            draw_calls: 0,
            batch_draw_calls: 0,
            rt_draw_calls: 0,
        };

        pipeline.render(target, &render_data, &mut render_stats);
        return render_stats;
    }

    pub(super) fn finalize_batch(&mut self) {
        for i in 0..=255 {
            self.batch_data.finalize_batch(i);
            self.batch_data.swap_batch_buffers(i);
        }
    }

    pub(super) fn set_camera(&mut self, cam_data: CamData) {
        assert_expr!(cam_data.half_size.0 != 0.0 && cam_data.half_size.1 != 0.0, "The size of the camera must be a vector with non-zero components.");

        self.scheduled_cam_data = cam_data;
    }

    fn gen_ref_state<'a>(&'a self, mode: Mode, attribs: &'a [usize], textures: &'a [&'a Texture]) -> RefBatchState {
        assert_expr!(!matches!(mode, Mode::Custom) || self.custom_material.is_some(), "Must provide a material for custom meshes!");
        
        return RefBatchState {
            material: self.get_material(mode).unwrap(),
            attribs: attribs.into(),
            textures: unsafe { PtrSlice::from(textures).pointerify() },
            blending: self.blending,
            is_line: matches!(mode, Mode::Line),
        };
    }
}

mod internal {
    use crate::math::{quad::Quad, Matrix3x3, Vector2};

    pub fn convert_vert_data<T>(src: &[T]) -> &[f32] {
        let mul = std::mem::size_of::<T>() / std::mem::size_of::<f32>();
        return unsafe { std::slice::from_raw_parts(src.as_ptr() as *const f32, src.len() * mul) };
    }

    pub fn make_quad(pivot: Vector2, tf_mat: &Matrix3x3) -> Quad {
        return Quad {
            ld: tf_mat * (Vector2::ZERO - pivot),
            lu: tf_mat * (Vector2::UP - pivot),
            ru: tf_mat * (Vector2::ONE - pivot),
            rd: tf_mat * (Vector2::RIGHT - pivot),
        };
    }

    pub fn fix_quad(quad: Quad) -> Quad {
        return Quad {
            ld: Vector2(quad.ld.0, -quad.ld.1),
            rd: Vector2(quad.rd.0, -quad.rd.1),
            lu: Vector2(quad.lu.0, -quad.lu.1),
            ru: Vector2(quad.ru.0, -quad.ru.1),
        }
    }
}