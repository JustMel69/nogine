use nogine::{window::WindowCfg, context::Context, input::Input};

fn main() {
    let mut ctx = Context::init();
    let mut window = WindowCfg::default().main(true).title("Hello World!").init(&mut ctx).unwrap();
    
    while window.is_running() {
        _ = window.handle_events(&mut ctx);
        Input::flush();
        window.swap_buffers()
    }
}