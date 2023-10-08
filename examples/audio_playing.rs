use std::{sync::mpsc, time::Instant};

use nogine::{audio::Audio, window::WindowCfg};

fn main() {
    let mut window = WindowCfg::default().main(true).title("Audio Playing Example").init().expect("Couldn't open window");
    
    let mut last_frame = Instant::now();
    while window.is_running() {
        window.handle_events();

        window.force_framerate(last_frame, 60.0);
        last_frame = Instant::now();
    }
}