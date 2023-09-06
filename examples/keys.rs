use std::time::Instant;

use nogine::{window::WindowCfg, context::Context, input::{Input, KeyInput}};

fn main() {
    let mut ctx = Context::init();
    let mut window = WindowCfg::default().main(true).title("Press keys!").init(&mut ctx).unwrap();
    
    let mut press_duration = 0.0;
    let mut last_frame = Instant::now();
    while window.is_running() {
        let ts = last_frame.elapsed().as_secs_f32();
        last_frame = Instant::now();
        
        _ = window.handle_events(&mut ctx);

        if Input::key_pressed(KeyInput::E) {
            println!("E has been pressed");
            press_duration = 0.0;
        }

        if Input::key_released(KeyInput::E) {
            println!("E has been released. Key was pressed for {} seconds", press_duration);
        }

        if Input::key(KeyInput::E) {
            press_duration += ts;
        }

        Input::flush();
    }
}