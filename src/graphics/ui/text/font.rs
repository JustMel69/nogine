use std::collections::HashMap;

use crate::{assert_expr, color::Color4, graphics::{consts::UV_RECT_EPSILON, render_scope::RenderScope, texture::{atlasgen::AtlasBuilder, SprRect, Sprite, SpriteAtlas, Texture, TextureCfg, TextureFiltering, TextureFormat, TextureWrapping}, Mode}, math::{rect::URect, Matrix3x3, Vector2}, unwrap_res};

#[allow(private_bounds)]
pub trait Font : FontInternal {
    fn cfg(&self) -> &FontCfg;

    /// Returns the character width in unit size (not multiplied by font size).
    fn char_width(&self, c: char) -> f32;

    /// Returns the sprite associated with a character.
    fn sample_char(&self, c: char) -> Option<Sprite<'_>>;
}

pub struct BitmapFont {
    atlas: SpriteAtlas,
    charset: HashMap<char, SprRect>,
    cfg: FontCfg,
    mono_data: Option<MonospaceData>,
}

impl BitmapFont {
    pub fn new(atlas: SpriteAtlas, charset: &str, cfg: FontCfg) -> Self {
        let cell_width = atlas.tex().dims().0 / atlas.sprite_dims().0;
        let sprite_dims = atlas.sprite_dims();
        let atlas = atlas.to_freesample();

        let mut hashmap = HashMap::new();
        for (i, c) in charset.chars().enumerate() {
            let base_rect = SprRect(
                (i as u32 % cell_width) * sprite_dims.0, (i as u32 / cell_width) * sprite_dims.1,
                sprite_dims.0, sprite_dims.1
            );

            let rect = if cfg.monospace { base_rect } else { Self::tight_fit(&atlas, base_rect) };
            
            hashmap.insert(c, rect);
        }

        let mono_data = if cfg.monospace {
            let spr_dims = atlas.sprite_dims();

            let char_width = spr_dims.0 as f32 / spr_dims.1 as f32;
            Some(MonospaceData { char_width })
        } else {
            None
        };

        return Self { atlas, charset: hashmap, cfg, mono_data };
    }

    pub fn tight_fit(atlas: &SpriteAtlas, rect: SprRect) -> SprRect {
        let px0 = rect.0 * atlas.sprite_dims().0;
        let px1 = (rect.0 + rect.2 - 1) * atlas.sprite_dims().0;

        let py0 = rect.1 * atlas.sprite_dims().1;
        let py1 = (rect.1 + rect.3 - 1) * atlas.sprite_dims().1;

        let res = atlas.tex().with_pixels(|p| {
            // Right sweep
            let mut start = px0;
            'outer: for x in px0..=px1 {
                for y in py0..=py1 {
                    if p.get((x, y)).3 != 0 {
                        start = x;
                        break 'outer;
                    }
                }
            }

            // Left sweep
            let mut end = px1;
            'outer: for x in (start..=px1).rev() {
                for y in py0..=py1 {
                    if p.get((x, y)).3 != 0 {
                        end = x;
                        break 'outer;
                    }
                }
            }

            return (start, end);
        });

        assert_expr!(res.is_some(), "Cannot read pixels from the font texture!");
        let (start, end) = res.unwrap();
        let width = end - start + 1;

        return SprRect(start, rect.1, width, rect.3);
    }

}

impl Font for BitmapFont {
    fn cfg(&self) -> &FontCfg {
        return &self.cfg;
    }

    fn char_width(&self, c: char) -> f32 {
        if c.is_whitespace() {
            if c == '\t' {
                return self.cfg.word_spacing * self.cfg.tab_size;
            }

            return self.cfg.word_spacing;
        }
        
        if let Some(mono_data) = &self.mono_data {
            return mono_data.char_width;
        } else {
            let rect = self.charset.get(&c).copied().unwrap_or(SprRect(0, 0, 1, 1));
            return rect.2 as f32 / rect.3 as f32;
        }
    }

    fn sample_char(&self, c: char) -> Option<Sprite<'_>> {
        let rect = self.charset.get(&c)?;
        return Some(self.atlas.get(*rect));
    }
}

