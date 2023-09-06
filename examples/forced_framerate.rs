use std::time::Instant;

use nogine::{window::WindowCfg, context::Context, input::Input};

fn main() {
    let mut ctx = Context::init();
    let mut window = WindowCfg::default().main(true).title("Forced Framerate").init(&mut ctx).unwrap();
    
    let mut last_frame = Instant::now();
    while window.is_running() {
        let ts = last_frame.elapsed().as_secs_f32();
        last_frame = Instant::now();
        
        _ = window.handle_events(&mut ctx);
        
        window.set_title(&format!("{} FPS", 1.0 / ts));
        
        Input::flush();
        ctx.force_framerate(last_frame, 60.0);
    }
}