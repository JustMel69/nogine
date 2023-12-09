use std::time::Instant;

use nogine::{audio::{Audio, clip::AudioClip}, window::WindowCfg, input::{Input, KeyInput}, unwrap_res};


const METAL_BAR_AUDIO: &[u8] = include_bytes!("res/metal_bar.wav");

fn main() {
    let mut window = unwrap_res!(WindowCfg::default().title("Stereo Audio Playing Example (Press E to play, P to pause, R to resume and S to stop, Arrows to pan)").init());
    
    let clip = unwrap_res!(AudioClip::new(std::io::Cursor::new(METAL_BAR_AUDIO)));

    let mut pan = 0.0;
    let mut last_frame = Instant::now();
    while window.is_running() {
        window.pre_tick(None);

        if Input::key_pressed(KeyInput::E) {
            Audio::play_ext(&clip, 1.0, false, Some(pan));
        }

        if Input::key_pressed(KeyInput::P) {
            Audio::pause(&clip);
        }

        if Input::key_pressed(KeyInput::R) {
            Audio::resume(&clip);
        }

        if Input::key_pressed(KeyInput::S) {
            Audio::stop(&clip);
        }

        pan = (pan + Input::axis(KeyInput::Left, KeyInput::Right) as f32).clamp(-1.0, 1.0);

        window.post_tick();
        window.force_framerate(last_frame, 60.0);
        last_frame = Instant::now();
    }
}