impl FontInternal for BitmapFont {
    fn draw_char(&self, c: char, offset: Vector2, mat: &Matrix3x3, tint: Color4, font_size: f32, scope: &mut RenderScope) {
        #[repr(C)]
        struct Vertex(Vector2, Color4, Vector2);
        
        if let Some(sprite) = self.sample_char(c) {
            let char_size = Vector2(self.char_width(c), 1.0) * font_size;
            let char_quad = internal::make_quad(offset, char_size, mat, scope.snapping.as_ref());

            let rect = sprite.rect().expand(-UV_RECT_EPSILON);

            let vert_data = [
                Vertex(char_quad.ld, tint, rect.lu()),
                Vertex(char_quad.lu, tint, rect.ld()),
                Vertex(char_quad.ru, tint, rect.rd()),
                Vertex(char_quad.rd, tint, rect.ru()),
            ];
            let vert_data = internal::convert_vert_data(&vert_data);
            unsafe { scope.draw_manual(Mode::Textured, vert_data, &[0, 1, 2, 2, 3, 0], &[2, 4, 2], &[sprite.tex()]) };
        }
    }
}

pub struct FontCfg {
    pub monospace: bool,
    pub char_spacing: f32,
    pub line_spacing: f32,
    pub word_spacing: f32,
    pub tab_size: f32,
}

impl Default for FontCfg {
    fn default() -> Self {
        Self { char_spacing: 1.0, line_spacing: 1.0, word_spacing: 1.0, monospace: false, tab_size: 4.0 }
    }
}

pub struct MonospaceData {
    char_width: f32,
}

pub struct RasterFont {
    tex: Texture,
    charset: HashMap<char, URect>,
    origins: HashMap<char, (i32, i32)>,
    cfg: FontCfg,
}

impl RasterFont {
    pub fn new(data: &[u8], quality: f32, filtering: TextureFiltering, cfg: FontCfg) -> Self {
        let font = unwrap_res!(fontdue::Font::from_bytes(data, fontdue::FontSettings::default()));
        let characters = font.chars().keys();

        let mut builder = AtlasBuilder::new(TextureFormat::R);
        let mut origins = HashMap::new();

        for &c in characters {
            let (metrics, data) = font.rasterize(c, quality);

            builder.push((metrics.height as u32, metrics.width as u32), &data, c);
            origins.insert(c, (metrics.xmin, metrics.ymin));
        }

        let (tex, charset) = builder.bake(TextureCfg { filtering, wrapping: TextureWrapping::Clamp });
        return Self { tex, charset, origins, cfg };
    }
}

impl Font for RasterFont {
    fn cfg(&self) -> &FontCfg {
        &self.cfg
    }

    fn char_width(&self, c: char) -> f32 {
        todo!()
    }

    fn sample_char(&self, c: char) -> Option<Sprite<'_>> {
        todo!()
    }
}

impl FontInternal for RasterFont {
    fn draw_char(&self, c: char, offset: Vector2, mat: &Matrix3x3, tint: Color4, font_size: f32, scope: &mut RenderScope) {
        todo!()
    }
}

pub(crate) trait FontInternal {
    fn draw_char(&self, c: char, offset: Vector2, mat: &Matrix3x3, tint: Color4, font_size: f32, scope: &mut RenderScope);
}


mod internal {
    use crate::{graphics::render_scope::Snapping, math::{quad::Quad, Matrix3x3, Vector2}};

    pub fn make_quad(offset: Vector2, size: Vector2, mat: &Matrix3x3, snapping: Option<&Snapping>) -> Quad {
        return if let Some(s) = snapping {
            Quad {
                ld: s.snap(mat * (Vector2::ZERO + offset)),
                lu: s.snap(mat * (size.yvec() + offset)),
                ru: s.snap(mat * (size + offset)),
                rd: s.snap(mat * (size.xvec() + offset)),
            }
        } else {
            Quad {
                ld: mat * (Vector2::ZERO + offset),
                lu: mat * (size.yvec() + offset),
                ru: mat * (size + offset),
                rd: mat * (size.xvec() + offset),
            }
        };
    }

    pub fn convert_vert_data<T>(src: &[T]) -> &[f32] {
        let mul = std::mem::size_of::<T>() / std::mem::size_of::<f32>();
        return unsafe { std::slice::from_raw_parts(src.as_ptr() as *const f32, src.len() * mul) };
    }
}