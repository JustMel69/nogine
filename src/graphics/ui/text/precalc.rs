use std::{ops::Range, str::CharIndices};

use super::{font::Font, HorTextAlignment};

pub struct LenLexer<'a> {
    iter: CharIndices<'a>,
    font: &'a dyn Font,
    saved: Option<(usize, char)>,
    len: usize,
}

impl<'a> LenLexer<'a> {
    pub fn new(text: &'a str, font: &'a dyn Font) -> Self {
        let iter = text.char_indices();
        return Self { iter, font, saved: None, len: text.len()};
    }

    fn len_inner(&mut self, initial: f32) -> (f32, usize) {
        let mut accum = initial;
        let mut last_index = self.len - 1;
        while let Some((indx, char)) = self.get_next_char() {
            last_index = indx;
            if char.is_whitespace() {
                self.saved = Some((last_index, char));
                break;
            }
            accum += self.font.char_width(char) + self.font.cfg().char_spacing;
        }
        return (accum, last_index);
    }

    fn get_next_char(&mut self) -> Option<(usize, char)> {
        if let Some(char) = self.saved {
            self.saved = None;
            return Some(char);
        }

        let next = self.iter.next();
        return next;
    }
}

impl<'a> Iterator for LenLexer<'a> {
    type Item = (LenComm, Range<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        let (start_index, char) = self.get_next_char()?;
        let width = self.font.char_width(char) + self.font.cfg().char_spacing;

        if char == '\n' {
            return Some((LenComm::LineBreak, start_index..start_index));
        }

        if char.is_whitespace() {
            return Some((LenComm::Space(width), start_index..start_index));
        } else {
            let (width, end_index) = self.len_inner(width);
            return Some((LenComm::Word(width), start_index..end_index));
        }
    }
}

#[derive(Clone, Copy)]
pub enum LenComm {
    Word(f32),
    Space(f32),
    LineBreak,
}



pub struct LineSplit<'a> {
    text: &'a str,
    iter: LenLexer<'a>,
    bounds: f32,
    saved: Option<(LenComm, Range<usize>)>,
    font_size: f32,
    align: HorTextAlignment,
}

impl<'a> LineSplit<'a> {
    pub fn new(text: &'a str, font: &'a dyn Font, bounds: f32, margin: f32, font_size: f32, align: HorTextAlignment) -> Self {
        return Self { text, iter: LenLexer::new(text, font), bounds: bounds + margin, saved: None, font_size, align };
    }

    fn get_next(&mut self) -> Option<(LenComm, Range<usize>)> {
        if self.saved.is_some() {
            let (comm, range) = self.saved.as_ref().unwrap().clone();

            self.saved = None;
            return Some((comm, range));
        }
        return self.iter.next();
    }
}

impl<'a> Iterator for LineSplit<'a> {
    type Item = (&'a str, HorTextAlignment);

    fn next(&mut self) -> Option<Self::Item> {
        let initial = self.get_next()?;
        
        let mut range = initial.1;
        let mut accum = match initial.0 {
            LenComm::Word(w) => w * self.font_size,
            LenComm::Space(w) => w * self.font_size,
            LenComm::LineBreak => return Some(("", internal::realize(self.align, false))),
        };

        while let Some((comm, new_range)) = self.get_next() {
            match comm {
                LenComm::Word(w) => {
                    accum += w * self.font_size;
                    if accum >= self.bounds {
                        self.saved = Some((comm, new_range));
                        return Some((&self.text[range], internal::realize(self.align, true)));
                    }
                    range = (range.start)..(new_range.end);
                },
                LenComm::Space(w) => accum += w * self.font_size,
                LenComm::LineBreak => return Some((&self.text[range], internal::realize(self.align, false))),
            }
        }

        return Some((&self.text[range.start..=range.end], internal::realize(self.align, false)));
    }
}

mod internal {
    use crate::graphics::ui::text::HorTextAlignment;

    pub fn realize(align: HorTextAlignment, wrapped: bool) -> HorTextAlignment {
        match align {
            HorTextAlignment::Justified => if wrapped { HorTextAlignment::Expand } else { HorTextAlignment::Left },
            x => x
        }
    }
}