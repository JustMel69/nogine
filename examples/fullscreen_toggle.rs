use nogine::{window::{WindowCfg, WindowMode}, unwrap_res, input::{Input, KeyInput}};

fn main() {
    let mut window = unwrap_res!(WindowCfg::default().title("Fullscreen toggle").init());

    let mut fullscreen = false;

    while window.is_running() {
        window.pre_tick(None);

        if Input::key(KeyInput::LeftAlt) && Input::key_pressed(KeyInput::Enter) {
            fullscreen = !fullscreen;
            window.set_window_mode(if fullscreen { WindowMode::Fullscreen } else { WindowMode::Windowed });
        }

        window.post_tick();
    }
}