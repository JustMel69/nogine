use std::collections::HashMap;

use crate::{graphics::{consts::UV_RECT_EPSILON, render_scope::RenderScope, texture::{SpriteAtlas, Sprite, SprRect}, Mode}, non_implemented, math::{Matrix3x3, Vector2}, color::Color4};

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
    charset: HashMap<char, (u32, u32)>,
    cfg: FontCfg,
    mono_data: Option<MonospaceData>,
}

impl BitmapFont {
    pub fn new(atlas: SpriteAtlas, charset: &str, cfg: FontCfg) -> Self {
        let cell_width = atlas.tex().dims().0 / atlas.sprite_dims().0;

        let mut hashmap = HashMap::new();
        for (i, c) in charset.chars().enumerate() {
            hashmap.insert(c, (i as u32 % cell_width, i as u32 / cell_width));
        }

        let mono_data = if cfg.monospace {
            let spr_dims = atlas.sprite_dims();

            let char_width = spr_dims.0 as f32 / spr_dims.1 as f32;
            Some(MonospaceData { char_width })
        } else {
            non_implemented!("Non monospace fonts")
        };

        return Self { atlas, charset: hashmap, cfg, mono_data };
    }
}

impl Font for BitmapFont {
    fn cfg(&self) -> &FontCfg {
        return &self.cfg;
    }

    fn char_width(&self, c: char) -> f32 {
        if let Some(mono_data) = &self.mono_data {
            if c == '\t' {
                return self.cfg.word_spacing * self.cfg.tab_size;
            }

            if c.is_whitespace() {
                return self.cfg.word_spacing;
            }

            return mono_data.char_width;
        } else {
            non_implemented!("Non monospace fonts");
        }
    }

    fn sample_char(&self, c: char) -> Option<Sprite<'_>> {
        let pos = self.charset.get(&c)?;
        return Some(self.atlas.get(SprRect(pos.0, pos.1, 1, 1)));
    }
}

impl FontInternal for BitmapFont {
    fn draw_char(&self, c: char, offset: Vector2, mat: &Matrix3x3, tint: Color4, font_size: f32, scope: &mut RenderScope) {
        #[repr(C)]
        struct Vertex(Vector2, Color4, Vector2);
        
        if let Some(sprite) = self.sample_char(c) {
            let char_size = Vector2(self.char_width(c), 1.0) * font_size;
            let char_quad = internal::make_quad(offset, char_size, mat);

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

pub struct MSDFFont;

pub(crate) trait FontInternal {
    fn draw_char(&self, c: char, offset: Vector2, mat: &Matrix3x3, tint: Color4, font_size: f32, scope: &mut RenderScope);
}


mod internal {
    use crate::math::{quad::Quad, Matrix3x3, Vector2};

    pub fn make_quad(offset: Vector2, size: Vector2, mat: &Matrix3x3) -> Quad {
        return Quad {
            ld: mat * (Vector2::ZERO + offset),
            lu: mat * (size.yvec() + offset),
            ru: mat * (size + offset),
            rd: mat * (size.xvec() + offset),
        };
    }

    pub fn convert_vert_data<T>(src: &[T]) -> &[f32] {
        let mul = std::mem::size_of::<T>() / std::mem::size_of::<f32>();
        return unsafe { std::slice::from_raw_parts(src.as_ptr() as *const f32, src.len() * mul) };
    }
}