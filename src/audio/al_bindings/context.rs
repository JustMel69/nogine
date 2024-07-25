use std::sync::Arc;

use al_sys::AlApi;

use crate::{audio::al::al_call, math::vec3};

use super::device::ALDevice;

pub struct ALContext {
    api: Arc<AlApi>,
    _device: Arc<ALDevice>,
    context: *mut al_sys::ALCcontext,
}

impl ALContext {
    pub fn new(device: Arc<ALDevice>, api: Arc<AlApi>) -> Self {
        let context = al_call!(api, api.alcCreateContext(device.ptr(), std::ptr::null_mut()));
        //al_call!(api, api.alDistanceModel(al_sys::AL_NONE)); // alDistanceModel doens't seem to work for me, so I guess I'll have to handle it manually
        return Self { _device: device, context, api };
    }
    
    pub fn mark_as_current(&self) {
        al_call!(self.api, self.api.alcMakeContextCurrent(self.context));
    }

    pub fn set_position(&self, pos: vec3) {
        al_call!(self.api, self.api.alListener3f(al_sys::AL_POSITION, pos.0, pos.1, pos.2));
    }

    pub fn position(&self) -> vec3 {
        let mut pos = vec3::ZERO;
        al_call!(self.api, self.api.alGetListener3f(al_sys::AL_POSITION, &mut pos.0, &mut pos.1, &mut pos.2));
        return pos;
    }

    /// Gain values will be clamped between `0.0` and `1.0`.
    pub fn set_gain(&self, gain: f32) {
        al_call!(self.api, self.api.alListenerf(al_sys::AL_GAIN, gain.clamp(0.0, 1.0)));
    }

    pub fn gain(&self) -> f32 {
        let mut gain = 0.0;
        al_call!(self.api, self.api.alGetListenerf(al_sys::AL_GAIN, &mut gain));
        return gain;
    }
}

impl Drop for ALContext {
    fn drop(&mut self) {
        al_call!(self.api, self.api.alcMakeContextCurrent(std::ptr::null_mut()));
        al_call!(self.api, self.api.alcDestroyContext(self.context));
    }
}