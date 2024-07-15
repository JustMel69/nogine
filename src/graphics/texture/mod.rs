use std::{io::{Read, Seek, BufReader}, sync::Arc};

use image::{EncodableLayout, GenericImageView, ImageError};
use thiserror::Error;

use crate::{assert_expr, color::BColor4, math::{Rect, vec2}, Res};

use super::super::gl_call;

pub mod atlasgen;

#[derive(Debug, Error)]
pub enum TextureError {
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    ImageError(#[from] ImageError),
    #[error("Unsupported texture format")]
    UnsupportedFormat,
}

/// Defines how a texture is scaled.
#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TextureFiltering {
    Closest = gl::NEAREST,
    Linear = gl::LINEAR
}

/// Defines how a texture is sampled when going out of the unit bounds in the uvs.
#[derive(Clone, Copy)]
pub enum TextureWrapping {
    Clamp, Repeat, Wrap
}

/// Bundles the config for a texture.
#[derive(Clone, Copy)]
pub struct TextureCfg {
    pub filtering: TextureFiltering,
    pub wrapping: TextureWrapping,
}

impl Default for TextureCfg {
    fn default() -> Self {
        Self { filtering: TextureFiltering::Linear, wrapping: TextureWrapping::Repeat }
    }
}

#[derive(Clone, Copy)]
pub enum TextureFormat {
    R,
    RA,
    RGB,
    RGBA,
}

#[repr(transparent)]
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TextureCore(u32);
impl Drop for TextureCore {
    fn drop(&mut self) {
        gl_call!(gl::DeleteTextures(1, &self.0))
    }
}

impl TextureCore {
    pub fn enable(&self, slot: u8) {
        assert_expr!(slot < 16, "Opengl cannot ensure more than 16 texture slots at the same time.");

        gl_call!(gl::ActiveTexture(gl::TEXTURE0 + slot as u32));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.0));
    }

    pub(super) fn inner(&self) -> u32 {
        return self.0;
    }
}

#[derive(Debug, Clone)]
pub struct Texture {
    id: Arc<TextureCore>,
    data: Option<Box<[u8]>>,
    dims: (u32, u32)
}

impl Texture {
    /// Loads a texture from a reader.
    pub fn load(src: impl Read + Seek, cfg: TextureCfg) -> Res<Self, TextureError> {
        let tex_data = load_texture_data(src, cfg)?;
        return Ok(Self::new(tex_data.data, tex_data.fmt, tex_data.dims, tex_data.cfg));
    }

    /// Creates a texture from a set of data.
    pub fn new(rgba_colors: Box<[u8]>, fmt: TextureFormat, dims: (u32, u32), cfg: TextureCfg) -> Self {
        assert_expr!(dims.0 != 0 && dims.1 != 0, "None of the axis of the resolution can have 0 as a value.");
        
        let mut id = 0;
        gl_call!(gl::GenTextures(1, &mut id));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, id));

        let (wrapping, filtering, internal_fmt) = internal::wrap_filter_fmt(cfg.wrapping, cfg.filtering, fmt);

        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filtering as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filtering as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrapping as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrapping as i32));

        gl_call!(gl::TexImage2D(gl::TEXTURE_2D, 0, internal_fmt as i32, dims.0 as i32, dims.1 as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, rgba_colors.as_ptr() as *const std::ffi::c_void));

        return Texture { id: Arc::new(TextureCore(id)), data: Some(rgba_colors), dims };
    }

    pub fn empty(fmt: TextureFormat, dims: (u32, u32), cfg: TextureCfg) -> Self {
        assert_expr!(dims.0 != 0 && dims.1 != 0, "None of the axis of the resolution can have 0 as a value.");
        
        let mut id = 0;
        gl_call!(gl::GenTextures(1, &mut id));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, id));

        let (wrapping, filtering, internal_fmt) = internal::wrap_filter_fmt(cfg.wrapping, cfg.filtering, fmt);

        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filtering as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filtering as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrapping as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrapping as i32));

        gl_call!(gl::TexImage2D(gl::TEXTURE_2D, 0, internal_fmt as i32, dims.0 as i32, dims.1 as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null()));

        return Texture { id: Arc::new(TextureCore(id)), data: None, dims };
    }

    pub(crate) unsafe fn from_raw_parts(core: u32, dims: (u32, u32)) -> Self {
        return Self { id: Arc::new(TextureCore(core)), data: None, dims };
    }


    pub fn dims(&self) -> (u32, u32) {
        self.dims
    }

    pub(crate) fn core(&self) -> &TextureCore {
        &self.id
    }

    pub(crate) fn clone_core(&self) -> Arc<TextureCore> {
        self.id.clone()
    }

    pub fn with_pixels<T, F: Fn(Pixels<'_>) -> T>(&self, func: F) -> Option<T> {
        let x = &self.data.as_ref()?[..];

        let px = Pixels { inner: x, res: self.dims };
        return Some(func(px));
    }

    /// Removes the texture data from RAM.
    pub fn invalidate_data(&mut self) {
        self.data = None;
    }
}

impl PartialEq for Texture {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// A fragment of a texture.
#[derive(Clone, Copy)]
pub struct Sprite<'a>(&'a Texture, Rect);

impl<'a> Sprite<'a> {
    pub fn tex(&self) -> &'a Texture {
        self.0
    }

    pub fn rect(&self) -> Rect {
        self.1
    }
}


#[derive(Clone, Copy)]
pub struct SprRect(pub u32, pub u32, pub u32, pub u32);

/// A grid aligned texture. Allows to pull sprites.
pub struct SpriteAtlas {
    internal: Texture,
    sprite_dims: (u32, u32),
}

