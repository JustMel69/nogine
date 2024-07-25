use core::f32;
use std::sync::Arc;

use al_sys::AlApi;

use crate::{audio::al::al_call, crash, math::{vec2, vec3}};

use super::buffer::ALBuffer;

pub enum ALSourcePositioning {
    Global,
    Panned(f32),
    Space { pos: vec3, dist_range: vec2 },
}

pub enum ALSourceState {
    Initial, Playing, Paused, Stopped
}

pub struct ALSource {
    id: al_sys::ALuint,
    api: Arc<AlApi>,
    buffer: Arc<ALBuffer>,
    pos: ALSourcePositioning,
}

impl ALSource {
    pub fn new(buffer: Arc<ALBuffer>, api: Arc<AlApi>) -> Self {
        let mut id = 0;
        al_call!(api, api.alGenSources(1, &mut id));
        al_call!(api, api.alSourcei(id, al_sys::AL_BUFFER, buffer.id() as i32));

        let mut item = Self { id, buffer, pos: ALSourcePositioning::Global, api };
        item.set_global();
        return item;
    }

    pub fn play_or_resume(&self) {
        al_call!(self.api, self.api.alSourcePlay(self.id));
    }

    pub fn pause(&self) {
        al_call!(self.api, self.api.alSourcePause(self.id));
    }

    pub fn stop(&self) {
        al_call!(self.api, self.api.alSourceStop(self.id));
    }

    pub fn rewind(&self) {
        al_call!(self.api, self.api.alSourceRewind(self.id));
    }

    pub fn set_global(&mut self) {
        self.pos = ALSourcePositioning::Global;
        al_call!(self.api, self.api.alSourcef(self.id, al_sys::AL_ROLLOFF_FACTOR, 0.0));
        al_call!(self.api, self.api.alSourcei(self.id, al_sys::AL_SOURCE_RELATIVE, 1));
        al_call!(self.api, self.api.alSource3f(self.id, al_sys::AL_POSITION, 0.0, 0.0, 0.0));
    }

    /// Pan values will be clamped between `-0.5` and `0.5`.
    pub fn set_pan(&mut self, pan: f32) {
        let pan = pan.clamp(-0.5, 0.5);
        
        self.pos = ALSourcePositioning::Panned(pan);
        al_call!(self.api, self.api.alSourcef(self.id, al_sys::AL_ROLLOFF_FACTOR, 0.0));
        al_call!(self.api, self.api.alSourcei(self.id, al_sys::AL_SOURCE_RELATIVE, 1));
        al_call!(self.api, self.api.alSource3f(self.id, al_sys::AL_POSITION, pan, 0.0, -(1.0 - pan * pan).sqrt()));
    }

    pub fn pan(&self) -> Option<f32> {
        match self.pos {
            ALSourcePositioning::Global => Some(0.0),
            ALSourcePositioning::Panned(x) => Some(x),
            ALSourcePositioning::Space { pos: _, dist_range: _ } => None,
        }
    }

    
    pub fn set_pos_and_distance(&mut self, pos: vec3, mut dist_range: vec2) {
        dist_range.0 = dist_range.0.max(0.0);
        dist_range.1 = dist_range.1.max(dist_range.0 + f32::EPSILON);
        
        self.pos = ALSourcePositioning::Space { pos, dist_range };
        al_call!(self.api, self.api.alSourcef(self.id, al_sys::AL_ROLLOFF_FACTOR, 0.0)); // THIS IS SO I CAN MANUALLY HANDLE ATTENUATION, AS IT DOESN'T SEEM TO WORK AS INTENDED
        al_call!(self.api, self.api.alSourcei(self.id, al_sys::AL_SOURCE_RELATIVE, 0));
        al_call!(self.api, self.api.alSource3f(self.id, al_sys::AL_POSITION, pos.0, pos.1, pos.2));
    }

    pub fn pos_distance(&self) -> Option<(vec3, vec2)> {
        if let ALSourcePositioning::Space { pos, dist_range } = self.pos {
            Some((pos, dist_range))
        } else {
            None
        }
    }

    /// Gain values will be clamped between `0.0` and `1.0`.
    pub fn set_gain(&self, gain: f32) {
        al_call!(self.api, self.api.alSourcef(self.id, al_sys::AL_GAIN, gain.clamp(0.0, 1.0)));
    }

    pub fn gain(&self) -> f32 {
        let mut gain = 0.0;
        al_call!(self.api, self.api.alGetSourcef(self.id, al_sys::AL_GAIN, &mut gain));
        return gain;
    }

    /// Pitch values will be clamped in the range `(0, +inf)`<br>
    /// Each doubling equals a pitch shift of one octave (12 semitones).<br>
    /// `1.0` is default value.
    pub fn set_pitch(&self, gain: f32) {
        al_call!(self.api, self.api.alSourcef(self.id, al_sys::AL_PITCH, gain.max(f32::EPSILON)));
    }

    pub fn pitch(&self) -> f32 {
        let mut pitch = 0.0;
        al_call!(self.api, self.api.alGetSourcef(self.id, al_sys::AL_PITCH, &mut pitch));
        return pitch;
    }

    pub fn set_looping(&self, loops: bool) {
        al_call!(self.api, self.api.alSourcei(self.id, al_sys::AL_LOOPING, loops as i32));
    }

    pub fn looping(&self) -> bool {
        let mut loops = 0;
        al_call!(self.api, self.api.alGetSourcei(self.id, al_sys::AL_LOOPING, &mut loops));
        return loops != al_sys::AL_TRUE as i32;
    }

    pub fn state(&self) -> ALSourceState {
        let mut state = 0;
        al_call!(self.api, self.api.alGetSourcei(self.id, al_sys::AL_SOURCE_STATE, &mut state));

        return match state {
            al_sys::AL_INITIAL => ALSourceState::Initial,
            al_sys::AL_PLAYING => ALSourceState::Playing,
            al_sys::AL_PAUSED  => ALSourceState::Paused,
            al_sys::AL_STOPPED => ALSourceState::Stopped,
            _ => crash!("Invalid or unknown state."),
        };
    }
    
    pub fn buffer(&self) -> &ALBuffer {
        &self.buffer
    }

    pub fn recalculate_gain(&self, clip_gain: f32, channel_gain: f32, listener_pos: vec3) {
        self.set_gain(clip_gain * channel_gain * match self.pos {
            ALSourcePositioning::Global => 1.0,
            ALSourcePositioning::Panned(_) => 1.0,
            ALSourcePositioning::Space { pos, dist_range } => {
                let x = 1.0 - inv_lerp_clamped(dist_range.0, dist_range.1, listener_pos.dist_to(pos));
                x * x
            },
        });
    }
}

impl Drop for ALSource {
    fn drop(&mut self) {
        al_call!(self.api, self.api.alDeleteSources(1, &mut self.id));
    }
}

fn inv_lerp_clamped(min: f32, max: f32, val: f32) -> f32 {
    return ((val - min) / (max - min)).clamp(0.0, 1.0);
}