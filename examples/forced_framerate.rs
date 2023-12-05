use std::time::Instant;

use nogine::window::WindowCfg;

fn main() {
    // Create Window
    let mut window = WindowCfg::default().main(true).title("Forced Framerate Example").init().expect("Couldn't open window");
    
    let mut last_frame = Instant::now();
    while window.is_running() {
        window.pre_tick(None);
        
        // Get timestep
        let ts = last_frame.elapsed().as_secs_f32();
        last_frame = Instant::now();
        
        window.set_title(&format!("Forced Framerate Example | {} FPS", 1.0 / ts));
        
        // Handle window and force framerate
        window.post_tick(None);
        window.force_framerate(last_frame, 60.0);
    }
}