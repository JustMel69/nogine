use nogine::{window::WindowCfg, unwrap_res};

fn main() {
    let mut window = unwrap_res!(WindowCfg::default().title("Hello World!").init());

    while window.is_running() {
        window.pre_tick(None);
        window.post_tick();
    }
}