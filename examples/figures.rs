use nogine::{graphics::Graphics, window::{WindowCfg, WindowMode}, color::{Color4, Color}, math::Vector2};

fn main() {
    // Create Window
    let mut window = WindowCfg::default().main(true).res((1280, 720)).title("Figures Example").mode(WindowMode::Windowed).init().expect("Couldn't open window");

    // Setup graphics
    Graphics::set_cam(Vector2::ZERO, 1.5);

    while window.is_running() {
        window.pre_tick(None);
        
        Graphics::draw_rect(Vector2(-1.55, -0.5), Vector2::ONE, Color4::CYAN);
        Graphics::draw_circle(Vector2::ZERO, 0.5, Color4::YELLOW);
        Graphics::draw_polygon(Vector2(1.0, 0.0), 0.5, 0.0, 6, Color::PINK);
        
        window.post_tick(Some(Color4::BLACK));
    }
}