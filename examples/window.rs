use nogine::window::WindowCfg;

fn main() {
    let mut window = WindowCfg::default().main(true).title("Hello World!").init().expect("Couldn't open window");

    while window.is_running() {
        window.pre_tick(None);
        window.post_tick(None);
    }
}