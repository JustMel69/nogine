use nogine::{window::WindowCfg, unwrap_res};

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().title("Forced Framerate Example").init());
    
    //window.set_target_framerate(Some(60.0));
    window.set_vsync(true);

    while window.is_running() {
        window.pre_tick(None);
        
        // Get timestep
        let ts = window.ts();
        
        window.set_title(&format!("Forced Framerate Example | {} FPS", 1.0 / ts));
        
        // Handle window and force framerate
        window.post_tick();
    }
}