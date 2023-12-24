use rodio::{OutputStream, Sink, OutputStreamHandle, SpatialSink};

use crate::{log_info, log_error, crash, log_warn};

use self::{clip::AudioClip, channels::AudioChannel};

pub mod clip;
pub mod channels;

// TODO: Make this shit less unsafe
static mut AUDIO: Option<Audio> = None;

struct AudioPlayer {
    volume: f32,
    kind: AudioPlayerType,
}

impl AudioPlayer {
    fn new(volume: f32, kind: AudioPlayerType) -> Self {
        return Self { volume, kind };
    }

    fn set_volume(&mut self, volume: f32, inherited_volume: f32) {
        let volume = volume * inherited_volume;
        match &self.kind {
            AudioPlayerType::Simple(x) => x.set_volume(volume),
            AudioPlayerType::Panned(x) => x.set_volume(volume),
        }
    }

    fn set_volume_inherited(&mut self, inherited_volume: f32) {
        self.set_volume(self.volume, inherited_volume);
    }

    fn pause(&self) {
        match &self.kind {
            AudioPlayerType::Simple(x) => x.pause(),
            AudioPlayerType::Panned(x) => x.pause(),
        }
    }

    fn stop(&self) {
        match &self.kind {
            AudioPlayerType::Simple(x) => x.stop(),
            AudioPlayerType::Panned(x) => x.stop(),
        }
    }

    fn play(&self) {
        match &self.kind {
            AudioPlayerType::Simple(x) => x.play(),
            AudioPlayerType::Panned(x) => x.play(),
        }
    }

    fn empty(&self) -> bool {
        match &self.kind {
            AudioPlayerType::Simple(x) => x.empty(),
            AudioPlayerType::Panned(x) => x.empty(),
        }
    }

    fn is_paused(&self) -> bool {
        match &self.kind {
            AudioPlayerType::Simple(x) => x.is_paused(),
            AudioPlayerType::Panned(x) => x.is_paused(),
        }
    }
}

enum AudioPlayerType {
    Simple(Sink),
    Panned(SpatialSink),
}


/// Main struct for Audio playing.
pub struct Audio {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    channels: Vec<(&'static str, AudioChannel)>,
    target_channel: &'static str,
}

impl Audio {
    pub const DEFAULT_CHANNEL: &'static str = "default";
    
    fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self { _stream, stream_handle, channels: vec![(Self::DEFAULT_CHANNEL, AudioChannel::new(1.0))], target_channel: Self::DEFAULT_CHANNEL }
    }
    
    pub(crate) fn init() {
        unsafe { AUDIO = Some(Self::new()) };
        
        log_info!("Audio initialized.");
    }

    /// Plays an audio clip.
    pub fn play(clip: &AudioClip, loops: bool, volume: f32) {
        Self::play_ext(clip, volume, loops, None);
    }

    /// Plays an audio clip with a certain panning.
    pub fn play_ext(clip: &AudioClip, volume: f32, loops: bool, panning: Option<f32>) {
        let audio = unsafe { &mut AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index() {
            Some(x) => x,
            None => return,
        };

        audio.channels[index].1.play(clip, volume, loops, panning, &audio.stream_handle);
    }

    /// Pauses a clip.
    pub fn pause(clip: &AudioClip) {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        let index = match audio.get_channel_index() {
            Some(x) => x,
            None => return,
        };

        audio.channels[index].1.pause(clip);
    }

    /// Resumes a clip.
    pub fn resume(clip: &AudioClip) {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        let index = match audio.get_channel_index() {
            Some(x) => x,
            None => return,
        };

        audio.channels[index].1.resume(clip);
    }

    /// Stops a clip.
    pub fn stop(clip: &AudioClip) {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        let index = match audio.get_channel_index() {
            Some(x) => x,
            None => return,
        };

        audio.channels[index].1.stop(clip);
    }

    /// Checks if a clip is actually playing.
    pub fn is_fully_playing(clip: &AudioClip) -> bool {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        let index = match audio.get_channel_index() {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.is_fully_playing(clip);
    }

    /// Checks if a clip is playing or paused.
    pub fn is_playing_or_paused(clip: &AudioClip) -> bool {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        let index = match audio.get_channel_index() {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.is_playing_or_paused(clip);
    }

    /// Checks if the clip is paused.
    pub fn is_paused(clip: &AudioClip) -> bool {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        let index = match audio.get_channel_index() {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.is_paused(clip);
    }

    /// Sets the target channel
    pub fn set_target(channel: &'static str) {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        if audio.channels.iter().any(|x| x.0 == channel) {
            audio.target_channel = channel;
        } else {
            log_error!("\"{}\" is not a valid audio channel! All further audio calls will not go through, or may even crash.", channel);
            audio.target_channel = "";
        }
    }

    /// Sets the volume of the clip
    pub fn set_volume(clip: &AudioClip, volume: f32) {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index() {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.set_volume(clip, volume);
    }

    /// Returns the volume of the clip, or `None` if it's not playing
    pub fn get_volume(clip: &AudioClip) -> Option<f32> {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        let index = match audio.get_channel_index() {
            Some(x) => x,
            None => crash!("Invalid target audio channel."),
        };

        return audio.channels[index].1.get_volume(clip);
    }

    /// Sets the volume of a channel
    pub fn set_channel_volume(channel: &'static str, volume: f32) {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        if let Some(index) = audio.channels.iter().position(|x| x.0 == channel) {
            audio.channels[index].1.set_channel_volume(volume);
        } else {
            log_warn!("\"{}\" is not a valid audio channel!", channel);
        }
    }

    /// Returns the volume of a channel
    pub fn get_channel_volume(channel: &'static str) -> f32 {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        if let Some(index) = audio.channels.iter().position(|x| x.0 == channel) {
            return audio.channels[index].1.get_channel_volume();
        } else {
            crash!("\"{}\" is not a valid audio channel!", channel);
        }
    }

    /// Creates an audio channel
    pub fn create_channel(name: &'static str, volume: f32) {
        let audio = unsafe { AUDIO.as_mut().unwrap() };

        log_info!("Audio channel \"{name}\" created!");
        audio.channels.push((name, AudioChannel::new(volume)));
    }

    fn get_channel_index(&self) -> Option<usize> {
        return self.channels.iter().position(|x| x.0 == self.target_channel);
    }
}