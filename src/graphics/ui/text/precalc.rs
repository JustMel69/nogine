use std::{iter::Peekable, str::Chars};

use super::font::Font;

pub struct LenLexer<'a> {
    iter: Chars<'a>,
    font: &'a dyn Font,
    saved: Option<char>,
}

impl<'a> LenLexer<'a> {
    pub fn new(text: &'a str, font: &'a dyn Font) -> Self {
        let iter = text.chars();
        return Self { iter, font, saved: None };
    }

    fn len_inner(&mut self, initial: f32) -> f32 {
        let mut accum = initial;
        while let Some(char) = self.get_next_char() {
            if char.is_whitespace() {
                self.saved = Some(char);
                break;
            }
            accum += self.font.char_width(char) + self.font.cfg().char_spacing;
        }
        return accum;
    }

    fn get_next_char(&mut self) -> Option<char> {
        if let Some(char) = self.saved {
            self.saved = None;
            return Some(char);
        }

        let next = self.iter.next();
        return next;
    }
}

impl<'a> Iterator for LenLexer<'a> {
    type Item = Len;

    fn next(&mut self) -> Option<Self::Item> {
        let char = self.get_next_char()?;
        let width = self.font.char_width(char) + self.font.cfg().char_spacing;

        if char.is_whitespace() {
            return Some(Len::Space(width));
        } else {
            return Some(Len::Word(self.len_inner(width)));
        }
    }
}

pub enum Len {
    Word(f32),
    Space(f32),
}