use std::io::Read;

use png::Decoder;

use super::super::gl_call;

pub enum ColorKind {
    RGB, RGBA
}

pub enum TextureFiltering {
    Closest, Linear
}

pub enum TextureWrapping {
    Clamp, Repeat, Wrap
}

pub struct TextureCfg {
    pub filtering: TextureFiltering,
    pub wrapping: TextureWrapping,
}

impl Default for TextureCfg {
    fn default() -> Self {
        Self { filtering: TextureFiltering::Linear, wrapping: TextureWrapping::Repeat }
    }
}


pub struct Texture {
    id: u32,
    _colors: Box<[u8]>,
    dims: (u32, u32)
}

impl Texture {
    pub fn load(src: impl Read, cfg: TextureCfg) -> Self {
        let mut decoder = Decoder::new(src);
        let header = decoder.read_header_info().unwrap();
        let dims = header.size();
        
        let kind = match header.color_type {
            png::ColorType::Grayscale => panic!("Grayscale not supported"),
            png::ColorType::Rgb => ColorKind::RGB,
            png::ColorType::Indexed => panic!("Indexed not supported"),
            png::ColorType::GrayscaleAlpha => panic!("Grayscale alpha not supported"),
            png::ColorType::Rgba => ColorKind::RGBA,
        };

        let mut reader = decoder.read_info().unwrap();
        let size = reader.output_buffer_size();

        let mut out_buff = vec![0; size];
        reader.next_frame(out_buff.as_mut_slice()).unwrap();
        let colors: Box<[u8]> = out_buff.into();
        
        return Self::new(colors, dims, kind, cfg);
    }

    pub fn new(colors: Box<[u8]>, dims: (u32, u32), kind: ColorKind, cfg: TextureCfg) -> Self {
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

        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, filtering as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, filtering as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, wrapping as i32));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, wrapping as i32));

        let kind = match kind {
            ColorKind::RGB => gl::RGB,
            ColorKind::RGBA => gl::RGBA,
        };

        gl_call!(gl::TexImage2D(gl::TEXTURE_2D, 0, kind as i32, dims.0 as i32, dims.1 as i32, 0, kind, gl::UNSIGNED_BYTE, colors.as_ptr() as *const std::ffi::c_void));

        return Texture { id, _colors: colors, dims };
    }


    pub fn enable(&self, slot: u8) {
        assert!(slot < 16);

        gl_call!(gl::ActiveTexture(gl::TEXTURE0 + slot as u32));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.id));
    }

    pub fn dims(&self) -> (u32, u32) {
        self.dims
    }
}

