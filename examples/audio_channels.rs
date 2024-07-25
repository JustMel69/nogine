use nogine::{audio::{clip::AudioClip, Audio, AudioChannelID}, input::{Input, KeyInput}, unwrap_res, window::WindowCfg};

const BGM_AUDIO: &[u8] = include_bytes!("res/main_menu.wav");
const SFX_AUDIO: &[u8] = include_bytes!("res/metal_bar.wav");

const BGM_CHANNEL: AudioChannelID = 0;
const SFX_CHANNEL: AudioChannelID = 1;

fn main() {
    let mut window = unwrap_res!(WindowCfg::default().title("Audio Channels Example (W and S for BGM, Up and Down for SFX)").init());
    
    let bgm_clip = unwrap_res!(AudioClip::new(std::io::Cursor::new(BGM_AUDIO)));
    let sfx_clip = unwrap_res!(AudioClip::new(std::io::Cursor::new(SFX_AUDIO)));

    Audio::create_channel(SFX_CHANNEL, 1.0);
    Audio::create_channel(BGM_CHANNEL, 1.0);

    let mut bgm_volume = 1.0f32;
    let mut sfx_volume = 1.0f32;

    Audio::play(&bgm_clip, BGM_CHANNEL, true, 1.0);
    Audio::play(&sfx_clip, SFX_CHANNEL, true, 1.0);

    //window.set_target_framerate(Some(60.0));
    window.set_vsync(true);
    
    while window.is_running() {
        window.pre_tick(None);
        
        if Input::key_pressed(KeyInput::W) {
            bgm_volume = (bgm_volume + 0.25).clamp(0.0, 1.0);
            Audio::set_channel_volume(BGM_CHANNEL, bgm_volume);
        }

        if Input::key_pressed(KeyInput::Up) {
            sfx_volume = (sfx_volume + 0.25).clamp(0.0, 1.0);
            Audio::set_channel_volume(SFX_CHANNEL, sfx_volume);
        }


        if Input::key_pressed(KeyInput::S) {
            bgm_volume = (bgm_volume - 0.25).clamp(0.0, 1.0);
            Audio::set_channel_volume(BGM_CHANNEL, bgm_volume);
        }

        if Input::key_pressed(KeyInput::Down) {
            sfx_volume = (sfx_volume - 0.25).clamp(0.0, 1.0);
            Audio::set_channel_volume(SFX_CHANNEL, sfx_volume);
        }

        window.post_tick();
    }
}