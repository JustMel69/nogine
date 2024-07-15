use std::{f32::consts::PI, hint::unreachable_unchecked};

use crate::{assert_expr, color::{Color4, Color}, graphics::{ui::text::precalc::LineSplit, Mode}, math::{Matrix3x3, vec2, Rect, quad::Quad}, utils::ptr_slice::PtrSlice};

use super::{CamData, material::Material, BlendingMode, batch::{BatchData, RefBatchState}, texture::{Texture, TextureFiltering}, DefaultMaterials, pipeline::{RenderPipeline, RenderTexture, SceneRenderData, DefaultRenderPipeline}, RenderStats, DEFAULT_CAM_DATA, ui::{UI_SINGLETON, UI, text::Text}};

pub struct RenderScope {
    pub(super) is_global: bool,

    pub(super) cam_data: CamData,
    pub(super) cam_mat: Matrix3x3,
    
    pub(super) pixels_per_unit: f32,
    pub(super) pivot: vec2,

    pub(super) snapping: Option<Snapping>,

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
            cam_data: DEFAULT_CAM_DATA, cam_mat: Matrix3x3::IDENTITY, pixels_per_unit: 1.0, pivot: vec2::ZERO, snapping: None,
            line_material: None, rect_material: None, tex_material: None, ellipse_material: None, custom_material: None,
            render_target: 0, clear_col: Color4::BLACK, blending: BlendingMode::AlphaMix,
            batch_data: BatchData::new(),
        }
    }

    pub const fn new() -> Self {
        Self {
            is_global: false,
            cam_data: DEFAULT_CAM_DATA, cam_mat: Matrix3x3::IDENTITY, pixels_per_unit: 1.0, pivot: vec2::ZERO, snapping: None,
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
        let stats = self.render_internal(&mut rt, false, pipeline);

        let texture = rt.statify();

        return (texture, stats);
    }

    /// Renders to an already existing texture
    pub fn rerender(&mut self, texture: &mut Texture, pipeline: Option<&dyn RenderPipeline>) -> RenderStats {
        self.finalize_batch();

        let pipeline = pipeline.unwrap_or(&DefaultRenderPipeline);

        let mut rt = unsafe { RenderTexture::new_from_existing(&texture) };
        let stats = self.render_internal(&mut rt, false, pipeline);
        unsafe { rt.forget_tex() };

        texture.invalidate_data();

        return stats;
    }

    
    const RECT_TRIS: [u32; 6] = [0, 1, 2, 2, 3, 0];
    pub(super) fn draw_rect(&mut self, pos: vec2, extents: vec2, rot: f32, colors: [Color4; 4]) -> Quad {
        #[repr(C)]
        struct Vert(vec2, Color4);

        let tf_mat = Matrix3x3::transform_matrix(pos, rot, extents);
        let quad = internal::make_quad(self.pivot, &tf_mat, self.snapping.as_ref());
        let vert_data = [Vert(quad.ld, colors[0]), Vert(quad.lu, colors[1]), Vert(quad.ru, colors[2]), Vert(quad.rd, colors[3])];
        
        let vert_data = internal::convert_vert_data(&vert_data);

        let state = self.gen_ref_state(Mode::Rect, &[2, 4], &[]);
        self.batch_data.send(self.render_target, state, vert_data, &Self::RECT_TRIS);

        return internal::fix_quad(quad);
    }

    pub(super) fn draw_texture(&mut self, pos: vec2, scale: vec2, rot: f32, uvs: Rect, colors: [Color4; 4], tex: &Texture) -> Quad {
        #[repr(C)]
        struct Vert(vec2, Color4, vec2);

        let tex_res = tex.dims();
        let extents = (vec2(tex_res.0 as f32, tex_res.1 as f32) / self.pixels_per_unit).scale(scale).scale(uvs.size());

        let tf_mat = Matrix3x3::transform_matrix(pos, rot, extents);
        let quad = internal::make_quad(self.pivot, &tf_mat, self.snapping.as_ref());
        let vert_data = [Vert(quad.ld, colors[0], uvs.lu()), Vert(quad.lu, colors[1], uvs.ld()), Vert(quad.ru, colors[2], uvs.rd()), Vert(quad.rd, colors[3], uvs.ru())];

        let textures = &[tex];
        let vert_data = internal::convert_vert_data(&vert_data);
        
        let state = self.gen_ref_state(Mode::Textured, &[2, 4, 2], textures);
        self.batch_data.send(self.render_target, state, vert_data, &Self::RECT_TRIS);

        return internal::fix_quad(quad);
    }

    pub(super) fn draw_ellipse(&mut self, center: vec2, half_extents: vec2, rot: f32, color: Color4) -> Quad {
        #[repr(C)]
        struct Vert(vec2, Color4, vec2);

        let tf_mat = Matrix3x3::transform_matrix(center - half_extents, rot, half_extents * 2.0);
        let quad = internal::make_quad(self.pivot, &tf_mat, self.snapping.as_ref());
        let vert_data = [Vert(quad.ld, color, vec2::UP), Vert(quad.lu, color, vec2::ZERO), Vert(quad.ru, color, vec2::RIGHT), Vert(quad.rd, color, vec2::ONE)];

        let vert_data = internal::convert_vert_data(&vert_data);

        let state = self.gen_ref_state(Mode::Ellipse, &[2, 4, 2], &[]);
        self.batch_data.send(self.render_target, state, vert_data, &Self::RECT_TRIS);

        return internal::fix_quad(quad);
    }

    pub(super) fn draw_polygon(&mut self, center: vec2, half_extents: vec2, rot: f32, sides: u32, color: Color4) {
        assert_expr!(sides >= 3, "Every polygon must have at least 3 sides.");

        #[repr(C)]
        struct Vert(vec2, Color4);

        let delta_theta = (2.0 * PI) / (sides as f32);
        let mut verts = Vec::with_capacity(1 + sides as usize);

        let tf_mat = Matrix3x3::transform_matrix(center, rot, half_extents);

        verts.push(Vert(&tf_mat * vec2::ZERO, color));
        for i in 0..sides {
            let theta = delta_theta * (i as f32);
            let mut pos = &tf_mat * (vec2::UP.rotate(theta) - self.pivot);
            if let Some(s) = &self.snapping {
                pos = s.snap(pos);
            }
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
    pub(super) fn draw_line(&mut self, mut from: vec2, mut to: vec2, colors: [Color4; 2]) {
        #[repr(C)]
        struct Vert(vec2, Color4);

        from.1 = -from.1;
        to.1 = -to.1;

        if let Some(s) = &self.snapping {
            from = s.snap(from);
            to = s.snap(to);
        }

        let vert_data = [Vert(from, colors[0]), Vert(to, colors[1])];
        let vert_data = internal::convert_vert_data(&vert_data);

        let state = self.gen_ref_state(Mode::Line, &[2, 4], &[]);
        self.batch_data.send(self.render_target, state, vert_data, &Self::LINE_TRIS);
    }

    pub(super) unsafe fn draw_custom_mesh(&mut self, pos: vec2, rot: f32, scale: vec2, vert_data: &[f32], tri_data: &[u32], vert_attribs: &[usize], textures: &[&Texture]) {
        assert_expr!(tri_data.len() % 3 == 0, "The number of indices must be a multiple of 3.");

        let tf_mat = Matrix3x3::transform_matrix(pos, rot, scale);

        let stride = vert_attribs.iter().sum();
        let vert_data = vert_data.windows(2).step_by(stride).flat_map(|x| {
            let mut res = &tf_mat * vec2(x[0], x[1]);
            if let Some(s) = &self.snapping {
                res = s.snap(res);
            }

            [res.0, res.1].into_iter()
        }).collect::<Box<[_]>>();
        
        let state = self.gen_ref_state(Mode::Custom, vert_attribs, textures);
        self.batch_data.send(self.render_target, state, &vert_data, tri_data);
    }

    pub(super) fn draw_text<T>(&mut self, text: &Text<'_, T>) -> (Quad, Option<()>) {
        assert_expr!(text.font.is_some(), "A font was not provided!");

        let bounds_quad = {
            let bounds_mat = Matrix3x3::transform_matrix(text.pos, text.rot, text.bounds_size);
            internal::make_quad(self.pivot, &bounds_mat, self.snapping.as_ref())
        };

        let font = text.font.unwrap();
        let mat = Matrix3x3::transform_matrix(text.pos, text.rot, vec2::ONE);
        let pivot_offset = -self.pivot.scale(text.bounds_size);

        let lines = if text.word_wrapping {
            LineSplit::new(text.txt, font, text.bounds_size.0, font.cfg().char_spacing, text.font_size, text.hor_align).collect::<Vec<_>>()
        } else {
            text.txt.split('\n').map(|x| (x, text.hor_align)).collect::<Vec<_>>()
        };

        let mut char_count = 0;

        let (mut cursor_v, line_spacing) = internal::cursor_v_data(text.ver_align, text.bounds_size.1, text.font_size, font.cfg().line_spacing, lines.len());
        'outer: for (l, align) in lines {
            if cursor_v < -line_spacing {
                break;
            }
            
            if cursor_v > text.bounds_size.1 + line_spacing {
                cursor_v -= text.font_size;
                cursor_v -= line_spacing;
                continue;
            }

            let (mut cursor_h, space_size) = internal::cursor_h_data(l, font, align, text.bounds_size.0, text.font_size);
            let char_spacing = font.cfg().char_spacing * text.font_size;
            for c in l.chars() {
                if c.is_whitespace() {
                    cursor_h += space_size;
                    cursor_h += char_spacing;
                    continue;
                }

                let end_cursor = cursor_h + font.char_width(c) * text.font_size;
                if end_cursor > text.bounds_size.0 + char_spacing {
                    break;
                }

                if cursor_h > -char_spacing {
                    font.draw_char(c, vec2(cursor_h, cursor_v) + pivot_offset, &mat, text.tint, text.font_size, self);
                }
                
                cursor_h = end_cursor;
                cursor_h += char_spacing;

                char_count += 1;
                if let Some(progress) = text.progress {
                    if char_count >= progress {
                        break 'outer;
                    }
                }
            }

            cursor_v -= text.font_size;
            cursor_v -= line_spacing;
        }

        return (internal::fix_quad(bounds_quad), None); // TEMP
    }

    pub(super) unsafe fn draw_manual(&mut self, mode: Mode, vert_data: &[f32], tri_data: &[u32], vert_attribs: &[usize], textures: &[&Texture]) {
        assert_expr!(tri_data.len() % 3 == 0, "The number of indices must be a multiple of 3.");

        let state = self.gen_ref_state(mode, vert_attribs, textures);
        self.batch_data.send(self.render_target, state, &vert_data, tri_data);
    }

    pub(super) fn rect_positions(&mut self, pos: vec2, extents: vec2, rot: f32, should_fix: bool) -> Quad {
        let tf_mat = Matrix3x3::transform_matrix(pos, rot, extents);
        let quad = internal::make_quad(self.pivot, &tf_mat, self.snapping.as_ref());
        
        return if should_fix { internal::fix_quad(quad) } else { quad };
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

    pub(super) fn render_internal(&self, target: &mut RenderTexture, include_ui: bool, pipeline: &dyn RenderPipeline) -> RenderStats {
        let render_data = self.gen_scene_data();
        let mut render_stats = RenderStats {
            draw_calls: 0,
            batch_draw_calls: 0,
            rt_draw_calls: 0,
        };

        if include_ui && UI::is_enabled() {
            let ui_reader = UI_SINGLETON.read().unwrap();
            let ui_data = ui_reader.scope.gen_scene_data();
            pipeline.render(target, &render_data, Some(&ui_data), &mut render_stats);
        } else {
            pipeline.render(target, &render_data, None, &mut render_stats);
        }
        
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

        let cam_pos = if let Some(x) = &self.snapping {
            if x.apply_to_cam { x.snap(cam_data.pos) } else { cam_data.pos }
        } else {
            cam_data.pos
        };

        self.cam_data = cam_data;
        self.cam_mat = Matrix3x3::cam_matrix(cam_pos, cam_data.half_size);
    }

    pub(super) fn set_snapping(&mut self, snapping: Option<Snapping>) {
        assert_expr!(snapping.as_ref().map_or(true, |x| x.grid_size > 0.0), "Grid size must be greater than 0.");
        self.snapping = snapping;

        self.set_camera(self.cam_data);
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

    fn gen_scene_data(&self) -> SceneRenderData<'_> {
        SceneRenderData { products: self.batch_data.targets.as_slice(), clear_col: self.clear_col, cam: &self.cam_mat }
    }
}

mod internal {
    use crate::{crash, graphics::ui::text::{font::Font, precalc::{LenComm, LenLexer}, HorTextAlignment, VerTextAlignment}, math::{quad::Quad, Matrix3x3, vec2}};

    use super::Snapping;

    pub fn convert_vert_data<T>(src: &[T]) -> &[f32] {
        let mul = std::mem::size_of::<T>() / std::mem::size_of::<f32>();
        return unsafe { std::slice::from_raw_parts(src.as_ptr() as *const f32, src.len() * mul) };
    }

    pub fn make_quad(pivot: vec2, tf_mat: &Matrix3x3, snapping: Option<&Snapping>) -> Quad {
        return if let Some(s) = snapping {
            Quad {
                ld: s.snap(tf_mat * (vec2::ZERO - pivot)),
                lu: s.snap(tf_mat * (vec2::UP - pivot)),
                ru: s.snap(tf_mat * (vec2::ONE - pivot)),
                rd: s.snap(tf_mat * (vec2::RIGHT - pivot)),
            }
        } else {
            Quad {
                ld: tf_mat * (vec2::ZERO - pivot),
                lu: tf_mat * (vec2::UP - pivot),
                ru: tf_mat * (vec2::ONE - pivot),
                rd: tf_mat * (vec2::RIGHT - pivot),
            }
        };
    }

    pub fn fix_quad(quad: Quad) -> Quad {
        return Quad {
            ld: vec2(quad.ld.0, -quad.ld.1),
            rd: vec2(quad.rd.0, -quad.rd.1),
            lu: vec2(quad.lu.0, -quad.lu.1),
            ru: vec2(quad.ru.0, -quad.ru.1),
        }
    }

    /// returns `(initial_val, final_line_spacing)`
    pub fn cursor_v_data(alignment: VerTextAlignment, bounds: f32, font_size: f32, line_spacing: f32, line_count: usize) -> (f32, f32) {
        let def_line_spacing = line_spacing * font_size;
        let spacing_count = line_count.max(1) - 1;

        let text_height = line_count as f32 * font_size;
        let spacing_height = spacing_count as f32 * def_line_spacing;
        
        match alignment {
            VerTextAlignment::Top => (bounds - font_size, def_line_spacing),
            VerTextAlignment::Middle => ((text_height + spacing_height + bounds) * 0.5 - font_size, def_line_spacing),
            VerTextAlignment::Bottom => (text_height + spacing_height - font_size, def_line_spacing),
            VerTextAlignment::Expand => (bounds - font_size, (bounds - text_height) / spacing_count as f32),
        }
    }

    /// returns `(initial_val, space_width)`
    pub fn cursor_h_data(line: &str, font: &dyn Font, alignment: HorTextAlignment, bounds: f32, font_size: f32) -> (f32, f32) {
        let spacing = font.cfg().char_spacing;
        
        let iter = LenLexer::new(line, font);
        let (mut wordlen, mut spacelen, word_last, space_counts) = iter.fold((0.0f32, 0.0f32, false, 0), |mut accum, item| {
            match item {
                (LenComm::Word(x), _) => { accum.0 += x; accum.2 = true; },
                (LenComm::Space(x), _) => { accum.1 += x; accum.2 = false; accum.3 += 1 },
                (LenComm::LineBreak, _) => { }
            };
            accum
        });
        
        if word_last {
            wordlen -= spacing;
        } else {
            spacelen -= spacing;
        }

        let default_space = font.cfg().word_spacing * font_size;
        match alignment {
            HorTextAlignment::Left => (0.0, default_space),
            HorTextAlignment::Center => ((bounds - (wordlen + spacelen) * font_size) * 0.5, default_space),
            HorTextAlignment::Right => (bounds - (wordlen + spacelen) * font_size, default_space),
            HorTextAlignment::Justified => crash!("Justified text is not real, do not let the elite lie to you."),
            HorTextAlignment::Expand => (0.0, (bounds - wordlen * font_size) / (space_counts as f32) - font.cfg().char_spacing * font_size),
        }
    }
}

pub(super) struct Snapping {
    pub(super) grid_size: f32,
    pub(super) apply_to_cam: bool,
}

impl Snapping {
    pub(super) fn snap(&self, v: vec2) -> vec2 {
        return (v / self.grid_size).round() * self.grid_size;
    }
}