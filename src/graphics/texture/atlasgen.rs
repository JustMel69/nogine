use std::{collections::HashMap, hash::Hash};

use crate::{gl_call, math::rect::URect, utils::heap::Heap};

use super::{Texture, TextureCfg, TextureFormat};

pub struct AtlasBuilder<T> {
    pieces: Heap<internal::AtlasPiece<T>>,
    format: TextureFormat
}

impl<T: Clone + Eq + Hash> AtlasBuilder<T> {
    pub const fn new(format: TextureFormat) -> Self {
        Self { pieces: Heap::new(), format }
    }

    pub fn push(&mut self, res: (u32, u32), data: &[u8], id: T) {
        self.pieces.push(internal::AtlasPiece { res, data: data.into(), id });
    }

    pub fn bake(self, cfg: TextureCfg) -> (Texture, HashMap<T, URect>) {
        let full_width = self.pieces.iter().fold(0, |accum, x| x.res.0 + accum) as f32;
        let max_width = self.pieces.iter().fold(full_width.sqrt(), |max, x| (x.res.0 as f32).max(max)).ceil() as u32;

        let pieces = self.pieces.into_ordered_vec();
        
        let iter = internal::AtlasLineIter::new(&pieces, max_width);
        let res = (max_width as u32, iter.clone().map(|x| x.height).sum::<u32>());

        let (_, _, gl_fmt) = super::internal::wrap_filter_fmt(cfg.wrapping, cfg.filtering, self.format);
        
        let tex = Texture::empty(self.format, res, cfg);
        let mut map = HashMap::new();
        let mut cursor_v = 0;
        for internal::AtlasLineData { height, range } in iter {
            let mut cursor_h = 0;
            for x in &pieces[range] {
                gl_call!(gl::BindTexture(gl::TEXTURE_2D, tex.id.0));
                gl_call!(gl::TexSubImage2D(gl::TEXTURE_2D, 0, cursor_h as i32, cursor_v as i32, x.res.0 as i32, x.res.1 as i32, gl_fmt, gl::UNSIGNED_BYTE, x.data.as_ptr() as *const std::ffi::c_void));
                cursor_h += x.res.0;

                map.insert(x.id.clone(), URect(cursor_h, cursor_v, x.res.0, x.res.1));
            }
            cursor_v += height;
        }

        return (tex, map);
    }
}

mod internal {
    use std::ops::Range;

    pub struct AtlasPiece<T> {
        pub res: (u32, u32),
        pub data: Box<[u8]>,
        pub id: T,
    }
    
    impl<T> PartialEq for AtlasPiece<T> {
        fn eq(&self, other: &Self) -> bool {
            return self.res.0.eq(&other.res.0);
        }
    }
    
    impl<T> PartialOrd for AtlasPiece<T> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            return self.res.0.partial_cmp(&other.res.0);
        }
    }


    pub struct AtlasLineData {
        pub height: u32,
        pub range: Range<usize>,
    } 

    pub struct AtlasLineIter<'a, T> {
        slice: &'a [AtlasPiece<T>],
        index: usize,
        max_width: u32,
    }

    impl<'a, T> Clone for AtlasLineIter<'a, T> {
        fn clone(&self) -> Self {
        Self { slice: self.slice.clone(), index: self.index.clone(), max_width: self.max_width.clone() }
    }
    }

    impl<'a, T> AtlasLineIter<'a, T> {
        pub fn new(slice: &'a [AtlasPiece<T>], max_width: u32) -> Self {
            return Self { slice, index: 0, max_width };
        }
    }

    impl<'a, T> Iterator for AtlasLineIter<'a, T> {
        type Item = AtlasLineData;

        fn next(&mut self) -> Option<Self::Item> {
            if self.index >= self.slice.len() {
                return None; // Iter consumed
            }
            
            let start = self.index;

            let mut accum = 0;
            let mut height = 0;
            while self.index < self.slice.len() {
                let data = &self.slice[self.index];

                let width = data.res.0;
                if accum + width > self.max_width {
                    break; // Line overflow
                }

                height = height.max(data.res.1);
                accum += width;

                self.index += 1;
            }

            return Some(AtlasLineData { height, range: start..self.index });
        }
    }
}