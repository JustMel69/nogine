use nogine::{audio::{clip::AudioClip, Audio, AudioChannelID}, input::{Input, KeyInput}, unwrap_res, window::WindowCfg};


const METAL_BAR_AUDIO: &[u8] = include_bytes!("res/metal_bar.wav");
const MASTER_CHANNEL: AudioChannelID = 0;

fn main() {
    let mut window = unwrap_res!(WindowCfg::default().title("Audio Playing Example (Press E to play, P to pause, R to resume and S to stop)").init());
    
    Audio::create_channel(MASTER_CHANNEL, 1.0);

    let clip = unwrap_res!(AudioClip::new(std::io::Cursor::new(METAL_BAR_AUDIO)));

    window.set_vsync(true);

    let mut handle = None;
    while window.is_running() {
        window.pre_tick(None);

        if Input::key_pressed(KeyInput::E) {
            handle = Some(Audio::play(&clip, MASTER_CHANNEL, false, 1.0));
        }

        if let Some(handle) = &handle {
            if Input::key_pressed(KeyInput::P) {
                Audio::pause(&handle);
            }
    
            if Input::key_pressed(KeyInput::R) {
                Audio::resume(&handle);
            }
    
            if Input::key_pressed(KeyInput::S) {
                Audio::stop(&handle);
            }
        }

        window.post_tick();
    }
}