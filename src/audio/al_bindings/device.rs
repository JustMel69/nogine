use std::sync::Arc;

use al_sys::AlApi;

use crate::audio::al::al_call;

pub struct ALDevice {
    api: Arc<AlApi>,
    device: *mut al_sys::ALCdevice,
}

impl ALDevice {
    pub fn new(name: Option<&[u8]>, api: Arc<AlApi>) -> Arc<Self> {
        let device = al_call!(api, api.alcOpenDevice(if let Some(x) = name { x.as_ptr() as *const al_sys::ALCchar } else { std::ptr::null() }));

        return Arc::new(Self { device, api });
    }
    
    pub fn ptr(&self) -> *mut al_sys::ALCdevice_struct {
        self.device
    }
}

impl Drop for ALDevice {
    fn drop(&mut self) {
        al_call!(self.api, self.api.alcCloseDevice(self.device));
    }
}