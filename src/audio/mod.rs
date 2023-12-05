use std::{io::Cursor, collections::HashMap};

use rodio::{OutputStream, Sink, OutputStreamHandle, Decoder, SpatialSink};
use uuid::Uuid;

use crate::{log_info, unwrap_res};

use self::clip::AudioClip;

pub mod clip;

// TODO: Make this shit less unsafe
static mut AUDIO: Option<Audio> = None;

enum AudioPlayer {
    Simple(Sink),
    Panned(SpatialSink),
}

impl AudioPlayer {
    fn pause(&self) {
        match self {
            AudioPlayer::Simple(x) => x.pause(),
            AudioPlayer::Panned(x) => x.pause(),
        }
    }

    fn stop(&self) {
        match self {
            AudioPlayer::Simple(x) => x.stop(),
            AudioPlayer::Panned(x) => x.stop(),
        }
    }

    fn play(&self) {
        match self {
            AudioPlayer::Simple(x) => x.play(),
            AudioPlayer::Panned(x) => x.play(),
        }
    }

    fn empty(&self) -> bool {
        match self {
            AudioPlayer::Simple(x) => x.empty(),
            AudioPlayer::Panned(x) => x.empty(),
        }
    }

    fn is_paused(&self) -> bool {
        match self {
            AudioPlayer::Simple(x) => x.is_paused(),
            AudioPlayer::Panned(x) => x.is_paused(),
        }
    }
}


/// Main struct for Audio playing.
pub struct Audio {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    clips: HashMap<Uuid, AudioPlayer>,
}

impl Audio {
    fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        Self { _stream, stream_handle, clips: HashMap::new() }
    }
    
    pub(crate) fn init() {
        unsafe { AUDIO = Some(Self::new()) };
        
        log_info!("Audio initialized.");
    }

    /// Plays an audio clip.
    pub fn play(clip: AudioClip, volume: f32) {
        Self::play_ext(clip, volume, None);
    }

    /// Plays an audio clip with a certain panning.
    pub fn play_ext(clip: AudioClip, volume: f32, panning: Option<f32>) {
        let audio = unsafe { &mut AUDIO.as_mut().unwrap() };

        let decoder = unwrap_res!(Decoder::new(Cursor::new(clip.data())));
        if let Some(panning) = panning {
            let sink = unwrap_res!(SpatialSink::try_new(&audio.stream_handle, [panning, 0.0, 0.0], [-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]));

            sink.append(decoder);
            sink.set_speed(volume);
            sink.play();
    
            audio.clips.insert(clip.uuid(), AudioPlayer::Panned(sink));
        } else {
            let sink = unwrap_res!(Sink::try_new(&audio.stream_handle));
    
            sink.append(decoder);
            sink.set_speed(volume);
            sink.play();
    
            audio.clips.insert(clip.uuid(), AudioPlayer::Simple(sink));
        }

    }

    /// Pauses a clip.
    pub fn pause(clip: &AudioClip) {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        if let Some(player) = audio.clips.get(&clip.uuid()) {
            player.pause();
        };
    }

    /// Resumes a clip.
    pub fn resume(clip: &AudioClip) {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        if let Some(player) = audio.clips.get(&clip.uuid()) {
            player.play();
        };
    }

    /// Stops a clip.
    pub fn stop(clip: &AudioClip) {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        if let Some(player) = audio.clips.get(&clip.uuid()) {
            player.stop();
        };
    }

    /// Checks if a clip is actually playing.
    pub fn is_fully_playing(clip: &AudioClip) -> bool {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        if let Some(player) = audio.clips.get(&clip.uuid()) {
            !player.empty() && !player.is_paused()
        } else {
            false
        }
    }

    /// Checks if a clip is playing or paused.
    pub fn is_playing_or_paused(clip: &AudioClip) -> bool {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        if let Some(player) = audio.clips.get(&clip.uuid()) {
            !player.empty()
        } else {
            false
        }
    }

    /// Checks if the clip is paused.
    pub fn is_paused(clip: &AudioClip) -> bool {
        let audio = unsafe { &AUDIO.as_ref().unwrap() };

        if let Some(player) = audio.clips.get(&clip.uuid()) {
            player.is_paused()
        } else {
            false
        }
    }
}