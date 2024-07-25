use std::{collections::HashMap, sync::Arc};

use al_bindings::{buffer::ALBuffer, context::ALContext, device::ALDevice, source::ALSourcePositioning};
use uuid::Uuid;

use crate::{crash, log_info, log_warn, math::{vec2, vec3}, unwrap_res};

use self::{clip::AudioClip, channels::AudioChannel};

pub mod clip;
pub mod channels;
mod al;
mod al_bindings;

/*
 * I'll explain some design choices about this.
 * First, there's a context per channel, this is to be able to easily regulate the volume per channel.
 */

// TODO: Make this shit less unsafe
static mut AUDIO: Option<Audio> = None;

pub type AudioChannelID = u32;

#[derive(Debug, Clone, PartialEq)]
pub struct AudioHandle(Arc<Uuid>, AudioChannelID);


/// Main struct for Audio playing.
pub struct Audio {
    al_api: Arc<al_sys::AlApi>,
    
    _device: Arc<ALDevice>,
    context: ALContext,

    channels: Vec<(u32, AudioChannel)>,
    buffers: HashMap<Uuid, Arc<ALBuffer>>,
}

impl Audio {
    fn new() -> Self {
        let al_api = Arc::new(unwrap_res!(al_sys::AlApi::load_default()));
        //unsafe { al::load_common(&al_api) };
        
        let device = ALDevice::new(None, al_api.clone());
        let context = ALContext::new(device.clone(), al_api.clone());
        context.mark_as_current();

        return Self { al_api, _device: device, context, channels: Vec::new(), buffers: HashMap::new() };
    }
    
    pub(crate) fn init() {
        unsafe { AUDIO = Some(Self::new()) };

        log_info!("Audio initialized.");
    }

    /// Plays an audio clip.
    /// - Default `volume` is `0.0`.
    pub fn play(clip: &AudioClip, channel_id: AudioChannelID, loops: bool, volume: f32) -> AudioHandle {
        Self::play_panned_ext(clip, channel_id, loops, volume, 1.0, 0.0)
    }

    /// Plays an audio clip with a certain panning.
    /// - Default `volume` is `0.0`.
    /// - Default `panning` is `0.0`, value is clamped to range `[-0.5, 0.5]`.
    pub fn play_panned(clip: &AudioClip, channel_id: AudioChannelID, loops: bool, volume: f32, panning: f32) -> AudioHandle {
        Self::play_panned_ext(clip, channel_id, loops, volume, 1.0, panning)
    }

    /// Plays an audio clip with a certain panning and pitch.
    /// - Default `volume` is `0.0`.
    /// - Default `pitch` is `1.0`. Values less or equal to `0.0` will be clamped to `f32::EPSILON`. Doubling this value increases the pitch an octave.
    /// - Default `panning` is `0.0`, value is clamped to range `[-0.5, 0.5]`.
    pub fn play_panned_ext(clip: &AudioClip, channel_id: AudioChannelID, loops: bool, volume: f32, pitch: f32, panning: f32) -> AudioHandle {
        let audio = unsafe { &mut AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index(channel_id) {
            Some(x) => x,
            None => crash!("Audio channel {channel_id} doesn't exist."),
        };

        return AudioHandle(
            audio.channels[index].1.play(clip, loops, volume, pitch, if panning == 0.0 { ALSourcePositioning::Global } else { ALSourcePositioning::Panned(panning) }, &mut audio.buffers, audio.context.position()),
            channel_id,
        );
    }

    /// Plays an audio clip with a certain panning.
    /// - Default `volume` is `0.0`.
    pub fn play_at(clip: &AudioClip, channel_id: AudioChannelID, loops: bool, volume: f32, pos: vec3, distance_range: vec2) -> AudioHandle {
        Self::play_at_ext(clip, channel_id, loops, volume, 1.0, pos, distance_range)
    }

    /// Plays an audio clip with a certain panning and pitch.
    /// - Default `volume` is `0.0`.
    /// - Default `pitch` is `1.0`. Values less or equal to `0.0` will be clamped to `f32::EPSILON`. Doubling this value increases the pitch an octave.
    pub fn play_at_ext(clip: &AudioClip, channel_id: AudioChannelID, loops: bool, volume: f32, pitch: f32, pos: vec3, distance_range: vec2) -> AudioHandle {
        let audio = unsafe { &mut AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index(channel_id) {
            Some(x) => x,
            None => crash!("Audio channel {channel_id} doesn't exist."),
        };

        return AudioHandle(
            audio.channels[index].1.play(clip, loops, volume, pitch, ALSourcePositioning::Space { pos, dist_range: distance_range }, &mut audio.buffers, audio.context.position()),
            channel_id,
        );
    }

    /// Rewinds a clip.
    pub fn rewind(handle: &AudioHandle) {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => return,
        };

        audio.channels[index].1.rewind(handle);
    }

    /// Pauses a clip.
    pub fn pause(handle: &AudioHandle) {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => return,
        };

        audio.channels[index].1.pause(handle);
    }

