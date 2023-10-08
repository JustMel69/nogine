use std::{io::{Read, Seek}, time::Duration, sync::Arc};

use rodio::Source;

pub struct AudioClip {
    data: Arc<[i16]>,
    
    current_frame_len: Option<usize>,
    channels: u16,
    sample_rate: u32,
    total_duration: Option<Duration>,
}

impl AudioClip {
    pub fn new<R: Read + Seek + Send + Sync + 'static>(data: R) -> Self {
        let decoder = rodio::Decoder::new(data).unwrap();
        let current_frame_len =  decoder.current_frame_len();
        let channels = decoder.channels();
        let sample_rate = decoder.sample_rate();
        let total_duration = decoder.total_duration();
        let data = decoder.collect();

        return Self { data, current_frame_len, channels, sample_rate, total_duration };
    }

    pub(crate) fn gen_player(&self) -> AudioClipPlayer {
        return AudioClipPlayer { data: self.data.clone(), current: 0, current_frame_len: self.current_frame_len, channels: self.channels, sample_rate: self.sample_rate, total_duration: self.total_duration };
    }
}


pub struct AudioClipPlayer {
    data: Arc<[i16]>,
    current: usize,
    
    current_frame_len: Option<usize>,
    channels: u16,
    sample_rate: u32,
    total_duration: Option<Duration>,
}

impl Iterator for AudioClipPlayer {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.data.len() {
            return None;
        }

        let item = self.data[self.current];
        self.current += 1;
        return Some(item);
    }
}

impl Source for AudioClipPlayer {
    fn current_frame_len(&self) -> Option<usize> {
        self.current_frame_len
    }

    fn channels(&self) -> u16 {
        self.channels
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<std::time::Duration> {
        self.total_duration
    }
}