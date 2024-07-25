use core::f32;
use std::{collections::HashMap, sync::{Arc, Weak}};

use al_sys::AlApi;
use uuid::Uuid;

use crate::{audio::al_bindings::source::ALSourceState, log_warn, math::{vec2, vec3}};

use super::{al_bindings::{buffer::ALBuffer, source::{ALSource, ALSourcePositioning}}, clip::AudioClip, AudioHandle};

struct AudioSource {
    source: ALSource,
    volume: f32,
    uuid: Weak<Uuid>,
}

pub(super) struct AudioChannel {
    api: Arc<AlApi>,
    clips: HashMap<Uuid, AudioSource>,
    volume: f32,
}

impl AudioChannel {
    /// Volume will be clamped between 0.0 and 1.0
    pub fn new(volume: f32, api: Arc<AlApi>) -> Self {
        return Self { clips: HashMap::new(), api, volume};
    }

    pub fn play(&mut self, clip: &AudioClip, loops: bool, volume: f32, pitch: f32, positioning: ALSourcePositioning, sources: &mut HashMap<Uuid, Arc<ALBuffer>>, listener_pos: vec3) -> Arc<Uuid> {
        let uuid = Arc::new(Uuid::new_v4());
        let buffer = match sources.get(&clip.uuid()) {
            Some(x) => x.clone(),
            None => {
                let (data, len) = clip.data_ptr();
                let buf = ALBuffer::new(clip.format(), clip.freq() as i32, unsafe { std::slice::from_raw_parts(data, len) }, self.api.clone());
                sources.insert(clip.uuid(), buf.clone());
                buf
            },
        };

        let mut source = ALSource::new(buffer, self.api.clone());
        source.recalculate_gain(volume, self.volume, listener_pos);
        source.set_pitch(pitch);
        source.set_looping(loops);

        match positioning {
            ALSourcePositioning::Global => source.set_global(),
            ALSourcePositioning::Panned(pan) => source.set_pan(pan),
            ALSourcePositioning::Space { pos, dist_range } => source.set_pos_and_distance(pos, dist_range),
        };

        if clip.is_stereo() && matches!(positioning, ALSourcePositioning::Panned(_) | ALSourcePositioning::Space { pos: _, dist_range: _ }) {
            log_warn!("Panning and spatial audio is not supported for stereo audio clips.");
        }

        source.play_or_resume();
        self.clips.insert(*uuid, AudioSource { source, volume, uuid: Arc::downgrade(&uuid) });
        return uuid;
    }

    pub fn rewind(&self, handle: &AudioHandle) {
        if let Some(player) = self.clips.get(&handle.0) {
            player.source.rewind();
        };
    }

    pub fn pause(&self, handle: &AudioHandle) {
        if let Some(player) = self.clips.get(&handle.0) {
            player.source.pause();
        };
    }

    pub fn resume(&self, handle: &AudioHandle) {
        if let Some(player) = self.clips.get(&handle.0) {
            if let ALSourceState::Paused = player.source.state() {
                player.source.play_or_resume();
            }
        };
    }

    pub fn stop(&self, handle: &AudioHandle) {
        if let Some(player) = self.clips.get(&handle.0) {
            player.source.stop();
        };
    }

    pub fn is_fully_playing(&self, handle: &AudioHandle) -> bool {
        if let Some(player) = self.clips.get(&handle.0) {
            matches!(player.source.state(), ALSourceState::Playing | ALSourceState::Initial)
        } else {
            false
        }
    }

    pub fn is_playing_or_paused(&self, handle: &AudioHandle) -> bool {
        if let Some(player) = self.clips.get(&handle.0) {
            matches!(player.source.state(), ALSourceState::Paused | ALSourceState::Playing | ALSourceState::Initial)
        } else {
            false
        }
    }

    pub fn is_paused(&self, handle: &AudioHandle) -> bool {
        if let Some(player) = self.clips.get(&handle.0) {
            matches!(player.source.state(), ALSourceState::Paused)
        } else {
            false
        }
    }

    pub fn set_channel_volume(&mut self, volume: f32, listener_pos: vec3) {
        self.volume = volume;
        
        for (_, source) in &mut self.clips {
            source.source.recalculate_gain(source.volume, volume, listener_pos);
        }
    }

    pub fn get_channel_volume(&self) -> f32 {
        return self.volume;
    }

    pub fn get_volume(&self, handle: &AudioHandle) -> Option<f32> {
        return self.clips.get(&handle.0).map(|x| x.volume);
    }

    pub fn get_scaled_volume(&self, handle: &AudioHandle) -> Option<f32> {
        return self.clips.get(&handle.0).map(|x| x.source.gain());
    }

    pub fn set_volume(&mut self, handle: &AudioHandle, volume: f32, listener_pos: vec3) {
        if let Some(x) = self.clips.get_mut(&handle.0) {
            x.volume = volume;
            x.source.recalculate_gain(volume, self.volume, listener_pos);
        }
    }

    pub fn get_pan(&self, handle: &AudioHandle) -> Option<f32> {
        return self.clips.get(&handle.0).map(|x| x.source.pan()).flatten();
    }

    pub fn set_pan(&mut self, handle: &AudioHandle, pan: f32) {
        if let Some(x) = self.clips.get_mut(&handle.0) {
            x.source.set_pan(pan);

            if x.source.buffer().is_stereo() {
                log_warn!("Panned audio is not supported for stereo audio clips.");
            }
        }
    }

    pub fn set_pitch(&mut self, handle: &AudioHandle, pitch: f32) {
        if let Some(x) = self.clips.get_mut(&handle.0) {
            x.source.set_pitch(pitch);

            if x.source.buffer().is_stereo() {
                log_warn!("Panned audio is not supported for stereo audio clips.");
            }
        }
    }

    pub fn get_pitch(&self, handle: &AudioHandle) -> Option<f32> {
        return self.clips.get(&handle.0).map(|x| x.source.pitch());
    }

    pub fn set_looping(&mut self, handle: &AudioHandle, loops: bool) {
        if let Some(x) = self.clips.get_mut(&handle.0) {
            x.source.set_looping(loops);
        }
    }

    pub fn get_looping(&self, handle: &AudioHandle) -> Option<bool> {
        return self.clips.get(&handle.0).map(|x| x.source.looping());
    }

    pub fn get_pos_and_distance(&self, handle: &AudioHandle) -> Option<(vec3, vec2)> {
        return self.clips.get(&handle.0).map(|x| x.source.pos_distance()).flatten();
    }

    pub fn set_pos_and_distance(&mut self, handle: &AudioHandle, pos: vec3, dist_range: vec2) {
        if let Some(x) = self.clips.get_mut(&handle.0) {
            x.source.set_pos_and_distance(pos, dist_range);

            if x.source.buffer().is_stereo() {
                log_warn!("Spatial audio is not supported for stereo audio clips.");
            }
        }
    }
    
    pub(super) fn tick(&mut self, listener_pos: vec3) {
        let mut to_remove = vec![];

        for (uuid, clip) in &mut self.clips {
            if clip.uuid.strong_count() == 0 && matches!(clip.source.state(), ALSourceState::Stopped) {
                to_remove.push(*uuid);
                continue;
            }

            clip.source.recalculate_gain(clip.volume, self.volume, listener_pos);
        }

        for uuid in to_remove {
            self.clips.remove(&uuid);
        }
    }
}