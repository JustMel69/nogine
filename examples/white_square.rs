use nogine::{window::WindowCfg, input::Input, color::{Color4, Color}, graphics::Graphics, math::Vector2};

fn main() {
    let mut window = WindowCfg::default().main(true).title("Hello World!").init().unwrap();
    Graphics::set_cam(Vector2::ZERO, 0.0, Vector2(4.0, 2.25));

    while window.is_running() {
        _ = window.handle_events();
        window.clear_screen(Color4::BLACK);

        Graphics::draw_rect(-Vector2::ONE, Vector2::ONE * 2.0, Color4::WHITE);

        Input::flush();
        window.swap_buffers()
    }
}