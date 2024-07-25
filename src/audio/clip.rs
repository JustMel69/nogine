use std::io::{Read, Seek};

use hound::WavReader;
use thiserror::Error;
use uuid::Uuid;

use crate::Res;

use super::al_bindings::buffer::ALBufferFormat;

enum Data {
    Int8(Vec<i8>),
    Int16(Vec<i16>),
}

pub struct AudioClip {
    uuid: Uuid,
    freq: u32,
    format: ALBufferFormat,
    data: Data,
}

impl AudioClip {
    pub fn new<R: Read + Seek + 'static>(data: R) -> Res<Self, AudioClipError> {
        let mut reader = WavReader::new(data).map_err(|e| AudioClipError::DecoderErr(e))?;

        let spec = reader.spec();
        let (bitdepth, channels, freq) = (spec.bits_per_sample, spec.channels, spec.sample_rate);
        let format = match (bitdepth, channels) {
            (8, 1) => ALBufferFormat::Mono8,
            (8, 2) => ALBufferFormat::Stereo8,
            (16, 1) => ALBufferFormat::Mono16,
            (16, 2) => ALBufferFormat::Stereo16,
            _ => return Err(AudioClipError::InvalidFormat(bitdepth, channels)),
        };

        if matches!(spec.sample_format, hound::SampleFormat::Float) {
            return Err(AudioClipError::FloatNotSupported);
        }

        let data = if bitdepth == 8 {
            Data::Int8(reader.samples().collect::<Result<Vec<i8>, _>>()?)
        } else {
            Data::Int16(reader.samples().collect::<Result<Vec<i16>, _>>()?)
        };

        let uuid = Uuid::new_v4();
        return Ok(Self { uuid, data, format, freq });
    }

    pub(crate) fn data_ptr(&self) -> (*const u8, usize) {
        match &self.data {
            Data::Int8(x) => (x.as_ptr() as *const u8, x.len()),
            Data::Int16(x) => (x.as_ptr() as *const u8, x.len() * 2),
        }
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
    
    pub fn freq(&self) -> u32 {
        self.freq
    }
    
    pub fn format(&self) -> ALBufferFormat {
        self.format
    }
    
    pub fn is_stereo(&self) -> bool {
        match self.format {
            ALBufferFormat::Mono8 | ALBufferFormat::Mono16 => false,
            ALBufferFormat::Stereo8 | ALBufferFormat::Stereo16 => true,
        }
    }
}

#[derive(Error, Debug)]
pub enum AudioClipError {
    #[error("Decoding error")]
    DecoderErr(#[from] hound::Error),
    #[error("Invalid format ({0} bitdepth, {1} channels).")]
    InvalidFormat(u16, u16),
    #[error("Floating point format is not supported.")]
    FloatNotSupported,
}