    /// Resumes a clip.
    pub fn resume(handle: &AudioHandle) {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => return,
        };

        audio.channels[index].1.resume(handle);
    }

    /// Stops a clip.
    pub fn stop(handle: &AudioHandle) {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => return,
        };

        audio.channels[index].1.stop(handle);
    }

    /// Checks if a clip is actually playing.
    pub fn is_fully_playing(handle: &AudioHandle) -> bool {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.is_fully_playing(handle);
    }

    /// Checks if a clip is playing or paused.
    pub fn is_playing_or_paused(handle: &AudioHandle) -> bool {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.is_playing_or_paused(handle);
    }

    /// Checks if the clip is paused.
    pub fn is_paused(handle: &AudioHandle) -> bool {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.is_paused(handle);
    }

    /// Sets the volume of the clip
    pub fn set_volume(handle: &AudioHandle, volume: f32) {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.set_volume(handle, volume, audio.context.position());
    }

    /// Returns the volume of the clip, or `None` if it's not playing
    pub fn get_volume(handle: &AudioHandle) -> Option<f32> {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.get_volume(handle);
    }

    /// Returns the volume of the clip, scaled by channel volume and distance if applicable, or `None` if it's not playing
    pub fn get_scaled_volume(handle: &AudioHandle) -> Option<f32> {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.get_scaled_volume(handle);
    }

    /// Sets the pan of the clip.<br>
    /// Pan will be clamped between `-0.5` and `0.5`.
    pub fn set_pan(handle: &AudioHandle, pan: f32) {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.set_pan(handle, pan);
    }

    /// Returns the pan of the clip, or `None` if it's not playing or isn't panned.
    pub fn get_pan(handle: &AudioHandle) -> Option<f32> {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.get_pan(handle);
    }

    /// Sets the pan of the clip.<br>
    /// - Pan will be clamped between `f32::EPSILON` and `f32::INFINITY`.
    /// - Each doubling equals a pitch shift of one octave (12 semitones). Default is `1.0`.
    pub fn set_pitch(handle: &AudioHandle, pitch: f32) {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.set_pitch(handle, pitch);
    }

    /// Returns the pitch of the clip, or `None` if it's not playing.
    pub fn get_pitch(handle: &AudioHandle) -> Option<f32> {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.get_pitch(handle);
    }

    /// Sets if a clip should loop.
    pub fn set_looping(handle: &AudioHandle, loops: bool) {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.set_looping(handle, loops);
    }

    /// Returns if the clip loops, or `None` if it's not playing.
    pub fn get_looping(handle: &AudioHandle) -> Option<bool> {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.get_looping(handle);
    }

    /// Sets the 3D pos and distances of the clip.<br>
    /// Both the min distance and the max distance will be made greater than 0, and the max greater than min.
    pub fn set_pos_and_distance(handle: &AudioHandle, pos: vec3, distance_range: vec2) {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.set_pos_and_distance(handle, pos, distance_range);
    }

    /// Returns the 3D pos and distances of the clip, or `None` if it's not playing or isn't spatial.
    pub fn get_pos_and_distance(handle: &AudioHandle) -> Option<(vec3, vec2)> {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index(handle.1) {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.get_pos_and_distance(handle);
    }

    /// Sets the volume of a channel
    pub fn set_channel_volume(channel_id: AudioChannelID, volume: f32) {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        if let Some(index) = audio.channels.iter().position(|x| x.0 == channel_id) {
            audio.channels[index].1.set_channel_volume(volume, audio.context.position());
        } else {
            log_warn!("Channel {} is not a valid audio channel!", channel_id);
        }
    }

    /// Returns the volume of a channel
    pub fn get_channel_volume(channel_id: AudioChannelID) -> f32 {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        if let Some(index) = audio.channels.iter().position(|x| x.0 == channel_id) {
            return audio.channels[index].1.get_channel_volume();
        } else {
            crash!("Channel {} is not a valid audio channel!", channel_id);
        }
    }

    /// Sets the global volume
    pub fn set_master_volume(volume: f32) {
        let audio = unsafe { AUDIO.as_mut().unwrap() };
        audio.context.set_gain(volume);
    }

    /// Returns the global volume
    pub fn get_master_volume() -> f32 {
        let audio = unsafe { AUDIO.as_mut().unwrap() };
        return audio.context.gain();
    }

    /// Creates an audio channel
    pub fn create_channel(channel_id: AudioChannelID, volume: f32) {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        log_info!("Audio channel {channel_id} created!");
        audio.channels.push((channel_id, AudioChannel::new(volume, audio.al_api.clone())));
    }

    pub fn set_listener_position(pos: vec3) {
        let audio = unsafe { AUDIO.as_mut().unwrap() };
        audio.context.set_position(pos);
    }

    pub(crate) fn tick() {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        for (_, channel) in &mut audio.channels {
            channel.tick(audio.context.position());
        }
    }

    fn get_channel_index(&self, channel_id: AudioChannelID) -> Option<usize> {
        return self.channels.iter().position(|x| x.0 == channel_id);
    }
}