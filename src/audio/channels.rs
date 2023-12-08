use std::{collections::HashMap, io::Cursor};

use rodio::{Decoder, SpatialSink, OutputStreamHandle, Sink, Source};
use uuid::Uuid;

use crate::unwrap_res;

use super::{AudioPlayer, clip::AudioClip, AudioPlayerType};

pub(super) struct AudioChannel {
    clips: HashMap<Uuid, AudioPlayer>,
    volume: f32,
}

impl AudioChannel {
    pub fn new(volume: f32) -> Self {
        return Self { clips: HashMap::new(), volume };
    }

    pub fn play(&mut self, clip: &AudioClip, volume: f32, loops: bool, panning: Option<f32>, stream_handle: &OutputStreamHandle) {
        let decoder = unwrap_res!(Decoder::new(Cursor::new(clip.data())));
        let final_volume = self.volume * volume;

        if let Some(panning) = panning {
            let sink = unwrap_res!(SpatialSink::try_new(stream_handle, [panning, 0.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]));

            if loops {
                sink.append(decoder.repeat_infinite());
            } else {
                sink.append(decoder);
            }
            sink.set_speed(final_volume);
            sink.play();

    
            self.clips.insert(clip.uuid(), AudioPlayer::new(final_volume, AudioPlayerType::Panned(sink)));
        } else {
            let sink = unwrap_res!(Sink::try_new(stream_handle));
    
            if loops {
                sink.append(decoder.repeat_infinite());
            } else {
                sink.append(decoder);
            }
            sink.set_speed(final_volume);
            sink.play();
    
            self.clips.insert(clip.uuid(), AudioPlayer::new(final_volume, AudioPlayerType::Simple(sink)));
        }

    }

    pub fn pause(&self, clip: &AudioClip) {
        if let Some(player) = self.clips.get(&clip.uuid()) {
            player.pause();
        };
    }

    pub fn resume(&self, clip: &AudioClip) {
        if let Some(player) = self.clips.get(&clip.uuid()) {
            player.play();
        };
    }

    pub fn stop(&self, clip: &AudioClip) {
        if let Some(player) = self.clips.get(&clip.uuid()) {
            player.stop();
        };
    }

    pub fn is_fully_playing(&self, clip: &AudioClip) -> bool {
        if let Some(player) = self.clips.get(&clip.uuid()) {
            !player.empty() && !player.is_paused()
        } else {
            false
        }
    }

    pub fn is_playing_or_paused(&self, clip: &AudioClip) -> bool {
        if let Some(player) = self.clips.get(&clip.uuid()) {
            !player.empty()
        } else {
            false
        }
    }

    pub fn is_paused(&self, clip: &AudioClip) -> bool {
        if let Some(player) = self.clips.get(&clip.uuid()) {
            player.is_paused()
        } else {
            false
        }
    }

    pub fn set_channel_volume(&mut self, volume: f32) {
        self.volume = volume;

        for (_, v) in &mut self.clips {
            v.set_volume_inherited(self.volume);
        }
    }

    pub fn get_channel_volume(&self) -> f32 {
        return self.volume;
    }

    pub fn get_volume(&self, clip: &AudioClip) -> Option<f32> {
        return self.clips.get(&clip.uuid()).map(|x| x.volume)
    }

    pub fn set_volume(&mut self, clip: &AudioClip, volume: f32) {
        if let Some(x) = self.clips.get_mut(&clip.uuid()) {
            x.set_volume(volume, self.volume);
        }
    }
}