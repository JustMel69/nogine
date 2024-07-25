use nogine::{audio::{clip::AudioClip, Audio, AudioChannelID}, color::{Color, Color4}, graphics::Graphics, input::{Input, KeyInput}, math::{vec2, vec3}, unwrap_res, window::WindowCfg};


const MAIN_MENU_AUDIO: &[u8] = include_bytes!("res/main_menu.wav");
const MASTER_CHANNEL: AudioChannelID = 0;

fn main() {
    let mut window = unwrap_res!(WindowCfg::default().title("Spatial Audio Playing Example (Arrows to move source, Space and LShift to move vertically)").init());
    
    Audio::create_channel(MASTER_CHANNEL, 1.0);
    
    let clip = unwrap_res!(AudioClip::new(std::io::Cursor::new(MAIN_MENU_AUDIO)));
    window.set_vsync(true);
    
    let mut pos = vec3::FORW;
    let audio = Audio::play_at(&clip, MASTER_CHANNEL, true, 1.0, pos, vec2(1.0, 5.0));

    while window.is_running() {
        window.pre_tick(None);

        Graphics::set_cam(vec2::ZERO, vec2(window.aspect_ratio() * 5.0, 5.0));
        Graphics::draw_circle(vec2::ZERO, 0.25, Color4::WHITE);
        Graphics::draw_circle(vec2::ZERO, 5.0, Color4(1.0, 1.0, 1.0, 0.1));
        Graphics::draw_circle(vec2::ZERO, 1.0, Color4(1.0, 1.0, 1.0, 0.1));
        Graphics::draw_circle(pos.xz(), 0.125, Color4::RED);
        
        pos += vec3(
            Input::axis(KeyInput::Left, KeyInput::Right) as f32,
            Input::axis(KeyInput::LeftShift, KeyInput::Space) as f32,
            Input::axis(KeyInput::Down, KeyInput::Up) as f32,
        ) * window.ts() * 2.0;

        Audio::set_pos_and_distance(&audio, pos, vec2(1.0, 5.0));
        window.set_title(&format!("Spatial Audio Playing Example (Arrows to move source, Space and Shift to move vertically). Y: {}", pos.1));
        window.post_tick();
    }
}