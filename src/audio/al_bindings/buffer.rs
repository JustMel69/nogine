use std::{os::raw::c_void, sync::Arc};

use al_sys::AlApi;

use crate::audio::al::al_call;

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ALBufferFormat {
    Mono8 = al_sys::AL_FORMAT_MONO8,
    Mono16 = al_sys::AL_FORMAT_MONO16,
    Stereo8 = al_sys::AL_FORMAT_STEREO8,
    Stereo16 = al_sys::AL_FORMAT_STEREO16,
}

pub struct ALBuffer {
    id: al_sys::ALuint,
    api: Arc<AlApi>,
}

impl ALBuffer {
    pub fn new(format: ALBufferFormat, freq: i32, data: &[u8], api: Arc<AlApi>) -> Arc<Self> {
        let mut id = 0;
        al_call!(api, api.alGenBuffers(1, &mut id));
        al_call!(api, api.alBufferData(id, format as i32, data.as_ptr() as *const c_void, data.len() as i32, freq));
        return Arc::new(Self { id, api });
    }

    pub fn id(&self) -> al_sys::ALuint {
        return self.id;
    }

    pub fn is_stereo(&self) -> bool {
        let mut channel_count = 0;
        al_call!(self.api, self.api.alGetBufferi(self.id, al_sys::AL_CHANNELS, &mut channel_count));
        return channel_count == 2;
    }
}

impl Drop for ALBuffer {
    fn drop(&mut self) {
        al_call!(self.api, self.api.alDeleteBuffers(1, &mut self.id));
    }
}