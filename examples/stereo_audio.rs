use nogine::{audio::{clip::AudioClip, Audio, AudioChannelID}, input::{Input, KeyInput}, unwrap_res, window::WindowCfg};


const MAIN_MENU_AUDIO: &[u8] = include_bytes!("res/main_menu.wav");
const MASTER_CHANNEL: AudioChannelID = 0;

fn main() {
    let mut window = unwrap_res!(WindowCfg::default().title("Stereo Audio Playing Example (Arrows to pan)").init());
    
    Audio::create_channel(MASTER_CHANNEL, 1.0);
    
    let clip = unwrap_res!(AudioClip::new(std::io::Cursor::new(MAIN_MENU_AUDIO)));
    let audio = Audio::play_panned(&clip, MASTER_CHANNEL, true, 1.0, 0.0);
    
    //window.set_target_framerate(Some(60.0));
    window.set_vsync(true);
    
    let mut pan = 0.0;
    while window.is_running() {
        window.pre_tick(None);

        pan = (pan + Input::axis(KeyInput::Left, KeyInput::Right) as f32 * window.ts()).clamp(-0.5, 0.5);
        Audio::set_pan(&audio, pan);

        window.set_title(&format!("Stereo Audio Playing Example (Arrows to pan). PAN: {pan}"));
        window.post_tick();
    }
}