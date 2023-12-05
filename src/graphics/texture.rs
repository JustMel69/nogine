use std::{io::{Read, Seek, BufReader}, sync::Arc};

use image::{EncodableLayout, GenericImageView, ImageError};
use thiserror::Error;

use crate::{math::Rect, Res};

use super::super::gl_call;

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
pub enum TextureFiltering {
    Closest, Linear
}

/// Defines how a texture is sampled when going out of the unit bounds in the uvs.
pub enum TextureWrapping {
    Clamp, Repeat, Wrap
}

/// Bundles the config for a texture.
pub struct TextureCfg {
    pub filtering: TextureFiltering,
    pub wrapping: TextureWrapping,
}

impl Default for TextureCfg {
    fn default() -> Self {
        Self { filtering: TextureFiltering::Linear, wrapping: TextureWrapping::Repeat }
    }
}

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
        assert!(slot < 16);

        gl_call!(gl::ActiveTexture(gl::TEXTURE0 + slot as u32));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.0));
    }
}

pub struct Texture {
    id: Arc<TextureCore>,
    //_colors: Box<[u8]>,
    dims: (u32, u32)
}

impl Texture {
    /// Loads a texture from a reader.
    pub fn load(src: impl Read + Seek, cfg: TextureCfg) -> Res<Self, TextureError> {
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

        return Ok(Self::new(data, fmt, dims, cfg));
    }

    /// Creates a texture from a set of data.
    pub fn new(rgba_colors: Box<[u8]>, fmt: TextureFormat, dims: (u32, u32), cfg: TextureCfg) -> Self {
        assert!(dims.0 != 0 && dims.1 != 0);
        
        let mut id = 0;
        gl_call!(gl::GenTextures(1, &mut id));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, id));

        let wrapping = match cfg.wrapping {
            TextureWrapping::Clamp => gl::CLAMP_TO_EDGE,
            TextureWrapping::Repeat => gl::REPEAT,
            TextureWrapping::Wrap => gl::MIRRORED_REPEAT,
        };

        let filtering = match cfg.filtering {
            TextureFiltering::Closest => gl::NEAREST,
            TextureFiltering::Linear => gl::LINEAR,
        };

        let internal_fmt = match fmt {
            TextureFormat::R => gl::RED,
            TextureFormat::RA => gl::RG,
            TextureFormat::RGB => gl::RGB,
            TextureFormat::RGBA => gl::RGBA,
        };

        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filtering as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filtering as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrapping as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrapping as i32));

        gl_call!(gl::TexImage2D(gl::TEXTURE_2D, 0, internal_fmt as i32, dims.0 as i32, dims.1 as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, rgba_colors.as_ptr() as *const std::ffi::c_void));

        return Texture { id: Arc::new(TextureCore(id)), /*_colors: rgba_colors,*/ dims };
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
}

/// A fragment of a texture.
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

        let uv_rect = Rect(
            (p_pos.0 as f32) / (tex_dims.0 as f32),
            (p_pos.1 as f32) / (tex_dims.1 as f32),
            (p_size.0 as f32) / (tex_dims.0 as f32),
            (p_size.1 as f32) / (tex_dims.1 as f32),
        );

        return Sprite(&self.internal, uv_rect);
    }
}