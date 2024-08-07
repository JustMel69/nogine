use std::marker::PhantomData;

use crate::{color::{Color4, Color}, math::{vec2, Rect, quad::Quad}, assert_expr, graphics::Graphics};

use self::font::Font;

use super::UI;

pub mod font;
pub(in crate::graphics) mod precalc;

pub struct SourcedFromGraphics;
pub struct SourcedFromUI;

pub struct Text<'a, T> {
    _phantom: PhantomData<T>,
    
    pub(crate) pos: vec2,
    pub(crate) bounds_size: vec2,
    pub(crate) rot: f32,
    pub(crate) txt: &'a str,

    pub(crate) tint: Color4,
    pub(crate) font: Option<&'a dyn Font>,
    
    pub(crate) font_size: f32,
    pub(crate) hor_align: HorTextAlignment,
    pub(crate) ver_align: VerTextAlignment,

    pub(crate) word_wrapping: bool,
    pub(crate) progress: Option<usize>,

    pub(crate) rich: bool,
}

impl<'a, T> Text<'a, T> {
    /// Enables rich text.
    #[must_use] pub fn rich(mut self) -> Self {
        assert_expr!(false, "Not Implemented");
        
        self.rich = true;
        return self;
    }

    /// Sets the font.
    #[must_use] pub fn font(mut self, font: &'a dyn Font) -> Self {
        self.font = Some(font);
        return self;
    }

    /// Sets the font size.
    #[must_use] pub fn font_size(mut self, font_size: f32) -> Self {
        self.font_size = font_size;
        return self;
    }

    /// Sets the horizontal alignment.
    #[must_use] pub fn hor_align(mut self, hor_align: HorTextAlignment) -> Self {
        self.hor_align = hor_align;
        return self;
    }

    /// Sets the vertical alignment.
    #[must_use] pub fn ver_align(mut self, ver_align: VerTextAlignment) -> Self {
        self.ver_align = ver_align;
        return self;
    }

    /// Enables text wrapping.
    #[must_use] pub fn word_wrapped(mut self) -> Self {
        self.word_wrapping = true;
        return self;
    }

    /// Sets the progress of the text.
    #[must_use] pub fn progress(mut self, progress: Option<usize>) -> Self {
        self.progress = progress;
        return self;
    }
}

impl<'a> Text<'a, SourcedFromGraphics> {
    pub(crate) fn new(pos: vec2, bounds_size: vec2, rot: f32, txt: &'a str) -> Self {
        Self {
            _phantom: PhantomData,
            pos, bounds_size, rot, txt,
            tint: Color4::WHITE, font: None,
            font_size: 1.0, hor_align: HorTextAlignment::Left, ver_align: VerTextAlignment::Top,
            word_wrapping: false, progress: None,
            rich: false
        }
    }

    /// Sets the text tint.
    #[must_use] pub fn tint(mut self, tint: Color4) -> Self {
        self.tint = tint;
        return self;
    }

    /// Drawws the text.
    pub fn draw(&mut self) -> (Quad, Option<()>) {
        if matches!(self.hor_align, HorTextAlignment::Justified) {
            assert_expr!(self.word_wrapping, "Word wrapping must be enabled for justified text!");
        }

        if matches!(self.hor_align, HorTextAlignment::Expand) {
            assert_expr!(!self.word_wrapping, "Word wrapping must be disabled for horizontal expand text!");
        }

        return Graphics::using_scope(|scope| scope.draw_text(self));
    }
}

impl<'a> Text<'a, SourcedFromUI> {
    pub(crate) fn new(pos: vec2, bounds_size: vec2, tint: Color4, txt: &'a str) -> Self {
        Self {
            _phantom: PhantomData,
            pos, bounds_size, rot: 0.0, txt,
            tint, font: None,
            font_size: 1.0, hor_align: HorTextAlignment::Left, ver_align: VerTextAlignment::Top,
            word_wrapping: false, progress: None,
            rich: false
        }
    }

    /// Draws the text.
    pub fn draw(&mut self) -> (Rect, Option<()>) {
        if matches!(self.hor_align, HorTextAlignment::Justified) {
            assert_expr!(self.word_wrapping, "Word wrapping must be enabled for justified text!");
        }

        if matches!(self.hor_align, HorTextAlignment::Expand) {
            assert_expr!(!self.word_wrapping, "Word wrapping must be disabled for horizontal expand text!");
        }

        return UI::using_singleton(|ui| {
            let res = ui.scope.draw_text(self);
            return (ui.quad_to_rect(res.0), res.1)
        });
    }
}


#[derive(Debug, Clone, Copy)]
pub enum HorTextAlignment {
    Left, Center, Right, Justified, Expand
}

#[derive(Debug, Clone, Copy)]
pub enum VerTextAlignment {
    Top, Middle, Bottom, Expand
}


pub struct TextMetadata {

}