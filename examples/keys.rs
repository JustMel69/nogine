use std::time::Instant;

use nogine::{window::WindowCfg, input::{Input, KeyInput}, unwrap_res};

fn main() {
    // Create Window
    let mut window = unwrap_res!(WindowCfg::default().main(true).title("Press keys!").init());
    
    let mut press_duration = 0.0;
    let mut last_frame = Instant::now();
    while window.is_running() {
        window.pre_tick(None);
        
        let ts = last_frame.elapsed().as_secs_f32();
        last_frame = Instant::now();

        // Detects only presses
        if Input::key_pressed(KeyInput::E) {
            println!("E has been pressed");
            press_duration = 0.0;
        }
        
        // Detects only releases
        if Input::key_released(KeyInput::E) {
            println!("E has been released. Key was pressed for {} seconds", press_duration);
        }
        
        // Detects holding the key
        if Input::key(KeyInput::E) {
            press_duration += ts;
        }
        
        window.post_tick(None);
    }
}