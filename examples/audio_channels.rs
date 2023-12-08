use std::time::Instant;

use nogine::{audio::{Audio, clip::AudioClip}, window::WindowCfg, input::{Input, KeyInput}, unwrap_res};

const BGM_AUDIO: &[u8] = include_bytes!("res/main_menu.wav");
const SFX_AUDIO: &[u8] = include_bytes!("res/metal_bar.wav");

fn main() {
    let mut window = unwrap_res!(WindowCfg::default().main(true).title("Audio Channels Example (W and S for BGM, Up and Down for SFX)").init());
    
    let bgm_clip = unwrap_res!(AudioClip::new(std::io::Cursor::new(BGM_AUDIO)));
    let sfx_clip = unwrap_res!(AudioClip::new(std::io::Cursor::new(SFX_AUDIO)));

    Audio::create_channel("bgm", 1.0);
    Audio::create_channel("sfx", 1.0);

    let mut bgm_volume = 1.0f32;
    let mut sfx_volume = 1.0f32;

    Audio::set_target("bgm");
    Audio::play(&bgm_clip, true, 1.0);

    Audio::set_target("sfx");
    Audio::play(&sfx_clip, true, 1.0);

    let mut last_frame = Instant::now();
    while window.is_running() {
        window.pre_tick(None);

        if Input::key_pressed(KeyInput::W) {
            bgm_volume = (bgm_volume + 0.25).clamp(0.0, 1.0);
            Audio::set_channel_volume("bgm", bgm_volume);
        }

        if Input::key_pressed(KeyInput::Up) {
            sfx_volume = (sfx_volume + 0.25).clamp(0.0, 1.0);
            Audio::set_channel_volume("sfx", sfx_volume);
        }


        if Input::key_pressed(KeyInput::S) {
            bgm_volume = (bgm_volume - 0.25).clamp(0.0, 1.0);
            Audio::set_channel_volume("bgm", bgm_volume);
        }

        if Input::key_pressed(KeyInput::Down) {
            sfx_volume = (sfx_volume - 0.25).clamp(0.0, 1.0);
            Audio::set_channel_volume("sfx", sfx_volume);
        }

        window.post_tick();
        window.force_framerate(last_frame, 60.0);
        last_frame = Instant::now();
    }
}