use std::time::Instant;

use nogine::{window::WindowCfg, input::{Input, KeyInput}};

fn main() {
    // Create Window
    let mut window = WindowCfg::default().main(true).title("Press keys!").init().expect("Couldn't open window");
    
    let mut press_duration = 0.0;
    let mut last_frame = Instant::now();
    while window.is_running() {
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
        
        // Handle window (Flushing the input must be done BEFORE handling the events or the key presses and releases will not work properly)
        Input::flush();
        window.handle_events();
    }
}