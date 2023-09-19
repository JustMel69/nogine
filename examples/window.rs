use nogine::{window::WindowCfg, input::Input};

fn main() {
    let mut window = WindowCfg::default().main(true).title("Hello World!").init().unwrap();
    
    while window.is_running() {
        _ = window.handle_events();
        Input::flush();
    }
}