impl SpriteAtlas {
    pub fn new(tex: Texture, cell_dims: (u32, u32))  -> Self{
        return Self { internal: tex, sprite_dims: cell_dims };
    }

    pub fn get(&self, rect: SprRect) -> Sprite<'_> {
        let tex_dims = self.internal.dims;
        
        let p_pos = (rect.0 * self.sprite_dims.0, rect.1 * self.sprite_dims.1);
        let p_size = (rect.2 * self.sprite_dims.0, rect.3 * self.sprite_dims.1);

        let uv_rect = Rect {
            start: (vec2::from(p_pos) + vec2::one(0.1)).inv_scale(vec2::from(tex_dims)),
            end: (vec2::from((p_pos.0 + p_size.0, p_pos.1 + p_size.1)) + vec2::one(0.1)).inv_scale(vec2::from(tex_dims)),
        };

        return Sprite(&self.internal, uv_rect);
    }

    pub fn tex(&self) -> &Texture {
        return &self.internal;
    }

    pub fn sprite_dims(&self) -> (u32, u32) {
        self.sprite_dims
    }

    /// Creates a new sprite atlas with 1x1 cells.
    pub fn to_freesample(mut self) -> Self {
        self.sprite_dims = (1, 1);
        return self;
    }
}

pub(crate) struct RawTexData {
    pub data: Box<[u8]>,
    pub fmt: TextureFormat,
    pub dims: (u32, u32),
    pub cfg: TextureCfg,
}
pub(crate) fn load_texture_data(src: impl Read + Seek, cfg: TextureCfg) -> Res<RawTexData, TextureError> {
    let decoder = image::io::Reader::new(BufReader::new(src)).with_guessed_format().map_err(|e| TextureError::from(e))?;
        let img = decoder.decode().map_err(|e| TextureError::from(e))?;
        let dims = img.dimensions();
        
        let (data, fmt) = match img {
            image::DynamicImage::ImageLuma8(img) => {
                let data = img.as_bytes().into();
                (data, TextureFormat::R)
            },
            image::DynamicImage::ImageLumaA8(img) => {
                let data = img.as_bytes().into();
                (data, TextureFormat::RA)
            },
            image::DynamicImage::ImageRgb8(img) => {
                let data = img.as_bytes().into();
                (data, TextureFormat::RGB)
            },
            image::DynamicImage::ImageRgba8(img) => {
                let data = img.as_bytes().into();
                (data, TextureFormat::RGBA)
            },
            image::DynamicImage::ImageLuma16(img) => {
                let data = img.as_bytes().iter().skip(1).step_by(2).copied().collect();
                (data, TextureFormat::R)
            },
            image::DynamicImage::ImageLumaA16(img) => {
                let data = img.as_bytes().iter().skip(1).step_by(2).copied().collect();
                (data, TextureFormat::RA)
            },
            image::DynamicImage::ImageRgb16(img) => {
                let data = img.as_bytes().iter().skip(1).step_by(2).copied().collect();
                (data, TextureFormat::RGB)
            },
            image::DynamicImage::ImageRgba16(img) => {
                let data = img.as_bytes().iter().skip(1).step_by(2).copied().collect();
                (data, TextureFormat::RGBA)
            },
            image::DynamicImage::ImageRgb32F(img) => {
                let data = img.as_bytes().chunks(4).map(|x| {
                    let buf = [x[0], x[1], x[2], x[3]];
                    let x = f32::from_le_bytes(buf);
                    (x * 256.0).min(255.0) as u8
                }).collect();
                (data, TextureFormat::RGB)
            },
            image::DynamicImage::ImageRgba32F(img) => {
                let data = img.as_bytes().chunks(4).map(|x| {
                    let buf = [x[0], x[1], x[2], x[3]];
                    let x = f32::from_le_bytes(buf);
                    (x * 256.0).min(255.0) as u8
                }).collect();
                (data, TextureFormat::RGBA)
            },
            _ => return Err(TextureError::UnsupportedFormat),
        };

        return Ok(RawTexData { data, fmt, dims, cfg });
}

pub struct Pixels<'a> {
    inner: &'a [u8],
    res: (u32, u32),
}

impl<'a> Pixels<'a> {
    pub fn get(&self, pos: (u32, u32)) -> BColor4 {
        assert_expr!(pos.0 < self.res.0 && pos.1 < self.res.1, "Pixel out of bounds! (Pos was ({}, {}), Res was ({}, {}))", pos.0, pos.1, self.res.0, self.res.1);
        
        let index = (pos.0 + pos.1 * self.res.0) as usize * 4;
        return BColor4(self.inner[index], self.inner[index + 1], self.inner[index + 2], self.inner[index + 3])
    }

    pub fn res(&self) -> (u32, u32) {
        self.res
    }
}


mod internal {
    use super::{TextureWrapping, TextureFiltering, TextureFormat};

    pub fn wrap_filter_fmt(wrapping: TextureWrapping, filtering: TextureFiltering, fmt: TextureFormat) -> (u32, u32, u32) {
        let wrapping = match wrapping {
            TextureWrapping::Clamp => gl::CLAMP_TO_EDGE,
            TextureWrapping::Repeat => gl::REPEAT,
            TextureWrapping::Wrap => gl::MIRRORED_REPEAT,
        };

        let filtering = match filtering {
            TextureFiltering::Closest => gl::NEAREST,
            TextureFiltering::Linear => gl::LINEAR,
        };

        let internal_fmt = match fmt {
            TextureFormat::R => gl::RED,
            TextureFormat::RA => gl::RG,
            TextureFormat::RGB => gl::RGB,
            TextureFormat::RGBA => gl::RGBA,
        };

        return (wrapping, filtering, internal_fmt);
    }
}