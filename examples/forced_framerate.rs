use std::time::Instant;

use nogine::{window::WindowCfg, input::Input};

fn main() {
    let mut window = WindowCfg::default().main(true).title("Forced Framerate").init().unwrap();
    
    let mut last_frame = Instant::now();
    while window.is_running() {
        let ts = last_frame.elapsed().as_secs_f32();
        last_frame = Instant::now();

        _ = window.handle_events();
        
        window.set_title(&format!("{} FPS", 1.0 / ts));
        
        Input::flush();
        window.force_framerate(last_frame, 60.0);
    }
}