use nogine::{audio::{Audio, clip::AudioClip}, window::WindowCfg, input::{Input, KeyInput}, unwrap_res};


const METAL_BAR_AUDIO: &[u8] = include_bytes!("res/metal_bar.wav");

fn main() {
    let mut window = unwrap_res!(WindowCfg::default().title("Audio Playing Example (Press E to play, P to pause, R to resume and S to stop)").init());
    
    let clip = unwrap_res!(AudioClip::new(std::io::Cursor::new(METAL_BAR_AUDIO)));

    //window.set_target_framerate(Some(60.0));
    window.set_vsync(true);

    while window.is_running() {
        window.pre_tick(None);

        if Input::key_pressed(KeyInput::E) {
            Audio::play(&clip, false, 1.0);
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

        window.post_tick();
    }
}